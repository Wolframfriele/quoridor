#![allow(dead_code)]

use crate::actions::Action;
use crate::boardstate::{Boardstate, Player};

pub struct Gamestate {
    time_control: TimeControl,
    board_state: Boardstate,
    white_time_used: usize,
    black_time_used: usize,
    moves: Vec<Action>,
    status: GameStatus,
}

impl Gamestate {
    pub fn new(time_control: TimeControl) -> Self {
        Gamestate {
            time_control,
            board_state: Boardstate::new(),
            white_time_used: 0,
            black_time_used: 0,
            moves: Vec::new(),
            status: GameStatus::InProgress,
        }
    }
}

#[derive(Debug)]
pub enum TimeControl {
    Timed { seconds: usize, increment: usize },
    Unlimited,
    Correspondence,
}

#[derive(Debug)]
pub enum GameStatus {
    InProgress,
    Finished {
        won_by: Player,
        reason: VictoryReason,
    },
}

#[derive(Debug)]
pub enum VictoryReason {
    ReachedOppositeSide,
    Resigned,
    OutOffTime,
    Abandoned,
}
