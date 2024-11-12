use strum::{Display, EnumIter};

const ALPHABET: [char; 9] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];

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
        if (0..=80).contains(&square) {
            Ok(PawnLocation { square })
        } else {
            Err(format!(
                "The square should be in range 0..=80, but was: {square}"
            ))
        }
    }

    pub fn from_notation(notation: &str) -> Result<Self, String> {
        let coordinate = convert_notation_to_coordinate(notation)?;
        Ok(PawnLocation {
            square: convert_coordinate_to_square(coordinate),
        })
    }

    pub fn get_square(&self) -> u8 {
        assert!((0..=80).contains(&self.square));
        self.square
    }

    pub fn get_coordinate(&self) -> Coordinate {
        Coordinate {
            x: &self.square % 9,
            y: &self.square / 9,
        }
    }

    pub fn get_notation(&self) -> String {
        convert_location_to_notation(self.get_square(), None)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WallLocation {
    square: u8,
    orientation: WallOrientation,
}

impl WallLocation {
    /// Create a new WallCoordinate. There are 64 unique positions that a wall can be placed in,
    /// but the node value can be between 1 and 71. This is because the coordinate of the squares
    /// is
    pub fn build(square: u8, orientation: WallOrientation) -> Result<Self, String> {
        if (0..=70).contains(&square) && &square % 9 != 8 {
            Ok(WallLocation {
                square,
                orientation,
            })
        } else {
            Err(format!("The square should be in the range 0..=70 and should not be divisible by 9, but was {square}"))
        }
    }

    pub fn from_notation(notation: &str) -> Result<Self, String> {
        if !notation.len() == 3 {
            return Err(format!(
                "The notation for a wall needs to be 3 chars long, got {}",
                notation.len()
            ));
        }
        let coordinate = convert_notation_to_coordinate(&notation[0..2])?;
        match notation.chars().nth(2) {
            Some('v' | 'V') => Ok(WallLocation {
                square: convert_coordinate_to_square(coordinate),
                orientation: WallOrientation::Vertical,
            }),
            Some('h' | 'H') => Ok(WallLocation {
                square: convert_coordinate_to_square(coordinate),
                orientation: WallOrientation::Horizontal,
            }),
            _ => Err(String::from(
                "The last character of the notation needs to be either a v or an h",
            )),
        }
    }

    pub fn get_square(&self) -> u8 {
        assert!((0..=70).contains(&self.square));
        self.square
    }

    pub fn get_orientation(&self) -> WallOrientation {
        self.orientation.clone()
    }

    pub fn get_notation(&self) -> String {
        convert_location_to_notation(self.get_square(), Some(self.get_orientation()))
    }

    fn get_coordinate(&self) -> Coordinate {
        convert_square_to_coordinate(self.square)
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Display)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

#[derive(EnumIter, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

fn convert_coordinate_to_square(coordinate: Coordinate) -> u8 {
    (coordinate.y) * 9 + coordinate.x
}

fn convert_square_to_coordinate(square: u8) -> Coordinate {
    Coordinate {
        x: square % 9,
        y: square / 9,
    }
}

fn convert_location_to_notation(square: u8, orientation: Option<WallOrientation>) -> String {
    let coordinate = convert_square_to_coordinate(square);

    let mut notation = String::new();
    notation.push(number_to_alphabet(coordinate.x));
    notation.push(
        char::from_digit((coordinate.y + 1).into(), 10)
            .expect("The y coordinate can never be larger than 8"),
    );

    match orientation {
        Some(WallOrientation::Horizontal) => notation.push('h'),
        Some(WallOrientation::Vertical) => notation.push('v'),
        _ => (),
    }
    notation
}

fn number_to_alphabet(number: u8) -> char {
    ALPHABET[usize::from(number)]
}

fn convert_notation_to_coordinate(notation: &str) -> Result<Coordinate, String> {
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

    if let Ok(x) = ALPHABET.binary_search(&uppercase_first_char) {
        if let Some(y) = second_char.to_digit(10) {
            if y != 0 {
                return Ok(Coordinate {
                    x: x.try_into().unwrap(),
                    y: (y - 1).try_into().unwrap(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_from_location() {
        let combinations = [
            (PawnLocation::build(0).unwrap(), Coordinate { x: 0, y: 0 }),
            (PawnLocation::build(18).unwrap(), Coordinate { x: 0, y: 2 }),
            (PawnLocation::build(20).unwrap(), Coordinate { x: 2, y: 2 }),
            (PawnLocation::build(35).unwrap(), Coordinate { x: 8, y: 3 }),
            (PawnLocation::build(44).unwrap(), Coordinate { x: 8, y: 4 }),
            (PawnLocation::build(76).unwrap(), Coordinate { x: 4, y: 8 }),
            (PawnLocation::build(80).unwrap(), Coordinate { x: 8, y: 8 }),
        ];
        for (input, expected) in combinations {
            assert_eq!(input.get_coordinate().x, expected.x);
            assert_eq!(input.get_coordinate().y, expected.y);
        }
    }

    #[test]
    fn pawn_location_from_notation() {
        let input_and_expected = [
            ("A1", PawnLocation::build(0).unwrap()),
            ("A9", PawnLocation::build(72).unwrap()),
            ("b6", PawnLocation::build(46).unwrap()),
            ("E6", PawnLocation::build(49).unwrap()),
            ("H8", PawnLocation::build(70).unwrap()),
            ("i1", PawnLocation::build(8).unwrap()),
            ("I9", PawnLocation::build(80).unwrap()),
        ];
        for (input, expected) in input_and_expected {
            assert_eq!(PawnLocation::from_notation(input).unwrap(), expected);
        }
    }

    #[test]
    #[should_panic]
    fn pawn_location_from_notation_failed() {
        let inputs = ["x1", "A1v", "B0"];
        for input in inputs {
            PawnLocation::from_notation(input).unwrap();
        }
    }

    #[test]
    fn new_walllocation_successfull() {
        let invalid_square: Vec<u8> = vec![8, 17, 26, 35, 44, 53, 62];
        for square in 0..=70 {
            if !invalid_square.contains(&square) {
                let location = WallLocation::build(square, WallOrientation::Vertical).unwrap();
                assert_eq!(location.get_square(), square);
            }
        }
    }

    #[test]
    #[should_panic]
    fn new_walllocation_failed() {
        let location = WallLocation::build(35, WallOrientation::Horizontal).unwrap();
    }

    #[test]
    fn wall_location_from_notation() {
        let input_and_expected = [
            (
                "A1v",
                WallLocation::build(0, WallOrientation::Vertical).unwrap(),
            ),
            (
                "A8h",
                WallLocation::build(63, WallOrientation::Horizontal).unwrap(),
            ),
            (
                "B6v",
                WallLocation::build(46, WallOrientation::Vertical).unwrap(),
            ),
            (
                "E6h",
                WallLocation::build(49, WallOrientation::Horizontal).unwrap(),
            ),
            (
                "F8v",
                WallLocation::build(68, WallOrientation::Vertical).unwrap(),
            ),
            (
                "H1h",
                WallLocation::build(7, WallOrientation::Horizontal).unwrap(),
            ),
            (
                "H8v",
                WallLocation::build(70, WallOrientation::Vertical).unwrap(),
            ),
        ];
        for (input, expected) in input_and_expected {
            assert_eq!(WallLocation::from_notation(input).unwrap(), expected);
        }
    }

    #[test]
    #[should_panic]
    fn wall_location_from_notation_failed() {
        let inputs = ["A1", "x1v", "B0h", "c0x"];
        for input in inputs {
            WallLocation::from_notation(input).unwrap();
        }
    }
}
