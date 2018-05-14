use ext::*;
use controls::Action;
use block;
use entity;
use entity::{EntityWrapper, Player, Josef};
use shape::Shape;
use difficulty::Difficulty;
use inventory::InventoryItem;
use move_dir::{MoveDir, random_dir};

use std::collections::HashMap;
use std::mem;
use std::sync::mpsc::Sender;

pub const HOTBAR_HEIGHT: u16 = 5;
pub const SCROLL_FOLLOW_DIST: i16 = 10;

#[derive(Debug)]
pub enum MetaAction {
    Die, Win
}

pub struct World {
    pub blocks: Vec<Vec<block::Block>>,
    pub entities: HashMap<u64, entity::EntityWrapper>,
    pub difficulty: Difficulty,
    auto: Option<MoveDir>,
    last: Option<MoveDir>,
    action_sender: Sender<MetaAction>,
    pub scroll: (i16, i16),
}


impl World {
    pub fn empty(difficulty: Difficulty, action_sender: Sender<MetaAction>) -> World {
        World {
            blocks: vec![],
            entities: HashMap::new(),
            auto: None,
            last: None,
            difficulty: difficulty,
            action_sender: action_sender,
            scroll: (0, 0),
        }
    }

    pub fn tick(&mut self) {
        for k in self.entities.clone().keys() {
            if let Some(f) = self.entities.get(k).map(|x| x.get_tick_fn()) {
                f(self, *k);
            }
        }

        // Automove
        if let Some(auto) = self.auto {
            if !self.move_player_side(&auto) {
                self.auto = None;
                self.last = None;
            }
        }
    }

    pub fn update_scroll(&mut self, size: (u16, u16)) {
        if let Some(id) = self.get_player_id() {
            if let Some(en) = self.entities.get(&id) {
                if  self.scroll.0 > (en.get_pos().0 as i16) - SCROLL_FOLLOW_DIST {
                    self.scroll.0 = (en.get_pos().0 as i16) - SCROLL_FOLLOW_DIST;
                }
                if  self.scroll.1 > (en.get_pos().1 as i16) - SCROLL_FOLLOW_DIST {
                    self.scroll.1 = (en.get_pos().1 as i16) - SCROLL_FOLLOW_DIST;
                }
                if  self.scroll.0 < (en.get_pos().0 as i16) + SCROLL_FOLLOW_DIST - size.0 as i16 - 1 {
                    self.scroll.0 = (en.get_pos().0 as i16) + SCROLL_FOLLOW_DIST - size.0 as i16 - 1;
                }
                if  self.scroll.1 < (en.get_pos().1 as i16) + SCROLL_FOLLOW_DIST - (size.1 - 1 - HOTBAR_HEIGHT) as i16 {
                    self.scroll.1 = (en.get_pos().1 as i16) + SCROLL_FOLLOW_DIST - (size.1 - 1 - HOTBAR_HEIGHT) as i16;
                }
            }
        }
        if self.scroll.0 < 0 {
            self.scroll.0 = 0;
        }
        if self.scroll.1 < 0 {
            self.scroll.1 = 0;
        }
        if self.scroll.0 > self.blocks.len() as i16 - size.0 as i16 {
            self.scroll.0 = self.blocks.len() as i16 - size.0 as i16;
        }
        if self.scroll.1 > self.blocks[0].len() as i16 - size.1 as i16 + HOTBAR_HEIGHT as i16 {
            self.scroll.1 = self.blocks[0].len() as i16 - size.1 as i16 + HOTBAR_HEIGHT as i16;
        }
    }

    pub fn get_player_id(&self) -> Option<u64> {
        for (k, x) in &self.entities {
            if let &entity::EntityWrapper::WPlayer(_) = x {
                return Some(*k);
            }
        }
        None
    }

    pub fn do_metaaction(&mut self, action: MetaAction) {
        self.action_sender.send(action).expect("Can't send!");
    }

