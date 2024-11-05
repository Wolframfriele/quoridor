#![allow(dead_code)]
#![allow(unused_variables)]

use fixedbitset::FixedBitSet;

#[derive(Clone, Hash)]
/// The boardstate is responsible for keeping track of all the pawns and walls placed on the board.
///
/// It is also the place that holds the basic game rules related logic, since it is the place that
/// can check if move's keep the board in a legal state. This will require some implementation off
/// a path finding algorithm, since so much off the game revolves around not blocking the path off
/// for the pawns.
pub struct Boardstate {
    active_player: Player,
    white_position: Location,
    black_position: Location,
    white_available_walls: u8,
    black_available_walls: u8,
    wall_positions: [Option<WallOrientation>; 71],
    horizontal_edges: FixedBitSet,
    vertical_edges: FixedBitSet,
}

impl Boardstate {
    pub fn new() -> Boardstate {
        Boardstate {
            active_player: Player::White,
            white_position: Location::build(5)
                .expect("White player starting location on square 5 should be a valid location."),
            black_position: Location::build(77)
                .expect("Black player starting location on square 77 should be a valid location."),
            white_available_walls: 10,
            black_available_walls: 10,
            wall_positions: [const { None }; 71],
            horizontal_edges: FixedBitSet::with_capacity(72),
            vertical_edges: FixedBitSet::with_capacity(80),
        }
    }

    // problably will need some sort off method to start the board off in a certain position,
    // either from notation or from some other thing.

    pub fn get_position_white_pawn(&self) -> &Location {
        &self.white_position
    }

    pub fn get_position_black_pawn(&self) -> &Location {
        &self.black_position
    }

    pub fn get_available_walls_white_player(&self) -> &u8 {
        &self.white_available_walls
    }

    pub fn get_available_walls_black_player(&self) -> &u8 {
        &self.black_available_walls
    }

    pub fn get_wall_positions(&self) -> &[Option<WallOrientation>; 71] {
        &self.wall_positions
    }

    /// Method to get all the legal moves for the currently active player in the
    /// current boadstate.
    ///
    /// For the pawn moves this means checking all the move directions and the weird
    /// rules around hoping over other players (need to figure out clean way to do that)
    /// for the wall moves it needs to check all the open wall positions, see what direction would
    /// be available (not blocked by existing wall) and check if it doesn't block all paths to the
    /// opposite side for either player.
    pub fn get_legal_moves(&self) -> PossibleActions {
        let mut possible_moves = PossibleActions::default();

        possible_moves.add_pawn_action(Location::build(2).unwrap());
        possible_moves
    }

    /// The play action takes an action as input and attempts to play that move on the current
    /// board, if the action is illegal an error is returned. When the action is legal, the
    /// boardstate is updated.
    /// A check is executed if the game is won. When the game is won an enum with the Won result is
    /// returned, otherwise the active player is swapped and an InProgress enum is returned
    pub fn play_action(&self, action: Action) -> Result<(), String> {
        Ok(())
    }

    /// I might not need this method depending on what I need for MCTS, if I can make it work with
    /// just returning a random legal move on a new boardstate that will work. But I might need
    /// some heuristic for move selection based on all the possible legal moves. So will need to
    /// see at a later point if I can replace this with a return random move, I just prefer to keep
    /// the heuristics out of boardstate (Maybe I can Inject the logic for selecting a next move?).
    pub fn play_action_on_clone(&self, action: Action) -> Result<Boardstate, String> {
        Ok(Boardstate::new())
    }

    /// When playing against the computer a player should be able to take back moves, also usefull
    /// for stepping through a game history. Keeping the history of the game is the responsability
    /// of the gamestate.
    pub fn undo_action(&self, action: Action) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Clone, Hash)]
/// An enum with the two players
pub enum Player {
    White,
    Black,
}

#[derive(Clone, Hash)]
pub struct Location {
    square: u8,
}

impl Location {
    /// Create a new Location, the number is the square on the board where counting starts in the
    /// bottom left corner and goes right. So coordinate A1 is square 1, B1 is square 2 and A2 is
    /// square 10 etc.
    ///
    /// The square number must be between 1 and 81, or else an error will be returned.
    pub fn build(square: u8) -> Result<Self, String> {
        if (1..=81).contains(&square) {
            Ok(Location { square })
        } else {
            Err(format!(
                "The square should be in range 1..=81, but was: {square}"
            ))
        }
    }

    pub fn from_notation(coordinate_notation: &str) -> Result<Self, String> {
        // Convert quoridor notation string to coordinate; for example C2 is 12

        Ok(Location { square: 12 })
    }
}

#[derive(Clone, Hash)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

pub struct WallLocation {
    square: u8,
    orientation: WallOrientation,
}

impl WallLocation {
    /// Create a new WallCoordinate. There are 64 unique positions that a wall can be placed in,
    /// but the node value can be between 1 and 71. This is because the coordinate of the squares
    /// is
    pub fn build(square: u8, orientation: WallOrientation) -> Result<Self, String> {
        if (1..=71).contains(&square) && &square % 9 != 0 {
            Ok(WallLocation {
                square,
                orientation,
            })
        } else {
            Err(format!("The square should be in the range 1..=71 and should not be divisible by 9, but was {square}"))
        }
    }
}

pub enum Action {
    Pawn(Location),
    Wall(WallLocation),
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Default)]
pub struct PossibleActions {
    pawn_actions: Vec<Location>,
    wall_actions: Vec<WallLocation>,
}

impl PossibleActions {
    fn add_pawn_action(&mut self, coordinate: Location) {
        self.pawn_actions.push(coordinate);
    }

    fn add_wall_action(&mut self, wall_move: WallLocation) {
        self.wall_actions.push(wall_move);
    }

    pub fn get_pawn_actions(&self) -> &Vec<Location> {
        &self.pawn_actions
    }

    pub fn get_wall_actions(&self) -> &Vec<WallLocation> {
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
