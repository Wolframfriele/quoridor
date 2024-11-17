use bitmaps::Bitmap;

use crate::actions::{Action, PossibleActions};
use crate::gamestate::{GameStatus, VictoryReason};
use crate::locations::{
    Coordinate, Direction, Location, PawnLocation, WallLocation, WallOrientation,
};

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Clone, Hash, Debug)]
/// The boardstate is responsible for keeping track of all the pawns and walls placed on the board.
///
/// It is also the place that holds the basic game rules related logic, since it is the place that
/// can check if move's keep the board in a legal stat. This will require some implementation off
/// a path finding algorithm, since so much off the game revolves around not blocking the path off
/// for the pawns.
///
/// Still need to figure out the rules around jumping over pawns and how to check that efficiently
pub struct Boardstate {
    active_player: Player,
    white_position: PawnLocation,
    black_position: PawnLocation,
    white_available_walls: u8,
    black_available_walls: u8,
    wall_placed: Bitmap<71>,
    wall_orientation: Bitmap<71>,
}

impl Default for Boardstate {
    fn default() -> Boardstate {
        Boardstate {
            active_player: Player::White,
            white_position: PawnLocation::build(4)
                .expect("White player starting location on square 5 should be a valid location."),
            black_position: PawnLocation::build(76)
                .expect("Black player starting location on square 77 should be a valid location."),
            white_available_walls: 10,
            black_available_walls: 10,
            wall_placed: Bitmap::<71>::new(),
            wall_orientation: Bitmap::<71>::new(),
        }
    }
}

impl Boardstate {
    pub fn new() -> Boardstate {
        Boardstate::default()
    }

    pub fn start_from(
        white: PawnLocation,
        black: PawnLocation,
        walls: Vec<WallLocation>,
    ) -> Result<Boardstate, String> {
        let mut boardstate = Boardstate::new();
        for wall_location in walls {
            boardstate.insert_wall_at_location(wall_location)?;
        }
        boardstate.white_position = white;
        boardstate.black_position = black;

        Ok(boardstate)
    }

    pub fn get_active_player(&self) -> &Player {
        &self.active_player
    }

    pub fn get_position_white_pawn(&self) -> &PawnLocation {
        &self.white_position
    }

    pub fn get_position_black_pawn(&self) -> &PawnLocation {
        &self.black_position
    }

    pub fn get_available_walls_white_player(&self) -> &u8 {
        &self.white_available_walls
    }

    pub fn get_available_walls_black_player(&self) -> &u8 {
        &self.black_available_walls
    }

    pub fn get_wall_positions(&self) -> [Option<WallOrientation>; 71] {
        let mut wall_arrray: [Option<WallOrientation>; 71] = [const { None }; 71];
        for index in self.wall_placed.into_iter() {
            if self.wall_placed.get(index) {
                if self.wall_orientation.get(index) {
                    wall_arrray[index] = Some(WallOrientation::Vertical);
                } else {
                    wall_arrray[index] = Some(WallOrientation::Horizontal);
                }
            }
        }
        wall_arrray
    }

    /// Method to get all the legal moves for the currently active player in the
    /// current boadstate.
    ///
    /// For the pawn moves this means checking all the move directions and the weird
    /// rules around hoping over other players (need to figure out clean way to do that)
    /// for the wall moves it needs to check all the open wall positions, see what direction would
    /// be available (not blocked by existing wall) and check if it doesn't block all paths to the
    /// opposite side for either player.
    pub fn get_legal_actions(&self) -> PossibleActions {
        PossibleActions::build(self.get_possible_pawn_moves(), Vec::new())
    }

    /// The play action takes an action as input and attempts to play that move on the current
    /// board, if the action is illegal an error is returned. When the action is legal, the
    /// boardstate is updated.
    /// A check is executed if the game is won. When the game is won an enum with the Won result is
    /// returned, otherwise the active player is swapped and an InProgress enum is returned
    pub fn apply_action(&mut self, action: Action) -> Result<GameStatus, String> {
        match action {
            Action::Pawn(pawn_location) => self.move_pawn_to_location(pawn_location),
            Action::Wall(wall_location) => self.insert_wall_at_location(wall_location),
        }
    }

    