    pub fn do_action(&mut self, action: &Action) {
        if let &Action::Die = action {
            self.do_metaaction(MetaAction::Die);
        }

        if let &Action::IncActive = action {
            self.get_player_id()
                .and_then(|id| self.entities.get_mut(&id))
                .map(|en| {
                    if let &mut EntityWrapper::WPlayer(ref mut pl) = en {
                        if pl.active < pl.inventory.len() - 1{
                            pl.active += 1;
                        }
                    }
                });
        }

        if let &Action::DecActive = action {
            self.get_player_id()
                .and_then(|id| self.entities.get_mut(&id))
                .map(|en| {
                    if let &mut EntityWrapper::WPlayer(ref mut pl) = en {
                        if pl.active > 0 {
                            pl.active -= 1;
                        }
                    }
                });
        }

        self.auto = match *action {
            Action::RunDown  => { Some(MoveDir::Down) }
            Action::RunUp    => { Some(MoveDir::Up) }
            Action::RunLeft  => { Some(MoveDir::Left) }
            Action::RunRight => { Some(MoveDir::Right) }
            _                => { None }
        };


        let move_dir: Option<MoveDir> = match *action {
            Action::MoveDown  => { Some(MoveDir::Down) }
            Action::MoveUp    => { Some(MoveDir::Up) }
            Action::MoveLeft  => { Some(MoveDir::Left) }
            Action::MoveRight => { Some(MoveDir::Right) }
            _                 => { None }
        };

        if let Some(x) = move_dir {
            self.auto = None;
            self.get_player_id().map(|id| self.move_entity(id, &x));
            self.last = Some(x);
        }

        let break_dir: Option<MoveDir> = match *action {
            Action::BreakDown  => { Some(MoveDir::Down) }
            Action::BreakUp    => { Some(MoveDir::Up) }
            Action::BreakLeft  => { Some(MoveDir::Left) }
            Action::BreakRight => { Some(MoveDir::Right) }
            _                  => { None }
        };
        if let Some(break_dir) = break_dir {
            self.break_dir(break_dir);
        }

        let place_dir: Option<MoveDir> = match *action {
            Action::PlaceDown  => { Some(MoveDir::Down) }
            Action::PlaceUp    => { Some(MoveDir::Up) }
            Action::PlaceLeft  => { Some(MoveDir::Left) }
            Action::PlaceRight => { Some(MoveDir::Right) }
            _                  => { None }
        };
        if let Some(place_dir) = place_dir {
            self.get_player_id()
                .map(|id| Player::place(self, place_dir, id));
        }
    }

    fn break_dir(&mut self, break_dir: MoveDir) {
        let new_pos;
        if let Some(player) = self.get_player_id().and_then(|id| self.entities.get(&id)) {
            let pl_pos = player.get_pos();
            let (dx, dy) = break_dir.to_vec();

            new_pos = (pl_pos.0 + dx as u16, pl_pos.1 + dy as u16);
        } else {
            return;
        }

        let block_pickup;
        if let Some(block_at) = self.blocks
            .get_mut(new_pos.0 as usize)
            .and_then(|x| x.get_mut(new_pos.1 as usize))
        {
            if block_at.is_breakable() {
                // Break block
                block_pickup = mem::replace(block_at, block::GROUND.clone());
            } else {
                return;
            }
        } else {
            return;
        }

        if let Some(&mut EntityWrapper::WPlayer(ref mut player)) =
            self.get_player_id().and_then(|id| self.entities.get_mut(&id))
        {
            player.pick_up(InventoryItem::Block(block_pickup.clone()));
        }

        self.get_player_id().map(|id| self.move_entity(id, &break_dir));
    }

