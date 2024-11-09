use strum::EnumIter;

const ALPHABET: [char; 9] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];

#[derive(Debug)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Hash, Debug, PartialEq)]
pub struct PawnLocation {
    square: u8,
}

impl PawnLocation {
    /// Create a new Location, the number is the square on the board where counting starts in the
    /// bottom left corner and goes right. So coordinate A1 is square 1, B1 is square 2 and A2 is
    /// square 10 etc.
    ///
    /// The square number must be between 1 and 81, or else an error will be returned.
    pub fn build(square: u8) -> Result<Self, String> {
        if (1..=81).contains(&square) {
            Ok(PawnLocation { square })
        } else {
            Err(format!(
                "The square should be in range 1..=81, but was: {square}"
            ))
        }
    }

    pub fn from_notation(notation: &str) -> Result<Self, String> {
        let coordinate = convert_to_coordinate(notation)?;
        Ok(PawnLocation {
            square: convert_to_square(coordinate),
        })
    }

    pub fn get_square(&self) -> u8 {
        assert!((1..=81).contains(&self.square));
        self.square
    }

    pub fn get_coordinate(&self) -> Coordinate {
        let remainder = &self.square % 9;
        if remainder == 0 {
            Coordinate {
                x: 9,
                y: &self.square / 9,
            }
        } else {
            Coordinate {
                x: remainder,
                y: (&self.square / 9) + 1,
            }
        }
    }

    pub fn get_notation(&self) -> String {
        convert_to_notation(self.get_coordinate(), None)
    }

    /// I don't think I love where this logic is now, maybe it could be nice to add some off the
    /// max values etc to direction and than do all the checking in the boardstate, than I only
    /// have to have this logic once
    fn new_location_from_direction(&self, direction: Direction) -> Result<PawnLocation, String> {
        println!("{}", 21 % 9);
        match direction {
            Direction::North => PawnLocation::build(&self.square + 9),
            Direction::East => {
                if self.get_coordinate().x != 9 {
                    PawnLocation::build(&self.square + 1)
                } else {
                    Err(format!("Going East from square {0} is impossible since it is on the edge of the board", &self.square))
                }
            }
            Direction::South => PawnLocation::build(&self.square - 9),
            Direction::West => {
                if self.get_coordinate().x != 1 {
                    PawnLocation::build(&self.square - 1)
                } else {
                    Err(format!("Going West from square {0} is impossible since it is on the edge of the board", &self.square))
                }
            }
        }
    }
}

#[derive(EnumIter, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Hash, Debug, PartialEq)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq)]
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

    pub fn from_notation(notation: &str) -> Result<Self, String> {
        if !notation.len() == 3 {
            return Err(format!("The notation for a wall needs to be 3 chars long, got {}", notation.len()))
        }
        let coordinate = convert_to_coordinate(&notation[0..2])?;
        match notation.chars().nth(2) {
            Some('v' | 'V') => Ok(WallLocation{ square: convert_to_square(coordinate), orientation: WallOrientation::Vertical}),
            Some('h' | 'H') => Ok(WallLocation{ square: convert_to_square(coordinate), orientation: WallOrientation::Horizontal}),
            _ => Err(String::from("The last character of the notation needs to be either a v or an h"))
        }
    }

    pub fn get_square(&self) -> u8 {
        assert!((1..=71).contains(&self.square));
        self.square
    }

    pub fn get_orientation(&self) -> WallOrientation {
        self.orientation.clone()
    }

    pub fn get_notation(&self) -> String {
        convert_to_notation(self.get_coordinate(), Some(self.get_orientation()))
    }

    fn get_coordinate(&self) -> Coordinate {
        let remainder = &self.square % 9;
        if remainder == 0 {
            Coordinate {
                x: 9,
                y: &self.square / 9,
            }
        } else {
            Coordinate {
                x: remainder,
                y: (&self.square / 9) + 1,
            }
        }
    }
}

fn number_to_alphabet(number: u8) -> char {
    ALPHABET[usize::from(number - 1)]
}

fn convert_to_notation(coordinate: Coordinate, orientation: Option<WallOrientation>) -> String {
    assert!(coordinate.x <= 9);
    assert!(coordinate.y <= 9);

    let mut notation = String::new();
    notation.push(number_to_alphabet(coordinate.x));
    notation.push(
        char::from_digit(coordinate.y.into(), 10)
            .expect("The y coordinate can never be larger than 9"),
    );

    match orientation {
        Some(WallOrientation::Horizontal) => notation.push('h'),
        Some(WallOrientation::Vertical) => notation.push('v'),
        _ => (),
    }
    notation
}

