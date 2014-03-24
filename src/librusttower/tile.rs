use std::cell::RefCell;
use std::rc::Rc;

use entity::player::Player;
use feature::Feature;

pub struct Tile {
    entity: RefCell<Option<Rc<RefCell<Player>>>>,
    feature: RefCell<~Feature>
}

impl Tile {
    pub fn new () -> Tile {
        use std::cell::RefCell;

        Tile {
            entity: RefCell::new(None),
            feature: RefCell::new(~::feature::EmptySpace as ~Feature)
        }
    }

    pub fn is_passable (&self, player: &Player) -> bool {
        self.feature.borrow().is_passable(player)
    }

    pub fn set_feature (&self, feature: ~::feature::Feature) {
        self.feature.set(feature);
    }

    pub fn remove_entity (&self) {
        self.entity.set(None);
    }

    /*
    pub fn add_entity (&self, entity: Rc<RefCell<super::entity::player::Player>>) {
        if self.entity.get().is_some() {
            fail!("Tried to add entity on top of entity.");
        }

        self.entity.set(Some(entity));
    }
    */
}