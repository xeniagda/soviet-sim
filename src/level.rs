use std::f64;

use ext::*;
use world::{World, Callback};
use controls::Action;
use block;
use entity;
use entity::{EntityWrapper, Player, Josef};
use shape::Shape;
use difficulty::Difficulty;
use inventory::InventoryItem;
use move_dir::{MoveDir, random_dir, DIRECTIONS};

use std::collections::HashMap;
use std::mem;
use std::sync::mpsc::{Sender, SendError};

pub const HOTBAR_HEIGHT: u16 = 5;
pub const SCROLL_FOLLOW_DIST: i16 = 10;

#[derive(Debug)]
pub enum MetaAction {
    Die, Win
}

pub struct Level {
    pub blocks: Vec<Vec<block::Block>>,
    pub entities: HashMap<u64, entity::EntityWrapper>,
    pub difficulty: Difficulty,
    pub auto_walk: Vec<MoveDir>,
    pub auto_mine: Vec<MoveDir>,
    action_sender: Sender<MetaAction>,
    pub callback_sender: Sender<Callback>,
    pub scroll: (i16, i16),
    ticks_since_active: u64
}


impl Level {
    pub fn empty(difficulty: Difficulty, action_sender: Sender<MetaAction>, callback_sender: Sender<Callback>) -> Level {
        Level {
            blocks: vec![],
            entities: HashMap::new(),
            difficulty: difficulty,
            auto_walk: vec![],
            auto_mine: vec![],
            action_sender: action_sender,
            callback_sender: callback_sender,
            scroll: (0, 0),
            ticks_since_active: 0
        }
    }

    pub fn on_activated(&mut self) {
        self.ticks_since_active = 0;
    }

    pub fn tick(&mut self) {
        self.ticks_since_active += 1;
        for k in self.entities.clone().keys() {
            if let Some(f) = self.entities.get(k).map(|x| x.get_tick_fn()) {
                f(self, *k);
            }
        }

        if !self.auto_walk.is_empty() {
            if let Some(EntityWrapper::WPlayer(ref mut p)) =
                self.get_player_id().and_then(|id| self.entities.get_mut(&id))
            {
                let mut to_remove = None;
                if let Some((i, (InventoryItem::SuperBoots(ref mut d, max), ref mut count))) =
                    p.inventory.iter_mut()
                    .enumerate()
                    .find(|x| match (x.1).0 { InventoryItem::SuperBoots(_, _) => true, _ => false })
                {
                    *d -= 1;
                    if *d == 0 {
                        *count -= 1;
                        *d = *max;
                        if *count == 0 {
                            to_remove = Some(i);
                        }
                    }
                } else {
                    self.auto_walk.clear();
                    return;
                }
                if let Some(to_remove) = to_remove {
                    p.inventory.remove(to_remove);
                }
            }

            let dir = self.auto_walk.remove(0);
            self.get_player_id().map(|id| self.move_entity(id, dir));
        }
        if !self.auto_mine.is_empty() {
            if let Some(EntityWrapper::WPlayer(ref mut p)) =
                self.get_player_id().and_then(|id| self.entities.get_mut(&id))
            {
                let mut to_remove = None;
                if let Some((i, (InventoryItem::Pickaxe(ref mut d, max), ref mut count))) =
                    p.inventory.iter_mut()
                    .enumerate()
                    .find(|x| match (x.1).0 { InventoryItem::Pickaxe(_, _) => true, _ => false })
                {
                    *d -= 1;
                    if *d == 0 {
                        *count -= 1;
                        *d = *max;
                        if *count == 0 {
                            to_remove = Some(i);
                        }
                    }
                } else {
                    self.auto_mine.clear();
                    return;
                }
                if let Some(to_remove) = to_remove {
                    p.inventory.remove(to_remove);
                }
            }

            let dir = self.auto_mine.remove(0);
            self.break_dir(dir);
        }
    }

