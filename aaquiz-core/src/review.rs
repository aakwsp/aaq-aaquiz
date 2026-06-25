// -----------------------------------------------------------------------------
// --------------------------------------------------------------------- IMPORTS
// -----------------------------------------------------------------------------
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// ----------------------------------------------------- TYPES & IMPLEMENTATIONS
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ReviewState {
    New,
    Learning,
    Reviewing,
    Relearning,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