fn convert_to_coordinate(notation: &str) -> Result<Coordinate, String> {
    if notation.len() != 2 {
        return Err(format!(
            "A pawn location notation needs to be 2 characters, but was {} characters",
            notation.len()
        ));
    }
    let uppercase_first_char = notation
        .to_ascii_uppercase()
        .chars()
        .next()
        .expect("Already verified that notation contains 2 chars");
    let second_char = notation
        .chars()
        .nth(1)
        .expect("Already verified that notation contains 2 chars");

    if let Ok(index) = ALPHABET.binary_search(&uppercase_first_char) {
        if let Some(y) = second_char.to_digit(10) {
            if y != 0 {
                return Ok(Coordinate {
                    x: (index + 1).try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
            return Err(String::from("The second character can not be 0"));
        }
        return Err(format!(
            "The second character needs to be a number between 1 and 9, but got {second_char}"
        ));
    }
    Err(format!("The first character of a notation needs to be a letter between A and I, but got {uppercase_first_char}"))
}

fn convert_to_square(coordinate: Coordinate) -> u8 {
    (coordinate.y - 1) * 9 + coordinate.x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_from_location() {
        let combinations = [
            (PawnLocation::build(1).unwrap(), Coordinate { x: 1, y: 1 }),
            (PawnLocation::build(19).unwrap(), Coordinate { x: 1, y: 3 }),
            (PawnLocation::build(21).unwrap(), Coordinate { x: 3, y: 3 }),
            (PawnLocation::build(36).unwrap(), Coordinate { x: 9, y: 4 }),
            (PawnLocation::build(45).unwrap(), Coordinate { x: 9, y: 5 }),
            (PawnLocation::build(77).unwrap(), Coordinate { x: 5, y: 9 }),
            (PawnLocation::build(81).unwrap(), Coordinate { x: 9, y: 9 }),
        ];
        for (input, expected) in combinations {
            assert_eq!(input.get_coordinate().x, expected.x);
            assert_eq!(input.get_coordinate().y, expected.y);
        }
    }

    #[test]
    fn pawn_location_from_notation() {
        let input_and_expected = [
            ("A1", PawnLocation::build(1).unwrap()),
            ("A9", PawnLocation::build(73).unwrap()),
            ("b6", PawnLocation::build(47).unwrap()),
            ("E6", PawnLocation::build(50).unwrap()),
            ("H8", PawnLocation::build(71).unwrap()),
            ("i1", PawnLocation::build(9).unwrap()),
            ("I9", PawnLocation::build(81).unwrap()),
        ];
        for (input, expected) in input_and_expected {
            assert_eq!(PawnLocation::from_notation(input).unwrap(), expected);
        }
    }

    #[test]
    fn new_pawn_location_from_direction_east_succesfull() {
        let starting_location = PawnLocation::build(5).unwrap();
        let result = starting_location.new_location_from_direction(Direction::East);
        assert_eq!(result.unwrap().get_square(), 6)
    }

    #[test]
    #[should_panic]
    fn new_pawn_location_from_direction_north_failed() {
        let starting_location = PawnLocation::build(77).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::North)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn new_pawn_location_from_direction_east_failed() {
        let starting_location = PawnLocation::build(9).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::East)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn new_pawn_location_from_direction_south_failed() {
        let starting_location = PawnLocation::build(5).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::South)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn new_pawn_location_from_direction_west_failed() {
        let starting_location = PawnLocation::build(46).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::West)
            .unwrap();
    }

    #[test]
    fn new_walllocation_successfull() {
        let invalid_square: Vec<u8> = vec![9, 18, 27, 36, 45, 54, 63];
        for square in 1..=71 {
            if !invalid_square.contains(&square) {
                let location = WallLocation::build(square, WallOrientation::Vertical).unwrap();
                assert_eq!(location.get_square(), square);
            }
        }
    }

    #[test]
    #[should_panic]
    fn new_walllocation_failed() {
        let location = WallLocation::build(36, WallOrientation::Horizontal).unwrap();
    }

    #[test]
    fn wall_location_from_notation() {
        let input_and_expected = [
            ("A1v", WallLocation::build(1, WallOrientation::Vertical).unwrap()),
            ("A8h", WallLocation::build(64, WallOrientation::Horizontal).unwrap()),
            ("B6v", WallLocation::build(47, WallOrientation::Vertical).unwrap()),
            ("E6h", WallLocation::build(50, WallOrientation::Horizontal).unwrap()),
            ("F8v", WallLocation::build(69, WallOrientation::Vertical).unwrap()),
            ("H1h", WallLocation::build(8, WallOrientation::Horizontal).unwrap()),
            ("H8v", WallLocation::build(71, WallOrientation::Vertical).unwrap()),
        ];
        for (input, expected) in input_and_expected {
            assert_eq!(WallLocation::from_notation(input).unwrap(), expected);
        }
    }
}