    /// Attempts to insert a wall for the currently active player.
    ///
    /// Checks if the player has walls available, if the location is not blocked by another wall
    /// and if the wall doesn't completly block of the opponent from reaching the other side.
    ///
    /// At a successfull insert the active player is swapped.
    fn insert_wall_at_location(&mut self, location: WallLocation) -> Result<GameStatus, String> {
        if self.player_has_no_walls_available() {
            return Err(String::from("No more walls left make a pawn move instead"));
        }
        // check if a wall exists on the wall coordinate or if the wall would overlap with an
        // existing wall
        let square = usize::from(location.get_square());
        if self.wall_placed.get(square) {
            return Err(format!(
                "Can't insert wall, location {} already occupied",
                square
            ));
        }
        let coordinate = location.get_coordinate();
        match location.get_orientation() {
            WallOrientation::Vertical => {
                if self.vertical_wall_at_coordinate(&coordinate.from_direction(Direction::West))
                    | self.vertical_wall_at_coordinate(&coordinate.from_direction(Direction::East))
                {
                    return Err(format!(
                        "Can't insert wall, location {} overlaps with existing wall",
                        square
                    ));
                }
                self.wall_placed.set(square, true);
                self.wall_orientation.set(square, true);
            }
            WallOrientation::Horizontal => {
                if self.horizontal_wall_at_coordinate(&coordinate.from_direction(Direction::North))
                    | self.vertical_wall_at_coordinate(&coordinate.from_direction(Direction::South))
                {
                    return Err(format!(
                        "Can't insert wall, location {} overlaps with existing wall",
                        square
                    ));
                }
                self.wall_placed.set(square, true);
                self.wall_orientation.set(square, false);
            }
        }

        self.decrease_available_walls();
        Ok(GameStatus::InProgress)
    }

    fn player_has_no_walls_available(&self) -> bool {
        match self.active_player {
            Player::White => {
                if self.white_available_walls < 1 {
                    return true;
                }
            }
            Player::Black => {
                if self.black_available_walls < 1 {
                    return true;
                }
            }
        }
        false
    }

    fn horizontal_wall_at_coordinate(&self, coordinate: &Option<Coordinate>) -> bool {
        if let Some(coordinate) = coordinate {
            println!("Input coordinate: {:?}", coordinate);
            let horizontal_wall = self.wall_placed.get(coordinate.to_square().into())
                && !self.wall_orientation.get(coordinate.to_square().into());
            println!(
                "Horizontal_wall {} at coordinate: {:?}",
                horizontal_wall, coordinate
            );
            return horizontal_wall;
        }
        true
    }

    fn vertical_wall_at_coordinate(&self, coordinate: &Option<Coordinate>) -> bool {
        if let Some(coordinate) = coordinate {
            let horizontal_wall = self.wall_placed.get(coordinate.to_square().into())
                && self.wall_orientation.get(coordinate.to_square().into());
            println!(
                "Vertical_wall {} at coordinate: {:?}",
                horizontal_wall, coordinate
            );
            return horizontal_wall;
        }
        true
    }

    fn decrease_available_walls(&mut self) {
        match self.active_player {
            Player::White => self.white_available_walls -= 1,
            Player::Black => self.black_available_walls -= 1,
        }
    }

    fn move_pawn_to_location(&mut self, location: PawnLocation) -> Result<GameStatus, String> {
        let possible_pawn_moves = self.get_possible_pawn_moves();

        if possible_pawn_moves.contains(&location) {
            match self.active_player {
                Player::White => {
                    self.white_position = location;
                    if self.is_won() {
                        // Need to figure out if I still need to switch the game state is won
                        // it seems sort of important to be able for normal undo behavior in the
                        // case of playing the computer?
                        self.active_player = Player::Black;
                        return Ok(GameStatus::Finished {
                            won_by: Player::White,
                            reason: VictoryReason::ReachedOppositeSide,
                        });
                    }
                    self.active_player = Player::Black;
                    return Ok(GameStatus::InProgress);
                }
                Player::Black => {
                    self.black_position = location;
                    if self.is_won() {
                        self.active_player = Player::White;
                        return Ok(GameStatus::Finished {
                            won_by: Player::Black,
                            reason: VictoryReason::ReachedOppositeSide,
                        });
                    }
                    self.active_player = Player::White;
                    return Ok(GameStatus::InProgress);
                }
            }
        }

        Err(format!(
            "The move to square {} is not legal.",
            location.get_square()
        ))
    }

    fn get_possible_pawn_moves(&self) -> Vec<PawnLocation> {
        let current_location = match self.active_player {
            Player::White => &self.white_position,
            Player::Black => &self.black_position,
        };

        let mut possible_pawn_moves: Vec<PawnLocation> = Vec::with_capacity(4);
        for direction in DIRECTIONS {
            if !self.is_blocked_in_direction(current_location, &direction) {
                let new_location = current_location.from_direction(direction).expect("Going off the board should be handled by the is_blocked_in_direction method");
                if !self.is_occupied_by_other_player(&new_location) {
                    possible_pawn_moves.push(new_location);
                }
                // Handle jump logic in case off other player on square
            }
        }

        possible_pawn_moves
    }

