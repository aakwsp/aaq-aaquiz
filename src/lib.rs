// -----------------------------------------------------------------------------
// --------------------------------------------------------------------- IMPORTS
// -----------------------------------------------------------------------------
use chrono::NaiveDate;

// -----------------------------------------------------------------------------
// ----------------------------------------------------- TYPES & IMPLEMENTATIONS
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub id: u64,
    pub question: String,
    pub correct_answer: String,
    pub distractors: Vec<String>,
    pub explanation: String,
    pub tags: Vec<String>,
}

impl Card {
    pub fn new(
        id: u64,
        question: String,
        correct_answer: String,
        distractors: Vec<String>,
        explanation: String,
        tags: Vec<String>,
    ) -> Card {
        Card {
            id,
            question,
            correct_answer,
            distractors,
            explanation,
            tags,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>,
    pub records: Vec<ReviewRecord>,
    next_id: u64,
}

impl Deck {
    pub fn new(name: String) -> Deck {
        Deck {
            name,
            cards: Vec::new(),
            records: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_card(
        &mut self,
        question: String,
        correct_answer: String,
        distractors: Vec<String>,
        explanation: String,
        tags: Vec<String>,
    ) {
        let id = self.next_id;

        let card: Card = Card::new(id, question, correct_answer, distractors, explanation, tags);
        let record = ReviewRecord::new(id);

        self.cards.push(card);
        self.records.push(record);

        self.next_id += 1;
    }

    pub fn cards_due(&self) -> Vec<&Card> {
        // todays date, this is how u use chrono
        let today = chrono::Local::now().date_naive();

        // date due + our return
        let mut due = Vec::new();

        // PERF: this becomes o(n^2) worst case, fine for small decks, swap to HashMap<u64, usize>
        // index if it grows larger.

        // just gets the record of all cards
        for record in &self.records {
            // - Some(Card) is if the find returns Some (a value exists), unwrap whatever is inside
            //  and name it card. thats what Some(card) is
            // - |c| are parameters for the inline function. it would be like what the are inside the
            //  () in func().
            // - how did we get that? find looks thru all of the cards and gives us a c which is
            //  valid to the return thing. "does card 1 = record.card_id" and goes thru all cards
            if record.date_due <= today
                && let Some(card) = self.cards.iter().find(|c| c.id == record.card_id)
            {
                due.push(card);
            }
        }
        // rust returns like this, no semicolon no return statement (unless fast return)
        due
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReviewState {
    New,
    Learning,
    Reviewing,
    Relearning,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReviewRecord {
    pub card_id: u64,
    pub state: ReviewState,
    pub interval_days: u32,  // how many days till we bring the card again
    pub ease_factor: f64,    // how easy it is (via algo later)
    pub review_count: u32,   // how many times we have reviewed card
    pub date_due: NaiveDate, // this is the date we added
}

impl ReviewRecord {
    pub fn new(card_id: u64) -> ReviewRecord {
        ReviewRecord {
            card_id,
            state: ReviewState::New,
            interval_days: 0,
            ease_factor: 2.5,
            review_count: 0,
            date_due: chrono::Local::now().date_naive(),
        }
    }

    // &mut self, we wanna change this so we are taking a mutable borrow
    // quality is 0-5 value
    pub fn update(&mut self, quality: u8) {
        if quality < 3 {
            // failing
            self.state = ReviewState::Relearning;
            self.interval_days = 0;
            self.review_count = 0;
        } else {
            // passing
            self.interval_days = match self.review_count {
                0 => 1,
                1 => 6,
                _ => (self.interval_days as f64 * self.ease_factor).round() as u32,
            };

            // ease factor calc from the sm-2
            let q = quality as f64;
            self.ease_factor += 0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02);

            // floor cant be below 1.3 for ease factor
            if self.ease_factor < 1.3 {
                self.ease_factor = 1.3;
            }

            self.review_count += 1;
            self.state = ReviewState::Reviewing;
        }

        // new dew date = today + time interval
        self.date_due =
            chrono::Local::now().date_naive() + chrono::Duration::days(self.interval_days as i64);
    }
}

// -----------------------------------------------------------------------------
// ------------------------------------------------------------------- FUNCTIONS
// -----------------------------------------------------------------------------

pub fn calculate_quality(correct: bool, response_ms: u32, distractor_rank: Option<u8>) -> u8 {
    if correct {
        if response_ms < 3000 {
            5
        } else if response_ms < 8000 {
            4
        } else {
            3
        }
    } else {
        // Wrong answer. If a distractor rank was provided, give partial credit
        // for a near-miss; otherwise fall back to a flat fail score.
        match distractor_rank {
            Some(rank) => 2u8.saturating_sub(rank),
            None => 1,
        }
    }
}

// -----------------------------------------------------------------------------
// ----------------------------------------------------------------------- TESTS
// -----------------------------------------------------------------------------

// cfg (config) is an atribute like the one above called derive. it just means that
// "only compile this while running tests", so when shipping the app this code
#[cfg(test)]
mod tests {
    // declares module test, its a namespace and its for organiz
    use super::*; // bring parantal into scope. and star just means all
    // use is something/somewhat like #include

    #[test] // tell cargo that this is a test, cargo run all tests
    fn creates_a_card() {
        let card = Card::new(
            //just calls c.tor
            1,
            String::from("what is worm backwards ?"),
            String::from("mrow"),
            vec![
                String::from("worm"),
                String::from("road"),
                String::from("morw"),
            ],
            String::from("three mrows!"),
            vec![],
        );

        // compares two values and throws an error
        assert_eq!(card.id, 1);
        assert_eq!(card.correct_answer, "mrow");
        assert_eq!(card.distractors.len(), 3); // see how many there are
    }

    #[test]
    fn new_record_starts_fresh() {
        let record = ReviewRecord::new(1);

        assert_eq!(record.card_id, 1);
        assert_eq!(record.state, ReviewState::New);
        assert_eq!(record.interval_days, 0);
        assert_eq!(record.ease_factor, 2.5);
        assert_eq!(record.review_count, 0);
    }

    #[test]
    fn passing_grows_the_interval() {
        let mut record = ReviewRecord::new(1);

        // first one interval should be 1 day
        record.update(5);
        assert_eq!(record.interval_days, 1);
        assert_eq!(record.review_count, 1);
        assert_eq!(record.state, ReviewState::Reviewing);

        // interval should be 6 days
        record.update(5);
        assert_eq!(record.interval_days, 6);
        assert_eq!(record.review_count, 2);

        record.update(5);
        assert!(record.interval_days > 6);
    }

    #[test]
    fn failing_resets_the_card() {
        let mut record = ReviewRecord::new(1);

        // progress up
        record.update(5);
        record.update(5);

        // fail should make all go to default
        record.update(1);
        assert_eq!(record.interval_days, 0);
        assert_eq!(record.review_count, 0);
        assert_eq!(record.state, ReviewState::Relearning);
    }

    #[test]
    fn adding_cards_assigns_ids_and_records() {
        let mut deck = Deck::new(String::from("maw 100"));

        deck.add_card(
            String::from("wa is maw"),
            String::from("maw"),
            vec![String::from("wat"), String::from("sick")],
            String::from("maw is maw"),
            vec![],
        );

        assert_eq!(deck.cards.len(), 1);
        assert_eq!(deck.records.len(), 1);
        assert_eq!(deck.cards[0].id, 1);
        assert_eq!(deck.records[0].card_id, 1);
    }

    #[test]
    fn cards_due_filters_out_future_cards() {
        let mut deck = Deck::new(String::from("hilo 101"));

        deck.add_card(
            String::from("hello are u here"),
            String::from("yes"),
            vec![String::from("no"), String::from("maybe")],
            String::from("you are here"),
            vec![],
        );

        deck.add_card(
            String::from("not you again D:"),
            String::from("yes"),
            vec![String::from("no"), String::from("maybe")],
            String::from("you are here"),
            vec![],
        );

        // make card 2 due 10 days into the future.
        deck.records[1].date_due = chrono::Local::now().date_naive() + chrono::Duration::days(10);

        // card 1 should be the only one due currently
        assert_eq!(deck.cards_due().len(), 1);
    }
}