    pub fn update_scroll(&mut self, size: (u16, u16)) {
        if size.0 < self.blocks.len() as u16 {
            if let Some(id) = self.get_player_id() {
                if let Some(en) = self.entities.get(&id) {
                    if  self.scroll.0 > (en.get_pos().0 as i16) - SCROLL_FOLLOW_DIST {
                        self.scroll.0 = (en.get_pos().0 as i16) - SCROLL_FOLLOW_DIST;
                    }
                    if  self.scroll.0 < (en.get_pos().0 as i16) + SCROLL_FOLLOW_DIST - size.0 as i16 - 1 {
                        self.scroll.0 = (en.get_pos().0 as i16) + SCROLL_FOLLOW_DIST - size.0 as i16 - 1;
                    }
                }
            }
            if  self.scroll.0 < 0 {
                self.scroll.0 = 0;
            }
            if  self.scroll.0 > self.blocks.len() as i16 - size.0 as i16 {
                self.scroll.0 = self.blocks.len() as i16 - size.0 as i16;
            }
        } else {
            self.scroll.0 = (self.blocks.len() as i16 - size.0 as i16) / 2;
        }

        if size.1 < self.blocks[0].len() as u16 + HOTBAR_HEIGHT {
            if let Some(id) = self.get_player_id() {
                if let Some(en) = self.entities.get(&id) {
                    if  self.scroll.1 > (en.get_pos().1 as i16) - SCROLL_FOLLOW_DIST {
                        self.scroll.1 = (en.get_pos().1 as i16) - SCROLL_FOLLOW_DIST;
                    }
                    if  self.scroll.1 < (en.get_pos().1 as i16) + SCROLL_FOLLOW_DIST - (size.1 - 1 - HOTBAR_HEIGHT) as i16 {
                        self.scroll.1 = (en.get_pos().1 as i16) + SCROLL_FOLLOW_DIST - (size.1 - 1 - HOTBAR_HEIGHT) as i16;
                    }
                }
            }
            if  self.scroll.1 < 0 {
                self.scroll.1 = 0;
            }
            if  self.scroll.1 > self.blocks[0].len() as i16 - size.1 as i16 + HOTBAR_HEIGHT as i16 {
                self.scroll.1 = self.blocks[0].len() as i16 - size.1 as i16 + HOTBAR_HEIGHT as i16;
            }
        } else {
            self.scroll.1 = (self.blocks[0].len() as i16 + HOTBAR_HEIGHT as i16 - size.1 as i16) / 2;
        }
    }

    pub fn get_player_id(&self) -> Option<u64> {
        self.entities
            .iter()
            .find(|(_, en)|
                   if let EntityWrapper::WPlayer(_) = en { true }
                   else { false }
                   )
            .map(|(id, _)| *id)
    }

    pub fn do_metaaction(&mut self, action: MetaAction) {
        self.action_sender.send(action).expect("Can't send!");
    }

