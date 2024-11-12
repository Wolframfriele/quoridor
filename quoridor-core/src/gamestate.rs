#![allow(dead_code)]

use uuid::Uuid;

use crate::actions::Action;
use crate::boardstate::{Boardstate, Player};

pub struct Gamestate {
    id: Uuid,
    game_type: GameType,
    time_control: TimeControl,
    board_state: Boardstate,
    white_time_used: usize,
    black_time_used: usize,
    moves: Vec<Action>,
    status: GameStatus,
}

impl Gamestate {
    pub fn new(game_type: GameType, time_control: TimeControl) -> Self {
        Gamestate {
            id: Uuid::new_v4(),
            game_type,
            time_control,
            board_state: Boardstate::new(),
            white_time_used: 0,
            black_time_used: 0,
            moves: Vec::new(),
            status: GameStatus::InProgress,
        }
    }
}

pub enum GameType {
    PersonVsPerson,
    PersonVsPersonSameDevice,
    PersonVsComputer,
    ComputerVsComputer,
}

pub enum TimeControl {
    Timed { seconds: usize, increment: usize },
    Unlimited,
    Correspondence,
}

pub enum GameStatus {
    InProgress,
    Finished {
        won_by: Player,
        reason: VictoryReason,
    },
}

pub enum VictoryReason {
    ReachedOppositeSide,
    Resigned,
    OutOffTime,
    Abandoned,
}
