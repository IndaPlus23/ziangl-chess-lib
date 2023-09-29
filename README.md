# ziangl-chess

### Struct `Piece`
Includes a unique id for all pieces as `String`.

All id's listed from left to right, top to bottom:

`b_rook_1, b_knight_1, b_bishop_1, b_queen, b_king, b_bishop_2, b_knight_2, b_rook_2`
`b_pawn_<0-7>`
`w_pawn_<0-7>`
`w_rook_1, w_knight_1, w_bishop_1, w_queen, w_king, w_bishop_2, w_knight_2, w_rook_2`

Includes the type (pawn, rook, knight, bishop, queen, king) as `i8`.

Includes the location as coordinates `[x,y]` in an array.

![chess board](/readme_pic.png)

Includes color as `bool`, `true` is black, `false` is white.

### Function `new_game()`
Returns a vector of all pieces (game vector)

### Function `make_move(String, [i8;2], Vec<Piece>) -> Vec<Piece>`
Takes a piece id, a coordinate, and a game vector, makes a move for the piece with the inputed id and remove the captured opponent if any.
Returns the updated game vector.

### Function `get_moves(String,Vec<Piece>) -> Vec<[i8;2]>`
Takes a piece id and a game vector, returns a vector of all available moves for that piece in `[x,y]` coordinates.

### Function `promotion(String,i8,Vec<Piece>)->Vec<Piece>`
Takes a piece id(must be pawn), a type of chess piece to be promoted to, and a game vector, returns the updated game vector.

### Function `get_game_state(Vec<Piece>)->i32`
Takes a game vector, return the current game status: 1=checkmate, 2=check, 3=in progress

Warning:Edge case to be fixed!
