pub struct Card{
    figure: String,
    color: String,
}

impl Card {
    pub fn new(figure: String, color: String) -> Card {
        Card{
            figure: figure,
            color: color,
        }
    }

    pub fn new_with_pointers(figure: &str, color: &str) -> Card {
        Card::new(figure.to_string(), color.to_string())
    }

    pub fn get_figure(&self) -> String {
        self.figure.clone()
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn retrieve_card_info(){
        let card = Card::new("jack".to_string(),"heart".to_string());

        assert_eq!(card.get_figure(), "jack".to_string());
        assert_eq!(card.get_color(), "heart".to_string());
    }


    #[test]
    fn retrieve_card_info_with_pointer(){
        let card = Card::new_with_pointers("jack","heart");

        assert_eq!(card.get_figure(), "jack".to_string());
        assert_eq!(card.get_color(), "heart".to_string());
    }
}
