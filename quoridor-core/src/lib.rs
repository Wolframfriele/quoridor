pub struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Coordinate {
        assert!(x < 9);
        assert!(y < 9);
        Coordinate { x, y }
    }
    
    pub fn get_x(&self) -> &u8 {
        &self.x
    }

    pub fn get_y(&self) -> &u8 {
        &self.y
    }
}

pub enum Rotation {
    Horizontal,
    Vertical,
}

/// A Wallcoordinate is set in the middle of the wall, each wall is either horizontal or vertical.
/// Meaning that each row and column has 8 posible positions. 
///
/// For a total of 8 x 8 x 2 = 128 different possible wall positions at the start off a game.
pub struct WallCoordinate {
    x: u8,
    y: u8,
    rotation: Rotation,
}

impl WallCoordinate {
    pub fn new(x: u8, y: u8, rotation: Rotation) -> WallCoordinate {
        assert!(x < 8);
        assert!(y < 8);
        WallCoordinate { x, y, rotation }
    }

    pub fn get_x(&self) -> &u8 {
        &self.x
    }

    pub fn get_y(&self) -> &u8 {
        &self.y
    }

    pub fn get_rotation(&self) -> &Rotation {
        &self.rotation
    }
}

pub enum Player {
    Player1,
    PLayer2,
}

pub struct Boardstate {
    player_1_position: Coordinate,
    player_2_position: Coordinate,
    player_1_available_walls: u8,
    player_2_available_walls: u8,
    // A problem with only storing an array with WallCoordinates is that there it's possible to
    // store walls that overlap, and are therefore an illegal state. It would be nice to find a way
    // for it to be impossible to store illegal state.
    wall_positions: [WallCoordinate; 20],
}

impl Boardstate {
    pub fn get_pawn_position(&self, player: Player) -> &Coordinate {
        match player {
            Player::Player1 => &self.player_1_position,
            Player::PLayer2 => &self.player_2_position,
        }
    }

    pub fn get_available_walls(&self, player: Player) -> &u8 {
        match player {
            Player::Player1 => &self.player_1_available_walls,
            Player::PLayer2 => &self.player_2_available_walls,
        }
    }

    pub fn get_wall_positions(&self) -> &[WallCoordinate; 20] {
        &self.wall_positions
    }

    pub fn get_legal_moves(&self, player: Player) -> PossibleMoves {
        let mut possible_moves = PossibleMoves::default();

        possible_moves.add_pawn_move(Coordinate::new(2, 2));
        possible_moves
    }

    pub fn execute_move(&self, player: Player) {
        
    }
}

#[derive(Default)]
pub struct PossibleMoves {
    pawn_moves: Vec<Coordinate>,
    wall_moves: Vec<WallCoordinate>,
}

impl PossibleMoves {
    fn add_pawn_move(&mut self, coordinate: Coordinate) {
        self.pawn_moves.push(coordinate);
    }

    fn add_wall_move(&mut self, coordinate: WallCoordinate) {
        self.wall_moves.push(coordinate);
    }

    pub fn get_pawn_moves(&self) -> &Vec<Coordinate> {
        &self.pawn_moves
    }

    pub fn get_wall_moves(&self) -> &Vec<WallCoordinate> {
        &self.wall_moves
    }
}

enum PlayedMove {
    PawnMove(Coordinate),
    WallMove(WallCoordinate),
}

enum VictoryReason {
    Crossover,
    OtherPlayerResigned,
    OtherPlayerOutOffTime,
    GameAbandoned,
}

enum GameStatus {
    InProgress,
    Finished {
        won_by: Player,
        reason: VictoryReason,
    },
}

pub struct Gamestate {
    id: String,
    active_player: Player,
    current_board_state: Boardstate,
    player_1_time_used: usize,
    player_2_time_used: usize,
    previous_moves: Vec<PlayedMove>,
    status: GameStatus,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         // keep it for later
//     }
// }
