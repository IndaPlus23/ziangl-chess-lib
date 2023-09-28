# ziangl-chess

### Struct `Piece`
Includes a unique id for all pieces as `String`.

Includes the type (pawn, rook, knight, bishop, queen, king) as `i8`.

Includes the location as coordinates `[x,y]` in an array.

![chess board](/readme_pic.png)

Includes color as `bool`, `true` is black, `false` is white.

### Function `new_game()`
Returns a vector of all pieces (game vector)

### Function `make_move(String, [i8;2], Vec<Piece>) -> Vec<Piece>`
Takes a piece id and a game vector, makes a move for the piece with the inputed id and remove the captured opponent if any.
Returns the updated game vector.

### Function `get_moves(String,Vec<Piece>) -> Vec<[i8;2]>`
Takes a piece id and a game vector, returns a vector of all available moves for that piece in `[x,y]` coordinates.