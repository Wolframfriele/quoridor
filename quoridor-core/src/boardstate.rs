#[derive(Clone)]
pub struct Boardstate {
    white_position: Coordinate,
    black_position: Coordinate,
    white_available_walls: u8,
    black_available_walls: u8,
    wall_positions: [Option<WallOrientation>; 71],
    horizontal_edges: [bool; 72],
    vertical_edges: [bool; 80],
}

impl Boardstate {
    pub fn get_pawn_position(&self, player: Player) -> &Coordinate {
        match player {
            Player::White => &self.white_position,
            Player::Black => &self.black_position,
        }
    }

    pub fn get_available_walls(&self, player: Player) -> &u8 {
        match player {
            Player::White => &self.white_available_walls,
            Player::Black => &self.black_available_walls,
        }
    }

    pub fn get_wall_positions(&self) -> &[Option<WallOrientation>; 71] {
        &self.wall_positions
    }

    pub fn get_legal_moves(&self, player: Player) -> PossibleActions {
        let mut possible_moves = PossibleActions::default();

        possible_moves.add_pawn_action(Coordinate::new(2));
        possible_moves
    }

    pub fn play_action(&self, player: Player, action: Action) {}
}

pub enum Player {
    White,
    Black,
}

#[derive(Clone)]
pub struct Coordinate {
    node: u8,
}

impl Coordinate {
    pub fn new(node: u8) -> Coordinate {
        assert!(node >= 1 && node <= 81);
        Coordinate { node }
    }

    pub fn from_notation(coordinate_notation: &str) -> Coordinate {
        // implement
    }
}

#[derive(Clone)]
enum WallOrientation {
    Horizontal,
    Vertical,
}

pub struct WallCoordinate {
    // Might need to make separate Coordinate struct for wall, to lower the maximum number 71
    // instead of 81
    coordinate: Coordinate,
    orientation: WallOrientation,
}

pub enum Action {
    Pawn(Coordinate),
    Wall(WallCoordinate),
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Default)]
pub struct PossibleActions {
    pawn_actions: Vec<Coordinate>,
    wall_actions: Vec<WallCoordinate>,
}

impl PossibleActions {
    fn add_pawn_action(&mut self, coordinate: Coordinate) {
        self.pawn_actions.push(coordinate);
    }

    fn add_wall_action(&mut self, wall_move: WallCoordinate) {
        self.wall_actions.push(wall_move);
    }

    pub fn get_pawn_actions(&self) -> &Vec<Coordinate> {
        &self.pawn_actions
    }

    pub fn get_wall_actions(&self) -> &Vec<WallCoordinate> {
        &self.wall_actions
    }
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
