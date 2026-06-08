// Game module - contains all game-related structs and logic

mod card_collection;
mod deck;

// Re-export Deck so it can be used as game::Deck
pub use deck::Deck;