    fn move_player_side(&mut self, move_dir: &MoveDir) -> bool {
        if self.get_player_id().map(|id| self.move_entity(id, move_dir)) == Some(false) {
            match *move_dir {
                MoveDir::Up | MoveDir::Down => {
                    let mut d1 = MoveDir::Left;
                    let mut d2 = MoveDir::Right;
                    if let Some(last) = self.last {
                        if last == d2 {
                            d1 = MoveDir::Right;
                            d2 = MoveDir::Left;
                        }
                    } else if rand() < 0.5 {
                        d1 = MoveDir::Right;
                        d2 = MoveDir::Left;
                    }

                    self.last = Some(d1);
                    if self.get_player_id()
                        .map(|id| self.move_entity(id, &d1))
                            == Some(false) {
                        if self.get_player_id()
                            .map(|id| self.move_entity(id, &d2))
                                == Some(false) {
                            return false;
                        }
                    }
                }
                MoveDir::Left | MoveDir::Right => {
                    let mut d1 = MoveDir::Up;
                    let mut d2 = MoveDir::Down;
                    if let Some(last) = self.last {
                        if last == d2 {
                            d1 = MoveDir::Down;
                            d2 = MoveDir::Right;
                        }
                    } else if rand() < 0.5 {
                        d1 = MoveDir::Down;
                        d2 = MoveDir::Up;
                    }

                    self.last = Some(d1);
                    if self.get_player_id()
                        .map(|id| self.move_entity(id, &d1))
                            == Some(false) {
                        if self.get_player_id()
                            .map(|id| self.move_entity(id, &d2))
                                == Some(false) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }


    fn move_entity(&mut self, en_id: u64, move_dir: &MoveDir) -> bool {

        self.last = Some(*move_dir);
        if let Some(en_move_fn) = self.entities.get(&en_id).map(|x| x.get_move_fn()) {
            en_move_fn(self, en_id, *move_dir)
        } else {
            false
        }

    }

    pub fn draw(&self, size: (u16, u16)) {

        // Draw world
        for x in 0..size.0 {
            for y in 0..size.1 - HOTBAR_HEIGHT {
                if let (Some(x_), Some(y_)) =
                    ((x as i16).checked_add(self.scroll.0), (y as i16).checked_add(self.scroll.1))
                {
                    if let Some(block) = self.blocks.get(x_ as usize)
                        .and_then(|col| col.get(y_ as usize))
                    {
                        block.get_shape().draw((x, y));
                    } else {
                        put_char((x as u16, y as u16), &Shape::empty());
                    }
                }
            }
        }

        // Clear hotbar
        for x in 0..size.0 {
            for y in size.1 - HOTBAR_HEIGHT..size.1 {
                put_char((x as u16, y as u16), &Shape::empty());
            }
        }

        // Draw entities
        self.entities.iter()
            .for_each(|(_, x)| x.pre_draw(self, &size, &self.scroll));

        self.entities.iter()
            .for_each(|(_, en)| {
                let (x, y) = en.get_pos();
                if let (Some(x_), Some(y_)) =
                    ((x as i16).checked_sub(self.scroll.0), (y as i16).checked_sub(self.scroll.1))
                {
                    if x_ >= 0 && x_ < size.0 as i16 && y_ >= 0 && y_ < size.1 as i16 - HOTBAR_HEIGHT as i16 {
                        en.get_shape().draw((x_ as u16, y_ as u16));
                    }
                }
            }
            );
    }

    pub fn generate(&mut self, width: usize, height: usize) {
        log("Generating!");

        self.blocks = vec![];
        self.auto = None;

        for x in 0..width {
            self.blocks.push(vec![]);
            for _ in 0..height {
                if rand() > 0.1 {
                    self.blocks[x].push(block::WALL.clone());
                } else {
                    self.blocks[x].push(block::STONE.clone());
                }
            }
        }

        self.entities = HashMap::new();

        let mut placed = vec![];
        for _ in 0..10 * width * height {
            if rand() < 0.01 || placed.is_empty() {
                let x = (rand() * width as f64) as usize;
                let y = (rand() * height as f64) as usize;
                self.blocks[x][y] = block::GROUND.clone();
                placed.push((x, y, random_dir()));
            } else {
                let idx = (rand() * placed.len() as f64) as usize;
                let (x, y, mut dir) = placed[idx];

                if rand() < 0.05 {
                    dir = random_dir();
                }

                let dirv = dir.to_vec();

                let (nx, ny) = (x + dirv.0 as usize, y + dirv.1 as usize);

                let block_at = self.blocks.get(nx).and_then(|x| x.get(ny));
                if block_at == Some(&&*block::WALL) || block_at == Some(&&*block::WALL){
                    self.blocks[nx][ny] = block::GROUND.clone();
                    placed.push((nx, ny, dir));
                }
            }
        }

        let idx = (rand() * placed.len() as f64) as usize;
        let (x, y, _) = placed[idx];
        placed.remove(idx);
        self.add_entity(
            EntityWrapper::WPlayer(
                Player::new((x as u16, y as u16), self.difficulty.get_start_health())
                )
            );

        let idx = (rand() * placed.len() as f64) as usize;
        let (x, y, _) = placed[idx];
        placed.remove(idx);
        self.add_entity(
            EntityWrapper::WJosef(
                Josef::new((x as u16, y as u16), self.difficulty.get_josef_speed())
            ));

        log("Done!");
    }

    pub fn add_entity(&mut self, entity: EntityWrapper) {
        loop {
            let key = (rand() * <u64>::max_value() as f64) as u64;
            if !self.entities.contains_key(&key) {
                self.entities.insert(key, entity);
                break;
            }
        }
    }
}

