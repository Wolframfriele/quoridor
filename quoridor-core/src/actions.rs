use crate::locations::{PawnLocation, WallLocation};

#[derive(Clone)]
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
            0..=1 => Err(
                "Trying to create an action from a notation string that has less than 2 characters"
                    .to_string(),
            ),
            2 => {
                let pawn_location = PawnLocation::from_notation(notation)?;
                Ok(Action::Pawn(pawn_location))
            }
            3 => {
                let wall_location = WallLocation::from_notation(notation)?;
                Ok(Action::Wall(wall_location))
            }
            _ => Err(
                "Trying to create an action from a notation string that has more than 3 characters"
                    .to_string(),
            ),
        }
    }

    pub fn get_notation(&self) -> String {
        match self {
            Self::Pawn(pawn_location) => pawn_location.get_notation(),
            Self::Wall(wall_location) => wall_location.get_notation(),
        }
    }
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
    #[should_panic]
    fn new_action_notation_failed() {
        let inputs = ["a", "x1v", "B0h", "c1x", "B1vx", "x1", "A12"];
        for input in inputs {
            Action::from_notation(input).unwrap();
        }
    }
}
