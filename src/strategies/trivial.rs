use super::{NextMoveData, PassCardsData, Strategy, StrategyInitData};

pub struct TrivialStrategy;

impl Strategy for TrivialStrategy {
    fn init(&mut self, _data: StrategyInitData) {}

    fn pass_cards(&mut self, data: PassCardsData) -> Vec<usize> {
        (0..data.nb_to_choose).collect()
    }

    fn next_move(&mut self, data: NextMoveData) -> usize {
        data.allowed_range.start
    }
}
