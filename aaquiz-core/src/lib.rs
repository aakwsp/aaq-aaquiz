// -----------------------------------------------------------------------------
// --------------------------------------------------------------------- IMPORTS
// -----------------------------------------------------------------------------
mod card;
mod deck;
mod quality;
mod review;

pub use card::Card;
pub use deck::Deck;
pub use quality::calculate_quality;
pub use review::{ReviewRecord, ReviewState};

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

    fn sample_deck() -> Deck {
        let mut deck = Deck::new(String::from("sample"));
        deck.add_card(
            String::from("what is 2 + 2"),
            String::from("4"),
            vec![String::from("3"), String::from("5")],
            String::from("basic addition"),
            vec![],
        );
        deck
    }

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

    #[test]
    fn quality_from_correct_and_speed() {
        assert_eq!(calculate_quality(true, 1000, None), 5);
        assert_eq!(calculate_quality(true, 5000, None), 4);
        assert_eq!(calculate_quality(true, 8000, None), 3);
    }

    #[test]
    fn quality_from_wrong_answers() {
        assert_eq!(calculate_quality(false, 10000, Some(0)), 2);
        assert_eq!(calculate_quality(false, 10000, Some(2)), 0);
        assert_eq!(calculate_quality(false, 10000, Some(5)), 0);
        assert_eq!(calculate_quality(false, 10000, None), 1);
    }

    // NOTE: paths are &str for now. For cross-platform shipping (Windows/macOS),
    // switch to std::path::Path and use the `directories` crate for the correct
    // per-OS app-data location. Hardcoded path strings won't be portable.

    #[test]
    fn save_and_load_deck() {
        let deck = sample_deck();

        // save to temp file
        let path = "test_deck.json";
        deck.save(path).unwrap();
        let loaded = Deck::load(path).unwrap();

        // compare
        assert_eq!(deck, loaded);

        // clean file
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn review_card_and_update_deck() {
        let mut deck = sample_deck();
        assert!(deck.review_card(1, true, 1000, None));
        assert!(!deck.review_card(999, true, 1000, None));
        assert_eq!(deck.records[0].review_count, 1);
    }

    #[test]
    fn remove_card_from_deck() {
        let mut deck = sample_deck();

        deck.add_card(
            String::from("what is capital of france"),
            String::from("paris"),
            vec![String::from("nice"), String::from("lyon")],
            String::from("the capital is paris !"),
            vec![],
        );
        assert!(deck.remove_card(1));
        assert_eq!(deck.cards.len(), 1);
        assert_eq!(deck.records.len(), 1);
        assert_eq!(deck.cards[0].id, 2);

        assert!(!deck.remove_card(999)); // returns false
        assert_eq!(deck.cards.len(), 1); // nothing changed
    }

    #[test]
    fn edit_card_changes_content() {
        let mut deck = Deck::new(String::from("edit test"));

        deck.add_card(
            String::from("what is capital of france"),
            String::from("paris"),
            vec![String::from("nice"), String::from("lyon")],
            String::from("the capital is paris"),
            vec![],
        );

        deck.edit_card(
            1,
            String::from("what is the capital of japan"),
            String::from("tokyo"),
            vec![String::from("osaka"), String::from("kyoto")],
            String::from("the capital is tokyo"),
            vec![],
        );

        assert_eq!(deck.cards[0].question, "what is the capital of japan");
        assert_eq!(deck.cards[0].correct_answer, "tokyo");
        assert_eq!(deck.cards[0].id, 1);
    }
}
