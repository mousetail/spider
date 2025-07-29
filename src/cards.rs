enum Suit {
    Clubs, Hearts, Diamonds, Spades
}

enum CardColor {
    Red,
    Black
}

impl Suit {
    fn get_color(&self) -> CardColor {
        match self {
            Suit::Clubs => CardColor::Black,
            Suit::Hearts => CardColor::Red,
            Suit::Diamonds => CardColor::Red,
            Suit::Spades => CardColor::Black
        }
    }
}

struct Card {
    suit: Suit,
    rank: u8,
}

struct GameState {
    stacks: [Vec<Card>; 10],
    deck: Vec<Card>
}

impl GameState {
    pub fn init() {

    }
}