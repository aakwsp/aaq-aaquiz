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
}
