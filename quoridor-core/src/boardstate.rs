use anyhow::{ensure, Result};
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
        active_player: Option<Player>,
    ) -> Result<Boardstate> {
        let mut boardstate = Boardstate::new();
        for wall_location in walls {
            boardstate.insert_wall_at_location(wall_location)?;
        }
        boardstate.white_position = white;
        boardstate.black_position = black;
        if let Some(player) = active_player {
            boardstate.active_player = player;
        }

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

    pub fn wall_at_coordinate(&self, coordinate: &Coordinate) -> Option<WallOrientation> {
        if (0..8u8).contains(&coordinate.x)
            && (0..7u8).contains(&coordinate.y)
            && self.wall_placed.get(coordinate.to_square().into())
        {
            return match self.wall_orientation.get(coordinate.to_square().into()) {
                false => Some(WallOrientation::Horizontal),
                true => Some(WallOrientation::Vertical),
            };
        }
        None
    }

    pub fn horizontal_wall_at_coordinate(&self, coordinate: &Coordinate) -> bool {
        let mut wall_option_1 = false;
        if (0..8u8).contains(&coordinate.x) && (0..7u8).contains(&coordinate.y) {
            wall_option_1 = self.wall_placed.get(coordinate.to_square().into())
                && !self.wall_orientation.get(coordinate.to_square().into());
        }

        let mut wall_option_2 = false;
        let second_coordinate = coordinate.from_calculation(-1, 0);
        if let Some(second_coordinate) = second_coordinate {
            if (0..8u8).contains(&second_coordinate.x) && (0..7u8).contains(&second_coordinate.y) {
                wall_option_2 = self.wall_placed.get(second_coordinate.to_square().into())
                    && !self
                        .wall_orientation
                        .get(second_coordinate.to_square().into());
            }
        }

        wall_option_1 | wall_option_2
    }

    pub fn vertical_wall_at_coordinate(&self, coordinate: &Coordinate) -> bool {
        let mut wall_option_1 = false;
        if (0..7u8).contains(&coordinate.x) && (0..8u8).contains(&coordinate.y) {
            wall_option_1 = self.wall_placed.get(coordinate.to_square().into())
                && self.wall_orientation.get(coordinate.to_square().into());
        }

        let mut wall_option_2 = false;
        let second_coordinate = coordinate.from_calculation(0, -1);
        if let Some(second_coordinate) = second_coordinate {
            if (0..7u8).contains(&second_coordinate.x) && (0..8u8).contains(&second_coordinate.y) {
                wall_option_2 = self.wall_placed.get(second_coordinate.to_square().into())
                    && self
                        .wall_orientation
                        .get(second_coordinate.to_square().into());
            }
        }

        wall_option_1 | wall_option_2
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
    pub fn apply_action(&mut self, action: Action) -> Result<GameStatus> {
        match action {
            Action::Pawn(pawn_location) => self.move_pawn_to_location(pawn_location),
            Action::Wall(wall_location) => self.insert_wall_at_location(wall_location),
        }
    }

    fn move_pawn_to_location(&mut self, location: PawnLocation) -> Result<GameStatus> {
        ensure!(
            self.get_possible_pawn_moves().contains(&location),
            format!("The move to square {} is not legal.", location.get_square())
        );
        match self.active_player {
            Player::White => {
                self.white_position = location;
            }
            Player::Black => {
                self.black_position = location;
            }
        }

        if self.is_won() {
            return Ok(GameStatus::Finished {
                won_by: self.get_active_player().clone(),
                reason: VictoryReason::ReachedOppositeSide,
            });
        }

        self.swap_active_player();
        Ok(GameStatus::InProgress)
    }

    fn get_possible_pawn_moves(&self) -> Vec<PawnLocation> {
        let current_location = match self.active_player {
            Player::White => &self.white_position,
            Player::Black => &self.black_position,
        };

        let mut possible_pawn_moves: Vec<PawnLocation> = Vec::with_capacity(4);
        for direction in DIRECTIONS {
            if !self.is_blocked_in_direction(current_location, &direction) {
                let new_location = current_location.from_direction(&direction).expect(
                    "Going off the board should be handled by the is_blocked_in_direction method",
                );

                if self.not_occupied_by_other_player(&new_location) {
                    possible_pawn_moves.push(new_location);
                } else {
                    possible_pawn_moves
                        .append(&mut self.get_possible_jump_moves(&direction, &new_location));
                }
            }
        }

        possible_pawn_moves
    }

    fn get_possible_jump_moves(
        &self,
        direction: &Direction,
        occupied_location: &PawnLocation,
    ) -> Vec<PawnLocation> {
        // check if new location is blocked in direction
        // if not blocked return the new location from going in the direction from the new location
        // if blocked check the two other directions (for when going North is blocked, check East
        // and West)
        // add the directions that are possible

        let mut possible_jump_moves: Vec<PawnLocation> = Vec::with_capacity(3);

        if !self.is_blocked_in_direction(occupied_location, direction) {
            possible_jump_moves.push(occupied_location.from_direction(direction).expect(
                "Going off the board should be handled by the is_blocked_in_direction method",
            ));
        } else {
            for perpendicular_direction in direction.get_perpendicular_directions() {
                if !self.is_blocked_in_direction(occupied_location, &perpendicular_direction) {
                    possible_jump_moves.push(occupied_location.from_direction(&perpendicular_direction).expect(
                        "Going off the board should be handled by the is_blocked_in_direction method",
                    ));
                }
            }
        }
        possible_jump_moves
    }

    /// Check if moving in a direction is blocked from a location, true when either a wall or the
    /// end of the board is blocking, false when the move is not blocking.
    fn is_blocked_in_direction(&self, location: &impl Location, direction: &Direction) -> bool {
        let coordinate = location.get_coordinate();
        match direction {
            Direction::North => {
                if coordinate.y < 8 {
                    return self.horizontal_wall_at_coordinate(&coordinate);
                }
                true
            }
            Direction::East => {
                if coordinate.x < 8 {
                    return self.vertical_wall_at_coordinate(&coordinate);
                }
                true
            }
            Direction::South => {
                if let Some(coordinate_below_current) = coordinate.from_calculation(0, -1) {
                    return self.horizontal_wall_at_coordinate(&coordinate_below_current);
                }
                true
            }
            Direction::West => {
                if let Some(coordinate_left_of_current) = coordinate.from_calculation(-1, 0) {
                    return self.vertical_wall_at_coordinate(&coordinate_left_of_current);
                }
                true
            }
        }
    }

    fn not_occupied_by_other_player(&self, location: &PawnLocation) -> bool {
        match self.active_player {
            Player::White => location.get_square() != self.black_position.get_square(),
            Player::Black => location.get_square() != self.white_position.get_square(),
        }
    }

    fn is_won(&self) -> bool {
        match self.active_player {
            Player::White => self.white_position.get_coordinate().y == 8,
            Player::Black => self.black_position.get_coordinate().y == 0,
        }
    }

    fn swap_active_player(&mut self) {
        self.active_player = match self.active_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }

    /// Attempts to insert a wall for the currently active player.
    ///
    /// Checks if the player has walls available, if the location is not blocked by another wall
    /// and if the wall doesn't completly block of the opponent from reaching the other side.
    ///
    /// At a successfull insert the active player is swapped.
    fn insert_wall_at_location(&mut self, location: WallLocation) -> Result<GameStatus> {
        ensure!(
            self.player_has_walls_available(),
            "No more walls left make a pawn move instead"
        );

        let square = usize::from(location.get_square());
        let coordinate = location.get_coordinate();

        ensure!(
            self.wall_at_coordinate(&coordinate).is_none(),
            format!("Can't insert wall, location {} already occupied", square)
        );

        match location.get_orientation() {
            WallOrientation::Vertical => {
                ensure!(
                    !self.vertical_wall_at_coordinate(&coordinate),
                    format!(
                        "Can't insert wall, location {} overlaps with existing wall",
                        square
                    )
                );

                self.wall_orientation.set(square, true);
            }

            WallOrientation::Horizontal => {
                ensure!(
                    !self.horizontal_wall_at_coordinate(&coordinate),
                    format!(
                        "Can't insert wall, location {} overlaps with existing wall",
                        square
                    )
                );

                self.wall_orientation.set(square, false);
            }
        }

        self.wall_placed.set(square, true);
        self.decrease_available_walls();
        self.swap_active_player();
        Ok(GameStatus::InProgress)
    }

    fn player_has_walls_available(&self) -> bool {
        match self.active_player {
            Player::White => {
                if self.white_available_walls > 0 {
                    return true;
                }
            }
            Player::Black => {
                if self.black_available_walls > 0 {
                    return true;
                }
            }
        }
        false
    }

    fn decrease_available_walls(&mut self) {
        match self.active_player {
            Player::White => self.white_available_walls -= 1,
            Player::Black => self.black_available_walls -= 1,
        }
    }
}

#[derive(Clone, Hash, Debug, PartialEq)]
/// An enum with the two player options
pub enum Player {
    White,
    Black,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_from_position() {
        let white_position = PawnLocation::build(0).unwrap();
        let black_position = PawnLocation::build(80).unwrap();
        let boardstate = Boardstate::start_from(
            white_position.clone(),
            black_position.clone(),
            vec![
                WallLocation::build(0, WallOrientation::Vertical).unwrap(),
                WallLocation::build(70, WallOrientation::Vertical).unwrap(),
                WallLocation::build(40, WallOrientation::Horizontal).unwrap(),
            ],
            None,
        )
        .unwrap();

        let mut expected_wall_positions: [Option<WallOrientation>; 71] = [const { None }; 71];
        expected_wall_positions[0] = Some(WallOrientation::Vertical);
        expected_wall_positions[70] = Some(WallOrientation::Vertical);
        expected_wall_positions[40] = Some(WallOrientation::Horizontal);
        assert_eq!(boardstate.get_position_white_pawn(), &white_position);
        assert_eq!(boardstate.get_position_black_pawn(), &black_position);
        assert_eq!(boardstate.get_wall_positions(), expected_wall_positions);
        assert_eq!(boardstate.get_available_walls_white_player(), &8u8);
        assert_eq!(boardstate.get_available_walls_black_player(), &9u8);
        assert_eq!(boardstate.get_active_player(), &Player::Black);
    }

    #[test]
    #[should_panic]
    fn start_from_position_overlapping_wall() {
        let _boardstate = Boardstate::start_from(
            PawnLocation::build(22).unwrap(),
            PawnLocation::build(67).unwrap(),
            vec![
                WallLocation::build(40, WallOrientation::Horizontal).unwrap(),
                WallLocation::build(41, WallOrientation::Horizontal).unwrap(),
            ],
            None,
        )
        .unwrap();
    }

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
            let result =
                boardstate.is_blocked_in_direction(&PawnLocation::build(4).unwrap(), &direction);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn is_blocked_in_direction_wall_north_and_east() {
        let boardstate = Boardstate::start_from(
            PawnLocation::build(22).unwrap(),
            PawnLocation::build(67).unwrap(),
            vec![
                WallLocation::build(22, WallOrientation::Vertical).unwrap(),
                WallLocation::build(21, WallOrientation::Horizontal).unwrap(),
            ],
            None,
        )
        .unwrap();
        let parameters = [
            (Direction::North, true),
            (Direction::East, true),
            (Direction::South, false),
            (Direction::West, false),
        ];
        for (direction, expected) in parameters {
            let result =
                boardstate.is_blocked_in_direction(&PawnLocation::build(22).unwrap(), &direction);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn is_blocked_in_direction_wall_south_and_west() {
        let boardstate = Boardstate::start_from(
            PawnLocation::build(22).unwrap(),
            PawnLocation::build(67).unwrap(),
            vec![
                WallLocation::build(13, WallOrientation::Horizontal).unwrap(),
                WallLocation::build(12, WallOrientation::Vertical).unwrap(),
            ],
            None,
        )
        .unwrap();
        let parameters = [
            (Direction::North, false),
            (Direction::East, false),
            (Direction::South, true),
            (Direction::West, true),
        ];
        for (direction, expected) in parameters {
            let result =
                boardstate.is_blocked_in_direction(&PawnLocation::build(22).unwrap(), &direction);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn get_possible_pawn_moves_starting_position() {
        let boardstate = Boardstate::new();
        let result = boardstate.get_possible_pawn_moves();
        let expected = vec![
            PawnLocation::build(13).unwrap(),
            PawnLocation::build(5).unwrap(),
            PawnLocation::build(3).unwrap(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn move_pawn_up_from_starting_position() {
        let mut boardstate = Boardstate::new();
        assert_eq!(
            boardstate
                .move_pawn_to_location(PawnLocation::build(13).unwrap())
                .unwrap(),
            GameStatus::InProgress
        );

        assert_eq!(boardstate.get_position_white_pawn().get_square(), 13);
    }

    #[test]
    fn white_move_to_top_row_should_win() {
        let mut boardstate = Boardstate::start_from(
            PawnLocation::build(67).unwrap(),
            PawnLocation::build(14).unwrap(),
            Vec::new(),
            None,
        )
        .unwrap();
        assert_eq!(
            boardstate
                .move_pawn_to_location(PawnLocation::build(76).unwrap())
                .unwrap(),
            GameStatus::Finished {
                won_by: Player::White,
                reason: VictoryReason::ReachedOppositeSide
            }
        )
    }

    #[test]
    fn black_move_to_bottom_row_should_win() {
        let mut boardstate = Boardstate::start_from(
            PawnLocation::build(67).unwrap(),
            PawnLocation::build(14).unwrap(),
            Vec::new(),
            Some(Player::Black),
        )
        .unwrap();
        assert_eq!(
            boardstate
                .move_pawn_to_location(PawnLocation::build(5).unwrap())
                .unwrap(),
            GameStatus::Finished {
                won_by: Player::Black,
                reason: VictoryReason::ReachedOppositeSide
            }
        )
    }

    #[test]
    #[should_panic]
    fn move_pawn_to_center_from_starting_position() {
        let mut boardstate = Boardstate::new();
        boardstate
            .move_pawn_to_location(PawnLocation::build(40).unwrap())
            .unwrap();
    }

    #[test]
    fn pawn_jump_over_other_player() {
        let boardstate = Boardstate::start_from(
            PawnLocation::build(40).unwrap(),
            PawnLocation::build(49).unwrap(),
            Vec::new(), 
            None,
        ).unwrap();
        let possible_moves = boardstate.get_possible_pawn_moves();
        assert!(possible_moves.contains(&PawnLocation::build(58).unwrap()));
    }

    #[test]
    fn pawn_jump_over_other_player_wall_behind() {
        let boardstate = Boardstate::start_from(
            PawnLocation::build(40).unwrap(),
            PawnLocation::build(49).unwrap(),
            vec![
                WallLocation::build(49, WallOrientation::Horizontal).unwrap(),
            ], 
            None,
        ).unwrap();
        let possible_moves = boardstate.get_possible_pawn_moves();
        assert!(possible_moves.contains(&PawnLocation::build(48).unwrap()));
        assert!(possible_moves.contains(&PawnLocation::build(50).unwrap()));
    }

    #[test]
    fn pawn_jump_over_other_player_wall_behind_and_side() {
        let boardstate = Boardstate::start_from(
            PawnLocation::build(40).unwrap(),
            PawnLocation::build(49).unwrap(),
            vec![
                WallLocation::build(49, WallOrientation::Horizontal).unwrap(),
                WallLocation::build(48, WallOrientation::Vertical).unwrap(),
            ], 
            None,
        ).unwrap();
        let possible_moves = boardstate.get_possible_pawn_moves();
        assert!(!possible_moves.contains(&PawnLocation::build(48).unwrap()));
        assert!(possible_moves.contains(&PawnLocation::build(50).unwrap()));
    }

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

    #[test]
    #[should_panic]
    fn insert_wall_failed_same_location() {
        let mut boardstate = Boardstate::new();
        boardstate
            .insert_wall_at_location(WallLocation::build(41, WallOrientation::Horizontal).unwrap())
            .unwrap();
        boardstate
            .insert_wall_at_location(WallLocation::build(41, WallOrientation::Vertical).unwrap())
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn insert_wall_failed_overlap() {
        let mut boardstate = Boardstate::new();
        boardstate
            .insert_wall_at_location(WallLocation::build(41, WallOrientation::Horizontal).unwrap())
            .unwrap();
        boardstate
            .insert_wall_at_location(WallLocation::build(42, WallOrientation::Horizontal).unwrap())
            .unwrap();
    }
}
