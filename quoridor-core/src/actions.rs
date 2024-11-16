use crate::locations::{Coordinate, Location, PawnLocation, WallLocation, WallOrientation};

const ALPHABET: [char; 9] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Pawn(PawnLocation),
    Wall(WallLocation),
}

impl Action {
    pub fn from_pawn_location(pawn_location: PawnLocation) -> Action {
        Action::Pawn(pawn_location)
    }

    pub fn from_wall_location(wall_location: WallLocation) -> Action {
        Action::Wall(wall_location)
    }

    pub fn from_notation(notation: &str) -> Result<Self, String> {
        match notation.len() {
            2 => Ok(Action::Pawn(pawn_location_from_notation(notation)?)),
            3 => Ok(Action::Wall(wall_location_from_notation(notation)?)),
            _ => Err(format!(
                "An action notation should have 2 or 3 characters, got {}",
                notation.len()
            )),
        }
    }

    pub fn get_notation(&self) -> String {
        match self {
            Self::Pawn(pawn_location) => location_to_notation(pawn_location.get_coordinate(), None),
            Self::Wall(wall_location) => location_to_notation(
                wall_location.get_coordinate(),
                Some(wall_location.get_orientation()),
            ),
        }
    }
}

fn pawn_location_from_notation(notation: &str) -> Result<PawnLocation, String> {
    PawnLocation::from_coordinate(notation_to_coordinate(notation)?)
}

fn wall_location_from_notation(notation: &str) -> Result<WallLocation, String> {
    let coordinate = notation_to_coordinate(notation)?;
    match notation.chars().nth(2) {
        Some('v' | 'V') => Ok(WallLocation::from_coordinate(
            coordinate,
            WallOrientation::Vertical,
        )?),
        Some('h' | 'H') => Ok(WallLocation::from_coordinate(
            coordinate,
            WallOrientation::Horizontal,
        )?),
        _ => Err(String::from(
            "The last character of the notation needs to be either a v or an h",
        )),
    }
}

fn number_to_alphabet(number: u8) -> char {
    ALPHABET[usize::from(number)]
}

fn notation_to_coordinate(notation: &str) -> Result<Coordinate, String> {
    let uppercase_first_char = notation
        .to_ascii_uppercase()
        .chars()
        .next()
        .expect("Verified that notation contains 2 chars in Action::from_notation");
    let second_char = notation
        .chars()
        .nth(1)
        .expect("Verified that notation contains 2 chars in Action::from_notation");

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

fn location_to_notation(coordinate: Coordinate, orientation: Option<WallOrientation>) -> String {
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

#[derive(Default)]
pub struct PossibleActions {
    actions: Vec<Action>,
}

impl PossibleActions {
    pub fn build(
        pawn_locations: Vec<PawnLocation>,
        _wall_actions: Vec<WallLocation>,
    ) -> PossibleActions {
        let pawn_actions = pawn_locations
            .iter()
            .map(|location| Action::from_pawn_location(location.clone()))
            .collect();

        PossibleActions {
            actions: pawn_actions,
        }
    }

    pub fn get_actions(&self) -> &Vec<Action> {
        &self.actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_from_notation() {
        let input_and_expected = [
            ("A1", Action::Pawn(PawnLocation::build(0).unwrap())),
            ("A9", Action::Pawn(PawnLocation::build(72).unwrap())),
            ("b6", Action::Pawn(PawnLocation::build(46).unwrap())),
            ("E6", Action::Pawn(PawnLocation::build(49).unwrap())),
            ("H8", Action::Pawn(PawnLocation::build(70).unwrap())),
            ("i1", Action::Pawn(PawnLocation::build(8).unwrap())),
            ("I9", Action::Pawn(PawnLocation::build(80).unwrap())),
            (
                "A1v",
                Action::Wall(WallLocation::build(0, WallOrientation::Vertical).unwrap()),
            ),
            (
                "A8h",
                Action::Wall(WallLocation::build(63, WallOrientation::Horizontal).unwrap()),
            ),
            (
                "B6v",
                Action::Wall(WallLocation::build(46, WallOrientation::Vertical).unwrap()),
            ),
            (
                "E6h",
                Action::Wall(WallLocation::build(49, WallOrientation::Horizontal).unwrap()),
            ),
            (
                "F8v",
                Action::Wall(WallLocation::build(68, WallOrientation::Vertical).unwrap()),
            ),
            (
                "H1h",
                Action::Wall(WallLocation::build(7, WallOrientation::Horizontal).unwrap()),
            ),
            (
                "H8v",
                Action::Wall(WallLocation::build(70, WallOrientation::Vertical).unwrap()),
            ),
        ];
        for (input, expected) in input_and_expected {
            assert_eq!(Action::from_notation(input).unwrap(), expected);
        }
    }

    #[test]
    #[should_panic]
    fn action_from_notation_failed() {
        let inputs = ["a", "x1v", "B0h", "c1x", "B1vx", "x1", "A12", "B0", "c0x"];
        for input in inputs {
            Action::from_notation(input).unwrap();
        }
    }
}
