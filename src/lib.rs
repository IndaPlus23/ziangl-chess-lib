/*
 *                        _oo0oo_
 *                       o8888888o
 *                       88" . "88
 *                       (| -_- |)
 *                       0\  =  /0
 *                     ___/`---'\___
 *                   .' \\|     |// '.
 *                  / \\|||  :  |||// \
 *                 / _||||| -:- |||||- \
 *                |   | \\\  - /// |   |
 *                | \_|  ''\---/''  |_/ |
 *                \  .-\__  '-'  ___/-. /
 *              ___'. .'  /--.--\  `. .'___
 *           ."" '<  `.___\_<|>_/___.' >' "".
 *          | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *          \  \ `_.   \_ __\ /__ _/   .-` /  /
 *      =====`-.____`.___ \_____/___.-`___.-'=====
 *                        `=---=' 
 *                      pls no bugs
 *                   PLEASE no runtime bugs
 */
#[derive(Clone)]
pub struct Piece{
    id:String,          //unique id
    _type:i8,           //Piece type: 1=pawn, 2=rook, 3=knight, 4=bishop, 5=queen, 6=king
    location: [i8;2],
    side:bool,          //white or black, true=black, false=white
}

pub fn new_game() -> Vec<Piece>{
    let mut game:Vec<Piece> = vec![Piece{id:"".to_string(),_type:0,location:[0,0],side:false};32];

    //white Pieces
    for i in 0..=7{
        game[i] = Piece{id:"w_pawn_".to_string() + &i.to_string(), _type:1, location:[i.try_into().unwrap(),1], side:false};
    }
    game[8] = Piece{id:"w_rook_1".to_string(), _type:2, location:[0,0], side:false};
    game[9] = Piece{id:"w_knight_1".to_string(), _type:3, location:[1,0], side:false};
    game[10] = Piece{id:"w_bishop_1".to_string(), _type:4, location:[2,0], side:false};
    game[11] = Piece{id:"w_queen".to_string(), _type:5, location:[3,0], side:false};
    game[12] = Piece{id:"w_king".to_string(), _type:6, location:[4,0], side:false};
    game[13] = Piece{id:"w_bishop_2".to_string(), _type:4, location:[5,0], side:false};
    game[14] = Piece{id:"w_knight_2".to_string(), _type:3, location:[6,0], side:false};
    game[15] = Piece{id:"w_rook_2".to_string(), _type:2, location:[7,0], side:false};

    //black Pieces
    for i in 16..=23{
        game[i] = Piece{id:"b_pawn_".to_string() + &(i-16).to_string(), _type:1, location:[(i-16).try_into().unwrap(),6], side:true};
    }
    game[24] = Piece{id:"b_rook_1".to_string(), _type:2, location:[0,7], side:true};
    game[25] = Piece{id:"b_knight_1".to_string(), _type:3, location:[1,7], side:true};
    game[26] = Piece{id:"b_bishop_1".to_string(), _type:4, location:[2,7], side:true};
    game[27] = Piece{id:"b_queen".to_string(), _type:5, location:[3,7], side:true};
    game[28] = Piece{id:"b_king".to_string(), _type:6, location:[4,7], side:true};
    game[29] = Piece{id:"b_bishop_2".to_string(), _type:4, location:[5,7], side:true};
    game[30] = Piece{id:"b_knight_2".to_string(), _type:3, location:[6,7], side:true};
    game[31] = Piece{id:"b_rook_2".to_string(), _type:2, location:[7,7], side:true};

    return game;

}

pub fn make_move(id:String, position:[i8;2], mut game:Vec<Piece>) -> Vec<Piece>{
    
    for i in 0..game.len() {                            //if a piece got captured
        if game[i].location==position{
            game.remove(i);
            break;
        }
    }

    for i in 0..game.len() {                            //moves the piece
        if game[i].id==id{
            game[i].location=position;
            break;
        }
    }

    return game;

}

