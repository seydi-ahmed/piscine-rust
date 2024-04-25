
extern crate rand;
use rand::Rng;
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Suit {
    Heart,
    Diamonds,
    Spade,
    Club,
}

impl Suit {
    pub fn random() -> Suit {
        let nombre_de_suits : u8 = 4_u8;
        let hasard = rand::thread_rng().gen_range(1_u8, nombre_de_suits + 1);
        
        Suit::translate(hasard)
    }
    
    pub fn translate(valeur : u8) -> Suit{
        match valeur {
            1 => Suit::Heart,
            2 => Suit::Diamonds,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => Suit::random(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
}


impl Rank {
    pub fn random() -> Rank {
        let nombre_de_rank : u8 = 4_u8;
        let hasard = rand::thread_rng().gen_range(1_u8, nombre_de_rank + 1);

        Rank::translate(hasard)
    }

    pub fn translate(valeur : u8) -> Rank{
        match valeur {
            1 => Rank::Ace,
            2 => Rank::King,
            3 => Rank::Queen,
            4 => Rank::Jack,
            _ => Rank::random(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Card {
    pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
