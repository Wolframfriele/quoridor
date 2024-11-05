use std::time;

use crate::boardstate::{Action, Boardstate, Player};

pub struct Gamestate {
    id: String,
    current_board_state: Boardstate,
    white_time_used: time::Duration,
    black_time_used: time::Duration,
    previous_moves: Vec<Action>,
    status: GameStatus,
}

enum GameStatus {
    InProgress,
    Finished {
        won_by: Player,
        reason: VictoryReason,
    },
}

enum VictoryReason {
    ReachedOppositeSide,
    Resigned,
    OutOffTime,
    Abandoned,
}
