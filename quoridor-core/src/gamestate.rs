use std::time;

use crate::boardstate::{Player, Boardstate, Action};

pub struct Gamestate {
    id: String,
    active_player: Player,
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
