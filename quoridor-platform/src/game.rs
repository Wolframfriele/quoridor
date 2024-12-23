use uuid::Uuid;

use quoridor_core::gamestate::{Gamestate, TimeControl};

use crate::player::AnonUser;

pub struct Game {
    id: Uuid,
    white_player: AnonUser,
    black_player: AnonUser,
    gamestate: Gamestate,
}

impl Game {
    pub fn new(player_1: AnonUser, player_2: AnonUser, time_control: TimeControl) -> Self {
        Game {
            id: Uuid::new_v4(),
            white_player: player_1,
            black_player: player_2,
            gamestate: Gamestate::new(time_control),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