    pub fn do_action(&mut self, action: &Action) {
        match *action {
            Action::Move(dir) => {
                self.get_player_id().map(|id| self.move_entity(id, dir));
                self.auto_walk = vec![];
                self.auto_mine = vec![];
            }
            Action::Break(dir)  => {
                self.break_dir(dir);
                self.auto_walk = vec![];
                self.auto_mine = vec![];
            }
            Action::SuperMine(dir) => {
                if let Some(EntityWrapper::WPlayer(p)) = self.get_player_id().and_then(|id| self.entities.get(&id)) {
                    let start_pos = dir.move_vec(p.pos);

                    let heur = |_| {
                        Some(rand())
                    };
                    self.auto_mine = self.find_path(
                        start_pos,
                        |block, _|
                            if block.is_passable()
                                { None }
                                else { Some(-1.) },
                        heur,
                        1000,
                        true)
                        .into_iter()
                        .collect();

                    self.break_dir(dir);
                }
            }
            Action::Place(dir)  => {
                self.get_player_id() .map(|id| Player::place(self, dir, id));
                self.auto_walk = vec![];
                self.auto_mine = vec![];
            }
            Action::Die => {
                self.do_metaaction(MetaAction::Die);
            }
            Action::IncActive => {
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
            Action::Run(dir) => {
                if let Some(EntityWrapper::WPlayer(p)) = self.get_player_id().and_then(|id| self.entities.get(&id)) {
                    let pos = p.pos;
                    let heur = |(x, y)| {
                        let score = match dir {
                            MoveDir::Left =>  pos.0 as f64 - x as f64,
                            MoveDir::Right => x as f64 - pos.0 as f64,
                            MoveDir::Up =>    pos.1 as f64 - y as f64,
                            MoveDir::Down =>  y as f64 - pos.1 as f64,
                        };
                        Some(-score * 3.)
                    };
                    self.auto_walk = self.find_path(
                        pos,
                        |block, _|
                            if block.is_passable()
                                { Some(1.) }
                                else { None },
                        heur,
                        1000,
                        true)
                        .into_iter()
                        .take(20)
                        .collect();
                }
            }
            Action::DecActive => {
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
            _ => {}
        };

    }

    fn break_dir(&mut self, break_dir: MoveDir) {
        let new_pos;
        if let Some(player) = self.get_player_id().and_then(|id| self.entities.get(&id)) {
            let pl_pos = player.get_pos();
            let (dx, dy) = break_dir.to_vec();

            new_pos = (pl_pos.0.wrapping_add(dx as u16), pl_pos.1.wrapping_add(dy as u16));
        } else {
            return;
        }

        let block_pickup = if let Some(block_at) = self.get_at_mut(new_pos) {
                if block_at.is_breakable() == block::Breakability::Breakable {
                    // Break block
                    mem::replace(block_at, block::GROUND.clone())
                } else {
                    return;
                }
            } else {
                return;
            };

        if let Some(EntityWrapper::WPlayer(ref mut player)) =
            self.get_player_id().and_then(|x| self.entities.get_mut(&x))
        {
            player.pick_up(InventoryItem::Block(block_pickup));
        }

        self.get_player_id().map(|id| self.move_entity(id, break_dir));
    }

    fn move_entity(&mut self, en_id: u64, move_dir: MoveDir) -> bool {
        if let Some(en) = self.entities.get(&en_id) {
            en.get_move_fn()(self, en_id, move_dir)
        } else {
            false
        }
    }

    pub fn draw(&self, size: (u16, u16)) {
        // Draw level
        for x in 0..size.0 {
            for y in 0..size.1 - HOTBAR_HEIGHT {
                if let (Some(x_), Some(y_)) =
                    ((x as i16).checked_add(self.scroll.0), (y as i16).checked_add(self.scroll.1))
                {
                    if let Some(block) = self.get_at((x_ as u16, y_ as u16)) {
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

        // Apply filter
        if let Some((pl_x, pl_y)) = self.get_player_id()
            .and_then(|id| self.entities.get(&id)).map(|pl| pl.get_pos())
        {
            if let (Some(screen_x), Some(screen_y)) =
                ((pl_x as i16).checked_sub(self.scroll.0), (pl_y as i16).checked_sub(self.scroll.1))
            {
                for x in 0..size.0 {
                    for y in 0..size.1 {
                        if let Some(shp) = get_shape((x, y)) {
                            let (dx, dy) = (x as f64 - screen_x as f64, y as f64 - screen_y as f64);
                            let dist_sq = dx * dx + dy * dy;

                            let norm_dist_sq = dist_sq as f64 / (size.0 * size.0 + size.1 * size.1) as f64;
                            let norm_dist = norm_dist_sq.sqrt();

                            let decay = (self.ticks_since_active * self.ticks_since_active + 1) as f64 / 600.;

                            let fade_mul = 1. - norm_dist / decay;

                            let fade_mul = fade_mul.max(0.).sqrt().sqrt().sqrt().min(1.);

                            let fg = ((shp.col.0 as f64 * fade_mul) as u8,
                                      (shp.col.1 as f64 * fade_mul) as u8,
                                      (shp.col.2 as f64 * fade_mul) as u8,
                                     );
                            let bg = ((shp.bg.0 as f64 * fade_mul) as u8,
                                      (shp.bg.1 as f64 * fade_mul) as u8,
                                      (shp.bg.2 as f64 * fade_mul) as u8,
                                     );

                            let new_shape = Shape { ch: shp.ch, col: fg, bg: bg };
                            put_char((x, y), &new_shape);
                        }
                    }
                }
            }
        }
        // Draw entities
        self.entities.iter()
            .for_each(|(_, x)| x.pre_draw(self, &size, &self.scroll));

    }

    pub fn generate(&mut self, settings: GenerationSettings) {
        log("Generating!");

        self.entities = HashMap::new();
        self.blocks = vec![];

        let total: f64 = settings.block_probs.values().sum();

        for x in 0..settings.width {
            self.blocks.push(vec![]);
            for _ in 0..settings.height {
                let block_num = rand() * total;

                let mut upto = 0.;

                for (blk, prob) in settings.block_probs.iter() {
                    upto += *prob;
                    if upto >= block_num {
                        self.blocks[x].push(blk.clone());
                        break;
                    }
                }

            }
        }

        let mut placed: Vec<(u16, u16, MoveDir)> = vec![];
        for _ in 0..(settings.amount_of_walls * settings.width as f64 * settings.height as f64) as usize {
            if rand() < settings.new_pos_prob || placed.is_empty() {
                let x = (rand() * settings.width as f64) as u16;
                let y = (rand() * settings.height as f64) as u16;
                self.blocks[x as usize][y as usize] = block::GROUND.clone();
                placed.push((x, y, random_dir()));
            } else {
                let idx = (rand() * placed.len() as f64) as usize;
                let (x, y, mut dir) = placed[idx];

                if rand() < settings.new_dir_prob {
                    dir = random_dir();
                }

                let dirv = dir.to_vec();

                let (nx, ny) = (x.wrapping_add(dirv.0 as u16), y.wrapping_add(dirv.1 as u16));

                let block_at = self.get_at((nx, ny));
                if block_at.map(|x| x.breakable == block::Breakability::Breakable).unwrap_or(false) {
                    self.blocks[nx as usize][ny as usize] = block::GROUND.clone();
                    placed.push((nx, ny, dir));
                }
            }
        }

        for mut en in settings.entities.into_iter() {
            let idx = (rand() * placed.len() as f64) as usize;
            let (x, y, _) = placed[idx];
            placed.remove(idx);
            *en.get_pos_mut() = (x as u16, y as u16);
            self.add_entity(
                en
                );
        }
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

    pub fn get_at(&self, pos: (u16, u16)) -> Option<&block::Block> {
        self.blocks.get(pos.0 as usize).and_then(|col| col.get(pos.1 as usize))
    }

    pub fn get_at_mut(&mut self, pos: (u16, u16)) -> Option<&mut block::Block> {
        self.blocks.get_mut(pos.0 as usize).and_then(|col| col.get_mut(pos.1 as usize))
    }

    /// Find a path using a cost and heuristics function.
    /// Cost: Takes a block and returns the cost of passing that block. If None is returned, the
    /// block is considered not passable. Large here is considered bad.
    /// Heuristics: Takes a level and position and gives back a heuristics of going to that
    /// position. If None, that position is the goal. Large here is considered dab.
    pub fn find_path(
        &self,
        from: (u16, u16),
        cost: impl Fn(block::Block, (u16, u16)) -> Option<f64>,
        heuristics: impl Fn((u16, u16)) -> Option<f64>,
        steps: u16,
        allow_incomplete: bool
        ) ->
        Vec<MoveDir>
    {
        let mut visited: Vec<(u16, u16)> = vec![];
        let mut to_visit: Vec<(u16, u16)> = vec![from]; // The last value has the lowest cost_heur

        let mut camefrom: HashMap<(u16, u16), MoveDir> = HashMap::new();
        let mut costs: HashMap<(u16, u16), f64> = HashMap::new();
        costs.insert(from, 0.);

        let mut costs_heur: HashMap<(u16, u16), Option<f64>> = HashMap::new();
        costs_heur.insert(from, heuristics(from));

        let mut count = 0;

        let mut min_heur: Option<((u16, u16), f64)> = None;

        while !to_visit.is_empty() && count < steps {
            let curr = to_visit.pop().unwrap();
            visited.push(curr);

            if costs_heur.get(&curr) == Some(&None) {
                // Reconstruct path

                let mut curr = curr;
                let mut total = vec![];
                while let Some(dir) = camefrom.get(&curr) {
                    total.insert(0, *dir);
                    curr = dir.rotate_180().move_vec(curr);
                }
                return total;
            }

            let mut dirs = DIRECTIONS.to_vec();
            for i in 0..4 {
                let idx = (rand() * (4. - i as f64)) as usize;
                let dir = dirs[idx];
                dirs.swap(idx, 3 - i);

                let moved = dir.move_vec(curr);

                let block = self.get_at(moved);
                if block.is_none() { continue; }

                let block = block.unwrap();

                if visited.contains(&moved) { continue; }

                let curr_cost = cost(block.clone(), moved);
                if curr_cost.is_none() {
                    continue;
                }
                let tot_cost = curr_cost.unwrap() + *costs.get(&curr).unwrap_or(&f64::MAX);

                if &tot_cost >= costs.get(&moved).unwrap_or(&f64::MAX) { continue; }

                camefrom.insert(moved, dir);

                costs.insert(moved, tot_cost);

                let heur = heuristics(moved);
                if let Some(heur) = heur {
                    match min_heur {
                        Some((_pos, m_heur)) => {
                            if m_heur > heur {
                                min_heur = Some((moved, heur));
                            }
                        }
                        None => {
                            min_heur = Some((moved, heur));
                        }
                    }
                }

                costs_heur.insert(moved, heur.map(|x| x + tot_cost));

                if !to_visit.contains(&moved) {

                    let moved_hcost = costs_heur.get(&moved).unwrap_or(&Some(f64::MAX));

                    if moved_hcost.is_none() {
                        to_visit.push(moved);
                        continue;
                    }

                    let moved_hcost = moved_hcost.unwrap();

                    let mut inserted = false;
                    for i in 0..to_visit.len() {
                        match costs_heur.get(&to_visit[i]).unwrap_or(&Some(f64::MAX)) {
                            Some(x) => {
                                if x < &moved_hcost {
                                    to_visit.insert(i, moved);
                                    inserted = true;
                                    break;
                                }
                            }
                            None => {
                                to_visit.insert(i, moved);
                                inserted = true;
                                break;
                            }
                        }
                    }
                    if !inserted {
                        to_visit.push(moved);
                    }
                }
            }

            count += 1;
        }

        if allow_incomplete {
            // Find the best path and reconstruct

            if let Some(mut curr) = min_heur.map(|x| x.0) {
                let mut total = vec![];
                while let Some(dir) = camefrom.get(&curr) {
                    total.insert(0, *dir);
                    curr = dir.rotate_180().move_vec(curr);
                }
                total
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub fn send_callback(&self, callback: Box<Fn(&mut World)>) -> Result<(), SendError<Callback>> {
        self.callback_sender.send(Callback(callback))
    }
}

pub struct GenerationSettings {
    pub width: usize,
    pub height: usize,
    pub block_probs: HashMap<block::Block, f64>,
    pub amount_of_walls: f64,
    pub new_pos_prob: f64,
    pub new_dir_prob: f64,
    pub entities: Vec<EntityWrapper>,
}

impl GenerationSettings {
    pub fn default_for_difficulty(diff: Difficulty, include_player: bool, include_josef: bool) -> GenerationSettings {
        let mut entities = vec![];
        if include_player {
            entities.push(
                EntityWrapper::WPlayer(
                    Player::new((0, 0), diff.get_start_health())
                )
            );
        }
        if include_josef {
            entities.push(
                EntityWrapper::WJosef(
                    Josef::new(
                        (0, 0),
                        diff.get_josef_police_rate(),
                        diff.get_josef_speed(),
                        diff.get_josef_health()
                    )
                )
            );
        }
        GenerationSettings {
            width: 180,
            height: 111,
            block_probs: hashmap!{
                block::WALL.clone()   => 0.895,
                block::STONE.clone()  => 0.1,
                block::STAIRS.clone() => 0.005,
            },
            amount_of_walls: 10.0,
            new_pos_prob: 0.01,
            new_dir_prob: 0.05,
            entities: entities
        }
    }
}
