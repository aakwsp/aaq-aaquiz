// -----------------------------------------------------------------------------
// ------------------------------------------------------------------------ TYPE
// -----------------------------------------------------------------------------

// derive just means run these types of functions on it, run Debug function and
// Clone function
#[derive(Debug, Clone)]
pub struct Card {
    pub id: u64,
    pub question: String,
    pub correct_answer: String,
    pub distractors: Vec<String>,
    pub explaination: String,
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
    pub interval_days: u32, // how many days till we bring the card again
    pub ease_factor: f64,   // how easy it is (via algo later)
    pub review_count: u32,  // how many times we have reviewed card
}
// -----------------------------------------------------------------------------
// ------------------------------------------------------------------------ IMPL
// -----------------------------------------------------------------------------

// impl is what functions are in the type
impl Card {
    pub fn new(
        // this is us saying the constructor
        id: u64,
        question: String,
        correct_answer: String,
        distractors: Vec<String>,
        explaination: String,
    ) -> Card {
        // -> Card is the return type which is card
        Card {
            // this just says what goes where in the card
            id,
            question,
            correct_answer,
            distractors,
            explaination,
        }
    }
}

impl ReviewRecord {
    pub fn new(card_id: u64) -> ReviewRecord {
        ReviewRecord {
            card_id,
            state: ReviewState::New,
            interval_days: 0,
            ease_factor: 2.5,
            review_count: 0,
        }
    }

    // &mut self, we wanna change this so we are taking a mutable borrow
    // quality is 1-5 value
    pub fn update(&mut self, quality: u8){
        // failing score: 3
        // aakwsp did not recall card 
        if quality < 3 {
            self.state = ReviewState::Relearning;
            self.interval_days = 0;
            self.review_count = 0;
            return; // early ret are allowed
        }

        // passing score:
        // aakwsp is doing well
        self.interval_days = match self.review_count{
            0 => 1,
            1 => 6,
            _ => (self.interval_days as f64 * self.ease_factor).round() as u32
        };

        // adjust ease factor using the SM-2 formula cuz claude said it was good
        let q = quality as f64; // "as f64" is a cast, pre cool acc
        self.ease_factor += 0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02);

        // dont let ease factor go below the SM-2 floor, 1.3
        if self.ease_factor < 1.3{
            self.ease_factor = 1.3;
        }

        self.review_count += 1;
        self.state = ReviewState::Reviewing;
    }
}

// -----------------------------------------------------------------------------
// ------------------------------------------------------------------------ TEST
// -----------------------------------------------------------------------------

// cfg is an atribute like the one above called derive. it just means that
// "only compile this while running tests", so when shipping the app this code
// aint finna compile. also cfg means config
#[cfg(test)]
mod tests {
    // declares module test, its a namespace shi and its for organiz
    use super::*; // bring parantal shi into scope. and star just means all
    // use is something/somewhat like #include

    #[test] // tell cargo that this is a test, cargo run all tests
    fn creates_a_card() {
        let card = Card::new(
            //just calls c.tor
            1,
            String::from("what is worm backwards ?"),
            String::from("mrow"),
            vec![
                String::from("maw"),
                String::from("mro"),
                String::from("meow"),
            ],
            String::from("three maws !"),
        );

        // i know u a dumbass this is just compareing the vals @fatkwsp
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

        // lefail should make all go to default
        record.update(1);
        assert_eq!(record.interval_days, 0);
        assert_eq!(record.review_count, 0);
        assert_eq!(record.state, ReviewState::Relearning);
    }
}
