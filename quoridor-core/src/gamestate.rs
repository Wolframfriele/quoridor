#![allow(dead_code)]

use anyhow::{Ok, Result};
use std::time::{Duration, Instant};

use crate::actions::{Action, ExecutedAction};
use crate::boardstate::{Boardstate, Player};

pub struct Gamestate {
    board_state: Boardstate,
    start_time: Instant,
    time_control: TimeControl,
    white_time_used: Duration,
    black_time_used: Duration,
    moves: Vec<ExecutedAction>,
    status: GameStatus,
}

impl Gamestate {
    pub fn new(time_control: TimeControl) -> Self {
        Gamestate {
            board_state: Boardstate::new(),
            start_time: Instant::now(),
            time_control,
            white_time_used: Duration::new(0, 0),
            black_time_used: Duration::new(0, 0),
            moves: Vec::new(),
            status: GameStatus::InProgress,
        }
    }

    pub fn get_boardstate(&self) -> &Boardstate {
        &self.board_state
    }

    pub fn execute_action(&mut self, action: Action) -> Result<GameStatus> {
        let time = Instant::now();
        let status = self.board_state.apply_action(action)?;

        self.moves.push(ExecutedAction::new(time, action));
        self.status = status;

        Ok(self.status)
    }
}

#[derive(Debug)]
pub enum TimeControl {
    Timed { seconds: usize, increment: usize },
    Correspondence,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameStatus {
    InProgress,
    Finished {
        won_by: Player,
        reason: VictoryReason,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VictoryReason {
    ReachedOppositeSide,
    Resigned,
    OutOffTime,
    Abandoned,
}
