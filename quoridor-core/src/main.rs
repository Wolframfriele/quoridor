use anyhow::Result;

use quoridor_core::actions::Action;
use quoridor_core::gamestate::Gamestate;
use quoridor_core::locations::{Coordinate, PawnLocation, WallLocation, WallOrientation};
use quoridor_core::visualize::{print_action, print_board_state};

fn main() -> Result<()> {
    let mut gamestate = Gamestate::new(quoridor_core::gamestate::TimeControl::Unlimited);

    println!("Starting state");
    print_board_state(gamestate.get_boardstate());

    // white
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 1 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // black
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 7 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // white
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 2 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // black
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 6 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // white
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 3 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // black
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 5 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // white
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 4 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // black
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 3 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // white
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 5 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // black
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 2 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // white
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 6 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // black
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 1 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // white
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 7 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    // black
    let input_action =
        Action::Pawn(PawnLocation::from_coordinate(Coordinate { x: 4, y: 0 }).unwrap());
    println!("   {:?}\n", gamestate.execute_action(input_action)?);
    print_action(gamestate.get_boardstate(), &input_action);

    Ok(())
}
