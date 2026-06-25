// -----------------------------------------------------------------------------
// --------------------------------------------------------------------- IMPORTS
// -----------------------------------------------------------------------------
use crate::card::Card;
use crate::review::ReviewRecord;

use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// ----------------------------------------------------- TYPES & IMPLEMENTATIONS
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

    // NOTE: the dashboard stats need review history, add a reviewEvent log
    // for dates, correct quality, response time, etc to append on every review.
    // derive stats from it later.

    // the box<dyn std::error::Error> just says error of any type
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    // TODO open
    //

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
