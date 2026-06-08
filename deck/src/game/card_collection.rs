#[macro_export]
macro_rules! derive_from_card_collection {
    ($type:ty) => {
        impl $type {
            /// Returns the number of cards remaining in the deck
            pub fn len(&self) -> usize {
                self.cards.len()
            }

            /// Returns true if the deck has no cards
            pub fn is_empty(&self) -> bool {
                self.cards.is_empty()
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, " -{}", self.cards.join("\n -"))
            }
        }

        impl Index<usize> for $type {
            type Output = String;
            fn index(&self, index: usize) -> &Self::Output {
                &self.cards[index]
            }
        }

        impl IntoIterator for $type {
            type Item = String;
            type IntoIter = std::vec::IntoIter<Self::Item>;
            fn into_iter(self) -> Self::IntoIter {
                self.cards.into_iter()
            }
        }
    };
}
