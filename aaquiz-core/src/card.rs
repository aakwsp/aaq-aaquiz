// -----------------------------------------------------------------------------
// --------------------------------------------------------------------- IMPORTS
// -----------------------------------------------------------------------------
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// ----------------------------------------------------- TYPES & IMPLEMENTATIONS
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
