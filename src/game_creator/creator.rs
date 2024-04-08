mod creator{
    use std::collections::HashSet;
    use crate::gameModel::card::{Card, CardBuilder};
    use crate::gameModel::game::{Game, GameBuilder};

    fn create_game(items: HashSet<i16>) -> Game {
        let mut cards : Vec<Card> = Vec::new();

        for combinations in items.into_iter().combination(2){
            let card = CardBuilder::default()
                .with_items(combinations)
                .build();
            cards.push(card)
        }

        return GameBuilder::new()
            .with_cards(cards)
            .build()
        ;
    }

    fn are_cards_compatibles(card1: &Card, card2: &Card) -> bool {
        card1.items.iter().cloned().collect() - card2.items.iter().cloned().collect() == 1
    }

    #[cfg(test)]
    mod tests {
        use crate::game_creator::creator::creator::create_game;
        use crate::gameModel::game::Game;

        #[test]
        fn test_create_game_2items(){
            let game: Game = create_game(vec![1,2]);
            assert_eq!(game.cards.len(), 0);
        }
    }
}