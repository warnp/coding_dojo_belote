use card::Card;

pub struct CardDeck {
    cards:Vec<Card>,

}

impl CardDeck {
    pub fn new() -> CardDeck{
        CardDeck{}
    }

    pub fn get_cards(&self) -> Vec<Card>{
        let mut cards = Vec::new();
        for i in 0..32 {
            cards.push(Card::new_with_pointers("jack", "toto"));
        }
        cards
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_some_cards(){
        let card_deck = CardDeck::new();

        let cards = card_deck.get_cards();
        assert!(cards.len()>0);
    }

    #[test]
    fn get_32_cards(){
        let card_deck = CardDeck::new();

        let cards = card_deck.get_cards();
        assert!(cards.len() == 32);
    }
}