pub fn get_moves(id:String,game:Vec<Piece>) -> Vec<[i8;2]>{
    let mut _type:i8=0;
    let mut position:[i8;2]=[0,0];
    let mut side:bool=false;
    for i in 0..game.len() {
        if game[i].id==id {
            _type=game[i]._type;
            position=game[i].location;
            side=game[i].side;
            break;
        }
    }

    let mut result:Vec<[i8;2]>=vec![];
    match _type {

        1=>{                //pawn
            if side {       //Black

                //start with two steps
                if position[1]==6 {                 
                    for i in 0..game.len() {
                        if game[i].location==[position[0],5] {
                            result=vec![];
                            break;
                        }
                        else if game[i].location==[position[0],4] {
                            result=vec![[position[0],5]];
                            break;
                        }
                        else if i==game.len()-1 {
                            result=vec![[position[0],5],[position[0],4]];
                            break;
                        }
                    }
                }

                //one step
                else {                              
                    for i in 0..game.len() {
                        if game[i].location==[position[0],position[1]-1] {
                            result=vec![];
                            break;
                        }
                        else if i==game.len()-1 {
                            result=vec![[position[0],position[1]-1]];
                            break;
                        }
                    }
                }

                //Capture opponent if white
                for i in 0..game.len() {    
                    if !game[i].side&&game[i].location==[position[0]-1,position[1]-1]{
                        result.push([position[0]-1,position[1]-1]);
                    }
                    else if !game[i].side&&game[i].location==[position[0]+1,position[1]-1] {
                        result.push([position[0]-1,position[1]+1]);
                    }
                }
            }
            else {      //White
                //start with two steps
                if position[1]==1 {
                    for i in 0..game.len() {
                        if game[i].location==[position[0],2] {
                            result=vec![];
                            break;
                        }
                        else if game[i].location==[position[0],3] {
                            result=vec![[position[0],2]];
                            break;
                        }
                        else if i==game.len()-1 {
                            result=vec![[position[0],2],[position[0],3]];
                            break;
                        }
                    }
                }
                //one step
                else {
                    for i in 0..game.len() {
                        if game[i].location==[position[0],position[1]+1] {
                            result=vec![];
                            break;
                        }
                        else if i==game.len()-1 {
                            result=vec![[position[0],position[1]+1]];
                            break;
                        }
                    }
                }

                //Capture opponent if black
                for i in 0..game.len() {
                    if game[i].side&&game[i].location==[position[0]+1,position[1]+1]{
                        result.push([position[0]+1,position[1]+1]);
                    }
                    else if game[i].side&&game[i].location==[position[0]-1,position[1]+1] {
                        result.push([position[0]-1,position[1]+1]);
                    }
                }
            }
            result.retain(|x:&[i8;2]|0<=x[0]&&x[0]<8&&0<=x[1]&&x[1]<8);     //removes overflow out side 8x8 board
        }

        2=>{                //rook
            'outer: for i in position[0]+1..8 {                     //find move towards x+
                for j in 0..game.len() {
                    if game[j].location==[i,position[1]]{           //if some piece are in the way
                        if game[j].side!=side {                         //different side
                            result.push([i,position[1]])                //includes the location of the piece in way
                            
                        }                                               //does not include the location of the piece in way if on the same side

                        break 'outer;                               //pathfinding finished
                    }
                }
                result.push([i,position[1]]);               //add current pointer location to result
            }
            'outer: for i in (0..position[0]).rev() {                     //find move towards x-
                for j in 0..game.len() {
                    if game[j].location==[i,position[1]]{
                        if game[j].side!=side {
                            result.push([i,position[1]])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([i,position[1]]);
            }
            'outer: for i in position[1]+1..8 {                     //find move towards y+
                for j in 0..game.len() {
                    if game[j].location==[position[0],i]{
                        if game[j].side!=side {
                            result.push([position[0],i])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([position[0],i]);
            }
            'outer: for i in (0..position[1]).rev() {                     //find move towards y-
                for j in 0..game.len() {
                    if game[j].location==[position[0],i]{
                        if game[j].side!=side {
                            result.push([position[0],i])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([position[0],i]);
            }
        }

        3=>{                //knight
            //initialize vector with all possible move
            result=vec![[position[0]+1,position[1]+2],[position[0]+2,position[1]+1],[position[0]+2,position[1]-1],[position[0]+1,position[1]-2],[position[0]-1,position[1]-2],[position[0]-2,position[1]-1],[position[0]-2,position[1]+1],[position[0]-1,position[1]+2]];   
            for i in 0..game.len(){
                if game[i].location==[position[0]+1,position[1]+2]{     //remove move if ally piece is on it
                    if game[i].side==side{
                        result[0]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }
                else if game[i].location==[position[0]+2,position[1]+1]{
                    if game[i].side==side{
                        result[1]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }
                else if game[i].location==[position[0]+2,position[1]-1]{
                    if game[i].side==side{
                        result[2]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0]+1,position[1]-2]{
                    if game[i].side==side{
                        result[3]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0]-1,position[1]-2]{
                    if game[i].side==side{
                        result[4]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0]-2,position[1]-1]{
                    if game[i].side==side{
                        result[5]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0]-2,position[1]+1]{
                    if game[i].side==side{
                        result[6]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0]-1,position[1]+2]{
                    if game[i].side==side{
                        result[7]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }
            }
            result.retain(|x:&[i8;2]|0<=x[0]&&x[0]<8&&0<=x[1]&&x[1]<8);         //clear vector from overflow outside game board
        }

        4=>{        //bishop
            let mut x = position[0]+1;
            let mut y = position[1]+1;
            'outer: while x<8&&y<8{                     //find move towards x+y+
                for j in 0..game.len() {
                    if game[j].location==[x,y]{           //if some piece are in the way
                        if game[j].side!=side {                         //different side
                            result.push([x,y])                //includes the location of the piece in way
                            
                        }                                               //does not include the location of the piece in way if on the same side

                        break 'outer;                               //pathfinding finished
                    }
                }
                result.push([x,y]);               //add current pointer location to result
                x+=1;
                y+=1;
            }
            x = position[0]+1;
            y = position[1]-1;
            'outer: while x<8&&y>=0{                     //find move towards x+y-
                for j in 0..game.len() {
                    if game[j].location==[x,y]{
                        if game[j].side!=side {
                            result.push([x,y])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([x,y]);
                x+=1;
                y-=1;
            }
            x = position[0]-1;
            y = position[1]-1;
            'outer: while x>=8&&y>=0{                     //find move towards x-y-
                for j in 0..game.len() {
                    if game[j].location==[x,y]{
                        if game[j].side!=side {
                            result.push([x,y])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([x,y]);
                x-=1;
                y-=1;
            }
            x = position[0]-1;
            y = position[1]+1;
            'outer: while x>=0&&y<8{                     //find move towards x-y+
                for j in 0..game.len() {
                    if game[j].location==[x,y]{
                        if game[j].side!=side {
                            result.push([x,y])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([x,y]);
                x-=1;
                y+=1;
            }
        }
        5=>{            //queen
            //just run same pathfinding for bishop and rook
            let mut x = position[0]+1;
            let mut y = position[1]+1;
            'outer: while x<8&&y<8{                     //find move towards x+y+
                for j in 0..game.len() {
                    if game[j].location==[x,y]{           //if some piece are in the way
                        if game[j].side!=side {                         //different side
                            result.push([x,y])                //includes the location of the piece in way
                            
                        }                                               //does not include the location of the piece in way if on the same side

                        break 'outer;                               //pathfinding finished
                    }
                }
                result.push([x,y]);               //add current pointer location to result
                x+=1;
                y+=1;
            }
            x = position[0]+1;
            y = position[1]-1;
            'outer: while x<8&&y>=0{                     //find move towards x+y-
                for j in 0..game.len() {
                    if game[j].location==[x,y]{
                        if game[j].side!=side {
                            result.push([x,y])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([x,y]);
                x+=1;
                y-=1;
            }
            x = position[0]-1;
            y = position[1]-1;
            'outer: while x>=8&&y>=0{                     //find move towards x-y-
                for j in 0..game.len() {
                    if game[j].location==[x,y]{
                        if game[j].side!=side {
                            result.push([x,y])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([x,y]);
                x-=1;
                y-=1;
            }
            x = position[0]-1;
            y = position[1]+1;
            'outer: while x>=0&&y<8{                     //find move towards x-y+
                for j in 0..game.len() {
                    if game[j].location==[x,y]{
                        if game[j].side!=side {
                            result.push([x,y])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([x,y]);
                x-=1;
                y+=1;
            }
            'outer: for i in position[0]+1..8 {                     //find move towards x+
                for j in 0..game.len() {
                    if game[j].location==[i,position[1]]{           //if some piece are in the way
                        if game[j].side!=side {                         //different side
                            result.push([i,position[1]])                //includes the location of the piece in way
                            
                        }                                               //does not include the location of the piece in way if on the same side

                        break 'outer;                               //pathfinding finished
                    }
                }
                result.push([i,position[1]]);               //add current pointer location to result
            }
            'outer: for i in (0..position[0]).rev() {                     //find move towards x-
                for j in 0..game.len() {
                    if game[j].location==[i,position[1]]{
                        if game[j].side!=side {
                            result.push([i,position[1]])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([i,position[1]]);
            }
            'outer: for i in position[1]+1..8 {                     //find move towards y+
                for j in 0..game.len() {
                    if game[j].location==[position[0],i]{
                        if game[j].side!=side {
                            result.push([position[0],i])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([position[0],i]);
            }
            'outer: for i in (0..position[1]).rev() {                     //find move towards y-
                for j in 0..game.len() {
                    if game[j].location==[position[0],i]{
                        if game[j].side!=side {
                            result.push([position[0],i])
                            
                        }

                        break 'outer;
                    }
                }
                result.push([position[0],i]);
            }
        }
        6=>{
            result=vec![[position[0]+1,position[1]+1],[position[0]+1,position[1]],[position[0]+1,position[1]-1],[position[0],position[1]-1],[position[0]-1,position[1]-1],[position[0]-1,position[1]],[position[0]-1,position[1]+1],[position[0],position[1]+1]];   
            for i in 0..game.len(){
                if game[i].location==[position[0]+1,position[1]+1]{     //remove move if ally piece is on it
                    if game[i].side==side{
                        result[0]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }
                else if game[i].location==[position[0]+1,position[1]]{
                    if game[i].side==side{
                        result[1]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }
                else if game[i].location==[position[0]+1,position[1]-1]{
                    if game[i].side==side{
                        result[2]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0],position[1]-1]{
                    if game[i].side==side{
                        result[3]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0]-1,position[1]-1]{
                    if game[i].side==side{
                        result[4]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0]-1,position[1]]{
                    if game[i].side==side{
                        result[5]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0]-1,position[1]+1]{
                    if game[i].side==side{
                        result[6]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }else if game[i].location==[position[0],position[1]+1]{
                    if game[i].side==side{
                        result[7]=[-1,-1]
                    }
                    else {
                        continue;
                    }
                }
            }
        }
        _=>panic!("No matching piece type!")
    }
    result.retain(|x:&[i8;2]|0<=x[0]&&x[0]<8&&0<=x[1]&&x[1]<8); 
    return result;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_test() {
        let result = new_game();
        assert_eq!(result[0].id, "w_pawn_0".to_string());
        assert_eq!(result[8]._type, 2);
        assert_eq!(result[6].location[0], 6);
        assert_eq!(result[20].id, "b_pawn_4".to_string());
        assert_eq!(result[20].location, [4,6]);
        assert_eq!(result[23].id, "b_pawn_7".to_string());
    }

    #[test]
    fn make_move_test(){
        let mut game = new_game();
        game = make_move("w_pawn_0".to_string(), [0,6], game);
        assert_eq!(game[0].location,[0,6]);
        assert_eq!(game.len() as i32, 31)
    }

    #[test]
    fn get_moves_test(){
        let mut game=new_game();
        let mut possible_moves=get_moves("w_pawn_0".to_string(), game.clone());
        println!("{:?}",possible_moves);
        game = make_move("w_pawn_2".to_string(), [2,3], game);
        game = make_move("w_pawn_3".to_string(), [3,3], game);
        game = make_move("w_pawn_4".to_string(), [4,3], game);
        possible_moves=get_moves("w_queen".to_string(), game.clone());
        println!("{:?}",possible_moves);
        possible_moves=get_moves("b_knight_1".to_string(), game.clone());
        println!("{:?}",possible_moves);
        possible_moves=get_moves("b_pawn_1".to_string(), game.clone());
        println!("{:?}",possible_moves);
    }
}
