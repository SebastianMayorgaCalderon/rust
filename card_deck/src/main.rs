use rand::{seq::SliceRandom, thread_rng};
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits: [&str; 4] = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks: [&str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];
        let mut cards: Vec<String> = vec![];
        for suit in suits {
            for rank in ranks {
                let card: String = format!("{} of {}", rank, suit);
                cards.push(card);
            }
        } 
    
        let deck: Deck = Deck { 
            cards,
        };
        return deck
    }
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self, num: usize) -> Vec<String> {
        let hand = self.cards.split_off(self.cards.len()-num);
        return hand
    }
}            
fn main() {
    let mut deck: Deck = Deck::new();
    deck.shuffle();
    println!("Heres your deck len: {:#?}", deck.cards.len());
    // add error handling if the deck is too small
    println!(
        "Heres your hand: {:#?}", 
        deck.deal(5)
    );
    println!("Heres your deck len: {:#?}", deck.cards.len());

}
