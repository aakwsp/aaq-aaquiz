// derive just means run these types of functions on it, run Debug function and 
// Clone function
#[derive(Debug, Clone)]
pub struct Card{
    pub id: u64,
    pub question: String,
    pub correct_answer: String,
    pub distractors: Vec<String>,
    pub explaination: String
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReviewState{
    New,
    Learning, 
    Reviewing, 
    Relearning,
}

// impl is what functions are in the type
impl Card{
    pub fn new( // this is us saying the constructor
        id: u64,
        question: String,
        correct_answer: String, 
        distractors: Vec<String>,
        explaination: String,
    ) -> Card {  // -> Card is the return type which is card
        Card { // this just says what goes where in the card
            id, 
            question, 
            correct_answer,
            distractors, 
            explaination
        }
    }
}

// cfg is an atribute like the one above called derive. it just means that 
// "only compile this while running tests", so when shipping the app this code 
// aint finna compile. also cfg means config
#[cfg(test)]
mod tests { // declares module test, its a namespace shi and its for organiz 
    use super::*; // bring parantal shi into scope. and star just means all
                  // use is something/somewhat like #include

    #[test] // tell cargo that this is a test, cargo run all tests
    fn creates_a_card(){
        let card = Card::new( //just calls c.tor
            1,
            String::from("how many letters in maw"),
            String::from("three"),
            vec![
                String::from("one"),
                String::from("two"),
                String::from("four"),
            ],
            String::from("three maws !")
        );
        
        // i know u a dumbass this is just compareing the vals @fatkwsp
        assert_eq!(card.id, 1);
        assert_eq!(card.correct_answer, "three");
        assert_eq!(card.distractors.len(), 3); // see how many there are
    }
}