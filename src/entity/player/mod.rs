/*
pub enum Gender {
    Male,
    Female,
    Other
}
*/

use entity::Entity;

#[deriving(Clone)]
pub struct Player;
    //name: ~str,
    //gender: gender
//}

impl Player {
    pub fn new () -> Player {
        Player
    }
}

impl Entity for Player {

}