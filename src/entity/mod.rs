use std::ops::{Deref, DerefMut};

use level::Level;
use shape::Shape;
use block;
use move_dir::MoveDir;

mod player;
mod josef;
mod police;
mod bomb;
mod bullet;
pub use self::player::*;
pub use self::josef::*;
pub use self::police::*;
pub use self::bomb::*;
pub use self::bullet::*;

pub trait Entity {

    fn tick(_level: &mut Level, _en_id: u64) where Self: Sized { }

    fn get_pos(&self) -> (u16, u16);
    fn get_pos_mut(&mut self) -> &mut (u16, u16);

    fn hurt(_level: &mut Level, _en_id: u64, _amount: u16) where Self: Sized {}

    /// Try to move an entity in a specific direction
    /// Returns if the entity successfully moved
    fn move_dir(level: &mut Level, en_id: u64, dir: MoveDir) -> bool
        where Self: Sized {

        let (dx, dy) = dir.to_vec();

        let mut new_pos_and_dir: Option<((u16, u16), (i8, i8))> = None;

        if let Some(en) = level.entities.get_mut(&en_id) {
            en.get_pos_mut().0 = en.get_pos_mut().0.wrapping_add(dx as u16);
            en.get_pos_mut().1 = en.get_pos_mut().1.wrapping_add(dy as u16);

            new_pos_and_dir = Some((en.get_pos().clone(), (dx, dy)));
        }


        if let Some((pos, dir)) = new_pos_and_dir {
            let passable = level.blocks.get(pos.0 as usize)
                        .and_then(|x| x.get(pos.1 as usize))
                        .map(|x| x.is_passable())
                        .unwrap_or(false);

            if passable {
                let id = level.blocks.get(pos.0 as usize)
                    .and_then(|x| x.get(pos.1 as usize))
                    .map(|x| x.get_id())
                    .unwrap_or(0);

                let blkf = block::BLOCK_FUNCS.lock().unwrap();

                match blkf.get(id) {
                    Some(f) => {
                        f(level, en_id);
                    }
                    None => {}
                }
            } else {
                if let Some(en) = level.entities.get_mut(&en_id) {
                    en.get_pos_mut().0 = en.get_pos_mut().0.wrapping_sub(dx as u16);
                    en.get_pos_mut().1 = en.get_pos_mut().1.wrapping_sub(dy as u16);
                }
                return false;
            }
            for k in level.entities.clone().keys() {
                if k != &en_id && level.entities.get(k).map(|x| x.get_pos()) == Some(pos) {
                    let mut collided = false;

                    let f = level.entities.get(k).unwrap().get_collision_fn();

                    if !f(level, *k, en_id) {
                        if let Some(en) = level.entities.get_mut(&en_id) {
                            en.get_pos_mut().0 -= dir.0 as u16;
                            en.get_pos_mut().1 -= dir.1 as u16;
                            collided = true;
                        }
                    }


                    if let Some(f) = level.entities.get(&en_id).map(|x| x.get_collision_fn()) {
                        if !f(level, en_id, *k) {
                            if let Some(en) = level.entities.get_mut(&en_id) {
                                if !collided {
                                    en.get_pos_mut().0 -= dir.0 as u16;
                                    en.get_pos_mut().1 -= dir.1 as u16;
                                }
                                collided = true;
                            }
                        }
                    }
                    if collided {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn get_shape(&self) -> Shape;
    fn get_name(&self) -> String;

    fn pre_draw(&self, _level: &Level, _size: &(u16, u16), _scroll: &(i16, i16)) {
    }

    /// When another entity moves on top of this entity, what should happen?
    /// Returns if entity is passable
    fn on_collision(_level: &mut Level, _me_id: u64, _other_id: u64) -> bool
        where Self: Sized {
        false
    }
}

macro_rules! MakeEntityWrapper {
    ( $($name:ident = $wname:ident),+ ) => {

        #[derive(PartialEq, Eq, Clone)]
        pub enum EntityWrapper {
            $($wname($name)),+
        }

        impl EntityWrapper {
            pub fn get_tick_fn(&self) -> impl Fn(&mut Level, u64) {
                use self::EntityWrapper::*;
                match *self {
                    $( $wname(_) => $name::tick ),+
                }
            }

            pub fn get_move_fn(&self) -> impl Fn(&mut Level, u64, MoveDir) -> bool {
                use self::EntityWrapper::*;
                match *self {
                    $( $wname(_) => $name::move_dir ),+
                }
            }

            pub fn get_collision_fn(&self) -> impl Fn(&mut Level, u64, u64) -> bool {
                use self::EntityWrapper::*;
                match *self {
                    $( $wname(_) => $name::on_collision ),+
                }
            }
            pub fn get_hurt_fn(&self) -> impl Fn(&mut Level, u64, u16) {
                use self::EntityWrapper::*;
                match *self {
                    $( $wname(_) => $name::hurt ),+
                }
            }
        }

        impl Deref for EntityWrapper {
            type Target = Entity;

            fn deref(&self) -> &Self::Target {
                use self::EntityWrapper::*;

                match *self {
                    $( $wname(ref e) => e ),+
                }
            }
        }

        impl DerefMut for EntityWrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                use self::EntityWrapper::*;

                match *self {
                    $( $wname(ref mut e) => e ),+
                }
            }
        }
    }
}

MakeEntityWrapper!(
    Player=WPlayer,
    Josef=WJosef,
    Police=WPolice,
    Bomb=WBomb,
    Bullet=WBullet
    );

