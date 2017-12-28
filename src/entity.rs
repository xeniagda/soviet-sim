use world::{MoveDir, World};
use shape::Shape;
use ext::*;
use block;

use std::ops::{Deref, DerefMut};

const SHOW_PATH_FINDING: bool = true;

pub trait Entity {

    fn tick(_world: &mut World, _en_id: u64) where Self: Sized { }

    fn get_pos(&self) -> (u16, u16);
    fn get_pos_mut(&mut self) -> &mut (u16, u16);

    /// Try to move an entity in a specific direction
    /// Returns if the entity collided and had to move back
    fn move_dir(world: &mut World, en_id: u64, dir: MoveDir) -> bool
        where Self: Sized {

        let (dx, dy) = dir.to_vec();

        let mut new_pos_and_dir: Option<((u16, u16), (i8, i8))> = None;

        if let Some(en) = world.entities.get_mut(&en_id) {
            en.get_pos_mut().0 += dx as u16;
            en.get_pos_mut().1 += dy as u16;

            new_pos_and_dir = Some((en.get_pos().clone(), (dx, dy)));
        }


        if let Some((pos, dir)) = new_pos_and_dir {
            // log(&format!("Moved to {:?} in {:?}", pos, dir));

            let id = world.blocks.get(pos.0 as usize)
                        .and_then(|x| x.get(pos.1 as usize))
                        .map(|x| x.get_id())
                        .unwrap_or(0);

            let mut blkf = block::BLOCK_FUNCS.lock().unwrap();

            // log(&format!("Id: {}", id));

            match blkf.get(id) {
                Some(f) => {
                    if !f(world, Some(en_id)) {
                        // log("Moving back");
                        if let Some(en) = world.entities.get_mut(&en_id) {
                            en.get_pos_mut().0 -= dir.0 as u16;
                            en.get_pos_mut().1 -= dir.1 as u16;
                            return true;
                        }
                    }
                }
                None => {}
            }

            for k in world.entities.clone().keys() {
                if k != &en_id && world.entities.get(k).map(|x| x.get_pos()) == Some(pos) {
                    let mut collided = false;

                    let f = world.entities.get(k).unwrap().get_collision_fn();

                    if f(world, *k, en_id) {
                        // log("Entity collision");
                        if let Some(en) = world.entities.get_mut(&en_id) {
                            en.get_pos_mut().0 -= dir.0 as u16;
                            en.get_pos_mut().1 -= dir.1 as u16;
                            collided = true;
                        }
                    }


                    if let Some(f) = world.entities.get(&en_id).map(|x| x.get_collision_fn()) {
                        if f(world, en_id, *k) {
                            // log("Self collision");
                            if let Some(en) = world.entities.get_mut(&en_id) {
                                en.get_pos_mut().0 -= dir.0 as u16;
                                en.get_pos_mut().1 -= dir.1 as u16;
                                collided = true;
                            }
                        }
                    }
                    if collided {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_shape(&self) -> Shape;

    fn pre_draw(&self, _world: &World) {
    }

    /// When another entity moves on top of this entity, what should happen?
    /// Returns if the entity have to move back
    fn on_collision(_world: &mut World, _me_id: u64, _other_id: u64) -> bool
        where Self: Sized {
        true
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub pos: (u16, u16),
}

impl Entity for Player {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { Shape { ch: '@', col: (0, 255, 0), bg: (0, 0, 0) } }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Josef {
    pub countdown: u16,
    pub speed: u16,
    pub path: Vec<MoveDir>,
    pub visited: Vec<(u16, u16)>,
    pub pos: (u16, u16),
}

impl Entity for Josef {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }
    fn get_shape(&self) -> Shape { Shape { ch: 'J', col: (255, 0, 0), bg: (0, 0, 0) } }


    fn tick(world: &mut World, en_id: u64) where Self: Sized {
        let should_walk = {
            match world.entities.get_mut(&en_id) {
                Some(&mut EntityWrapper::WJosef(ref mut this)) => {
                    if this.countdown == 0 {
                        this.countdown = this.speed;
                        true
                    } else {
                        this.countdown -= 1;
                        false
                    }
                }
                _ => false
            }
        };

        if should_walk {
            let (player_pos, my_pos);
            if let Some(player) = world.get_player_id().and_then(|x| world.entities.get(&x)) {
                if let Some(this) = world.entities.get(&en_id) {
                    player_pos = player.get_pos();
                    my_pos = this.get_pos();
                } else {
                    return;
                }
            } else {
                return;
            }

            let mut visited = vec![ my_pos ];
            let mut paths: Vec<(_, Vec<MoveDir>)> = vec![ (my_pos, vec![]) ];

            let mut best_path = None;

            // Find closest path to player
            'outer: loop {
                if let Some((ref pos, ref path)) = paths.clone().into_iter()
                        .min_by_key(|&(ref pos, ref path)| {
                            let delta = (pos.0 - player_pos.0, pos.1 - player_pos.1);
                            delta.0 * delta.0 + delta.1 * delta.1 + path.len() as u16 * 2
                        }) {
                    paths.remove_item(&(*pos, path.clone()));

                    let mut dirs = vec! [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];
                    for _ in 0..4 {
                        let fidx = rand() * dirs.len() as f64;
                        let dir = dirs[fidx as usize];
                        dirs.remove(fidx as usize);

                        let (dx, dy) = dir.to_vec();
                        let new_pos = (pos.0 + dx as u16, pos.1 + dy as u16);

                        let mut new_path = path.clone();
                        new_path.push(dir);

                        if visited.contains(&new_pos) {
                            continue;
                        }
                        if new_pos == player_pos {
                            best_path = Some(new_path);
                            break 'outer;
                        }

                        let id = world.blocks.get(new_pos.0 as usize)
                            .and_then(|x| x.get(new_pos.1 as usize))
                            .map(|x| x.get_id())
                            .unwrap_or(0);

                        let mut blkf = block::BLOCK_FUNCS.lock().unwrap();

                        // log(&format!("Id: {}", id));

                        match blkf.get(id) {
                            Some(f) => {
                                if f(world, None) {
                                    paths.push((new_pos, new_path));
                                    visited.push(new_pos);
                                }
                            }
                            None => {}
                        }
                    }
                } else {
                    break 'outer;
                }
            }

            if let Some(best_path) = best_path.clone() {
                Josef::move_dir(world, en_id, best_path[0]);
            }
            if let Some(&mut EntityWrapper::WJosef(ref mut this)) = world.entities.get_mut(&en_id) {
                this.path = best_path.unwrap_or(vec![]);
                this.visited = visited;
            }
        }
    }

    fn on_collision(world: &mut World, _me_id: u64, _other_id: u64) -> bool
        where Self: Sized {

        let (w, h) = (world.blocks.len(), world.blocks[0].len());
        world.generate(w, h);

        true
    }

    fn pre_draw(&self, _world: &World) {
        if SHOW_PATH_FINDING {
            let mut pos = self.get_pos();

            for pos in &self.visited {
                recolor(*pos, (0, 0, 0), (0, 128, 0));
            }

            for dir in self.path.iter().skip(1) {
                let (dx, dy) = dir.to_vec();
                pos = (pos.0 + dx as u16, pos.1 + dy as u16);

                recolor(pos, (0, 0, 0), (128, 0, 0));
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum EntityWrapper {
    WPlayer(Player),
    WJosef(Josef)
}

impl EntityWrapper {
    pub fn get_tick_fn(&self) -> impl Fn(&mut World, u64) {
        use self::EntityWrapper::*;
        match *self {
            WPlayer(_) => Player::tick,
            WJosef(_) => Josef::tick,
        }
    }

    pub fn get_move_fn(&self) -> impl Fn(&mut World, u64, MoveDir) -> bool {
        use self::EntityWrapper::*;
        match *self {
            WPlayer(_) => Player::move_dir,
            WJosef(_) => Josef::move_dir,
        }
    }

    pub fn get_collision_fn(&self) -> impl Fn(&mut World, u64, u64) -> bool {
        use self::EntityWrapper::*;
        match *self {
            WPlayer(_) => Player::on_collision,
            WJosef(_) => Josef::on_collision,
        }
    }
}

impl Deref for EntityWrapper {
    type Target = Entity;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        use self::EntityWrapper::*;

        match *self {
            WPlayer(ref e) => e,
            WJosef(ref e) => e,
        }
    }
}

impl DerefMut for EntityWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        use self::EntityWrapper::*;

        match *self {
            WPlayer(ref mut e) => e,
            WJosef(ref mut e) => e,
        }
    }
}
