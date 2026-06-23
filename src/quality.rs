
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