    /// Check if moving in a direction is blocked from a location, true when either a wall or the
    /// end of the board is blocking, false when the move is not blocking.
    pub fn is_blocked_in_direction(&self, location: &impl Location, direction: &Direction) -> bool {
        let coordinate = location.get_coordinate();
        match direction {
            Direction::North => {
                let wall_at_coordinate =
                    self.horizontal_wall_at_coordinate(&Some(coordinate.clone()));
                if coordinate.x > 1 {
                    return wall_at_coordinate
                        | self.horizontal_wall_at_coordinate(&coordinate.from_calculation(-1, 0));
                }
                wall_at_coordinate
            }
            Direction::East => {
                let wall_at_coordinate =
                    self.vertical_wall_at_coordinate(&Some(coordinate.clone()));
                if coordinate.y > 1 {
                    return wall_at_coordinate
                        | self.vertical_wall_at_coordinate(&coordinate.from_calculation(0, -1));
                }
                wall_at_coordinate
            }
            Direction::South => {
                if coordinate.y > 1 {
                    let wall_below_coordinate =
                        self.horizontal_wall_at_coordinate(&coordinate.from_calculation(0, -1));
                    if coordinate.x > 1 {
                        return wall_below_coordinate
                            | self.horizontal_wall_at_coordinate(
                                &coordinate.from_calculation(-1, -1),
                            );
                    }
                    return wall_below_coordinate;
                }
                true
            }
            Direction::West => {
                if coordinate.x > 1 {
                    let wall_left_of_coordinate =
                        self.vertical_wall_at_coordinate(&coordinate.from_calculation(-1, 0));
                    if coordinate.y > 1 {
                        return wall_left_of_coordinate
                            | self
                                .vertical_wall_at_coordinate(&coordinate.from_calculation(-1, -1));
                    }
                    return wall_left_of_coordinate;
                }
                true
            }
        }
    }

    fn is_occupied_by_other_player(
        &self,
        location: &PawnLocation,
    ) -> bool {
        match self.active_player {
            Player::White => location.get_square() == self.black_position.get_square(),
            Player::Black => location.get_square() == self.white_position.get_square(),
        }
    }

    fn is_won(&self) -> bool {
        match self.active_player {
            Player::White => self.white_position.get_coordinate().y == 8,
            Player::Black => self.black_position.get_coordinate().y == 1,
        }
    }
}

#[derive(Clone, Hash, Debug)]
/// An enum with the two player options
pub enum Player {
    White,
    Black,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_blocked_in_direction_empty_board() {
        let boardstate = Boardstate::new();
        let parameters = [
            (Direction::North, false),
            (Direction::East, false),
            (Direction::South, true),
            (Direction::West, false),
        ];
        for (direction, expected) in parameters {
            println!("Direction: {:?}", direction);
            let result =
                boardstate.is_blocked_in_direction(&PawnLocation::build(4).unwrap(), &direction);
            assert_eq!(result, expected);
        }
    }

    // #[test]
    // fn get_possible_pawn_moves_starting_position() {
    //     let boardstate = Boardstate::new();
    //     let result = boardstate.get_possible_pawn_moves();
    //     let expected = vec![
    //         PawnLocation::build(13).unwrap(),
    //         PawnLocation::build(5).unwrap(),
    //         PawnLocation::build(3).unwrap(),
    //     ];
    //     assert_eq!(result, expected);
    // }
    //
    // #[test]
    // fn move_pawn_up_from_starting_position() {
    //     let mut boardstate = Boardstate::new();
    //     boardstate
    //         .move_pawn_to_location(PawnLocation::build(13).unwrap())
    //         .unwrap();
    //
    //     assert_eq!(boardstate.get_position_white_pawn().get_square(), 13);
    // }
    //
    // #[test]
    // #[should_panic]
    // fn move_pawn_to_center_from_starting_position() {
    //     let mut boardstate = Boardstate::new();
    //     boardstate
    //         .move_pawn_to_location(PawnLocation::build(40).unwrap())
    //         .unwrap();
    // }
    //
    #[test]
    fn insert_wall_successfull() {
        let mut boardstate = Boardstate::new();
        boardstate
            .insert_wall_at_location(WallLocation::build(41, WallOrientation::Horizontal).unwrap())
            .unwrap();

        assert_eq!(
            boardstate.get_wall_positions()[41],
            Some(WallOrientation::Horizontal)
        );
    }
}
