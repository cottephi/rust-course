// Import the game module
mod game;

// Use Deck from the game module
use game::Deck;

fn main() {
    // Arrays : fixed size, more performant (very slightly). Most importantly,
    // it indicates to other devs that it is constant in size.

    // Vector : can grow and shrink
    let mut deck = Deck::new();
    let n_players = 2;
    let mut deal_size = 3;

    deck.shuffle();

    println!("Shuffled deck: {}", deck);
    println!("Dealing...");

    for i in 0..n_players {
        while !deck.is_empty() {
            match deck.deal(deal_size) {
                Ok(cards) => {
                    println!("Dealt: {:#?} cards to player {}", cards, i);
                    println!("{} cards remaining", deck.len());
                }
                Err(_e) => {
                    deal_size -= 1;
                    println!(
                        "Not enough cards left in deck. Reducing deal size to {}",
                        deal_size
                    );
                }
            }
        }
    }
}

// Made with Bob
