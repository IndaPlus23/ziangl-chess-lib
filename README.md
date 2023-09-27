# ziangl-chess

### Struct `Piece`
Includes a unique id for all pieces as `String`.

Includes the type (pawn, rook, knight, bishop, queen, king) as `i8`.

Includes the location as coordinates `[x,y]` in an array.

![chess board](/readme_pic.png)

Includes color as `bool`, `true` is black, `false` is white.

### Function `new_game()`
Returns a vector of all pieces

### Function `make_move(String, [i8;2], Vec<Piece>) -> Vec<Piece>`
Makes a move and remove the captured opponent if any.
Returns the updated vector.

### Function `get_moves()`
Not finished