# Quoridor implementation

Quoridor is a game where the goal is to be the first player to walk their pawn to the opposite side. Each turn you can either move your pawn or place a wall. Each player can place 10 walls, but there always needs to be a route for each pawn to reach the opposite side.

This repository contains code related to playing Quoridor.

Eventually I want the following functionality:

- Visual user interface where players can:
    - Play against each other
    - Play against quoridor engines
    - Let their own developed engines play against other developers engines
    - Play Quoridor puzzles
    - Analyze quoridor game play

- A quoridor core library that can be used in engines and through wasm that:
    - Keeps board / game state
    - Possible moves
    - Move legality
    - Game finished status
        - A game is finished when either player has reached the opposite side
        - Either player has resigned
        - Either player has run out of time

- A game engine that gives the "best" moves to play

- An analysis tool that to step through finished games and analyse how close they where to optimal play / where there were big missed oppurtunities etc. 

## User interface

Clean and simple, lichess is a major inspiration, show the following in game: 
- pawns, 
- the placed walls 
- the left over walls per player
- who's turn
- the time left per player
- the column and row names
- highlight the last move
- when clicking or holding a pawn -> the possible moves with the pawn
- when hovering a wall placement -> if the placement is legal (this might be hard since each wall position will require path finding to determine legality)
- a history of moves up till that point with options to step through
- option to resign
- when playing the computer option to take back move
- when playing other player, the connection status

## Quoridor-core library

The core contains boardstate and gamestate: 

The boardstate only contains a singel state of the board and should be easy to copy over for stuff like Monte Carlo Tree Search (MCTS). On a state it should be possible to get possible moves for a player, should maybe be split into pawn move and wall move, execute a move for a player (returning a new boardstate when legal), and maybe execute a random move (I need to look into MCTS a bit more again).

The gamestate contains the game_id, the current boardstate, the active player, the previous moves, the time left for each player, finished status (who won and how).

