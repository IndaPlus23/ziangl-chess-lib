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
    let mut game:Vec<Piece> = vec![Piece{id:String::from(""),_type:0,location:[0,0],side:false};32];

    //white Pieces
    for i in 0..=7{
        game[i] = Piece{id:"w_pawn_".to_owned() + &i.to_string(), _type:1, location:[1,i.try_into().unwrap()], side:false};
    }
    game[8] = Piece{id:"w_rook_1".to_string(), _type:2, location:[0,0], side:false};
    game[9] = Piece{id:"w_knight_1".to_string(), _type:3, location:[0,1], side:false};
    game[10] = Piece{id:"w_bishop_1".to_string(), _type:4, location:[0,2], side:false};
    game[11] = Piece{id:"w_queen".to_string(), _type:5, location:[0,3], side:false};
    game[12] = Piece{id:"w_king".to_string(), _type:6, location:[0,4], side:false};
    game[13] = Piece{id:"w_bishop_2".to_string(), _type:4, location:[0,5], side:false};
    game[14] = Piece{id:"w_knight_2".to_string(), _type:3, location:[0,6], side:false};
    game[15] = Piece{id:"w_rook_2".to_string(), _type:2, location:[0,7], side:false};

    //black Pieces
    for i in 16..=23{
        game[i] = Piece{id:"b_pawn_".to_owned() + &(i-16).to_string(), _type:1, location:[6,(i-16).try_into().unwrap()], side:true};
    }
    game[24] = Piece{id:"b_rook_1".to_string(), _type:2, location:[7,0], side:true};
    game[25] = Piece{id:"b_knight_1".to_string(), _type:3, location:[7,1], side:true};
    game[26] = Piece{id:"b_bishop_1".to_string(), _type:4, location:[7,2], side:true};
    game[27] = Piece{id:"b_queen".to_string(), _type:5, location:[7,3], side:true};
    game[28] = Piece{id:"b_king".to_string(), _type:6, location:[7,4], side:true};
    game[29] = Piece{id:"b_bishop_2".to_string(), _type:4, location:[7,5], side:true};
    game[30] = Piece{id:"b_knight_2".to_string(), _type:3, location:[7,6], side:true};
    game[31] = Piece{id:"b_rook_2".to_string(), _type:2, location:[7,7], side:true};

    return game;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_test() {
        let result = new_game();
        assert_eq!(result[0].id, "w_pawn_0".to_string());
        assert_eq!(result[8]._type, 2);
        assert_eq!(result[6].location[0], 1);
        assert_eq!(result[20].id, "b_pawn_4".to_string());
        assert_eq!(result[20].location, [6,4]);
        assert_eq!(result[23].id, "b_pawn_7".to_string());
    }
}
