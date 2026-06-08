use rand::seq::SliceRandom;
use std::fmt;
use std::ops::Index;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<String>,
}

crate::derive_from_card_collection!(Deck);

impl Deck {
    /// Creates a new standard 52-card deck
    pub fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Spades", "Clubs"];
        let values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack",
            "Queen", "King",
        ];
        let cards: Vec<String> = suits
            .iter()
            .flat_map(|suit| {
                values
                    .iter()
                    .map(move |value| format!("{} of {}", value, suit))
            })
            .collect();

        Deck { cards }
    }

    /// Shuffles the deck randomly
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deals n cards from the deck
    ///
    /// Returns an error if there aren't enough cards
    pub fn deal(&mut self, n: u32) -> Result<Vec<String>, String> {
        let available = self.len();
        if n > available.try_into().unwrap() {
            return Err(format!(
                "Cannot deal {} cards, deck only has {} cards left",
                n, available,
            ));
        }

        let mut dealt = Vec::new();
        for _ in 0..n {
            if let Some(card) = self.cards.pop() {
                dealt.push(card);
            } else {
                break;
            }
        }
        Ok(dealt)
    }
}

// Made with Bob
