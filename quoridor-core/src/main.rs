use quoridor_core::boardstate::Boardstate;

fn main() {
    let boardstate = Boardstate::new();
    let moves = boardstate.get_legal_moves();
    for new_location in moves.get_pawn_actions(){
        println!("{:?}", new_location.get_coordinate());
    }

    
}
