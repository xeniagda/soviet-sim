#![feature(vec_remove_item, nll)]

#[macro_use]
extern crate lazy_static;

mod ext;
mod key;
mod world;
mod controls;
mod block;
mod entity;
mod shape;
mod difficulty;
mod crafting;
mod inventory;
mod move_dir;

use world::*;
use difficulty::Difficulty;
use shape::Shape;
use move_dir::MoveDir;

use std::sync::Mutex;
use std::sync::mpsc::{Receiver, channel};
use std::collections::HashSet;
use std::panic::set_hook;

const TITLE: &str = "☭☭☭ COMMUNISM SIMULATOR ☭☭☭";
const INVENTORY_TITLE: &str = "☭☭☭ INVENTORY ☭☭☭";
const INVENTORY_INVENTORY: &str = "Your inventory";
const INVENTORY_CRAFTING: &str = "Crafting";
const INVENTORY_INDENT: u16 = 5;

const WORLD_SIZE: (usize, usize) = (180, 111);

struct Game {
    state: GameState,
    size: (u16, u16),
}

enum GameState {
    Playing(WorldWrapper),
    Menu(Difficulty),
    GameOver(Difficulty, RestartMessage),
}

#[derive(Clone, Copy)]
enum RestartMessage {
    Died, Won
}

struct WorldWrapper {
    world: World,
    action_receiver: Receiver<MetaAction>,
    keys_down: HashSet<key::Key>,
    at_inventory: Option<AtInventory>
}

#[derive(Default, Debug, Clone, Copy)]
struct AtInventory {
    selected_recipe: usize,
    scroll: u16,
}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(
        Game {
            state: GameState::Menu(Difficulty::Easy),
            size: (0, 0)
        });
}


#[no_mangle]
pub fn start(width: u16, height: u16) {
    // Set panic hook
    set_hook(Box::new(|info| {
        ext::log(&format!("FATAL ERROR:"));
        if let Some(payload) = info.payload().downcast_ref::<&str>() {
            ext::log(&format!("    Payload: {:?}", payload));
        } else if let Some(payload) = info.payload().downcast_ref::<String>() {
            ext::log(&format!("    Payload: {:?}", payload));
        } else {
            ext::log(&format!("    Payload: unknown"));
        }
        if let Some(location) = info.location() {
            ext::log(&format!("    At: {:?}", location));
        } else {
            ext::log(&format!("    At: unknown"));
        }
    }));

    if let Ok(mut game) = GAME.try_lock() {
        game.size = (width, height);
    }
}

// Called 60 times every second from JavaScript
#[no_mangle]
pub fn tick() {
    if let Ok(mut game) = GAME.try_lock() {
        let mut diff = Difficulty::Easy;

        let mut actions_to_process = vec![];
        let size = game.size;
        match game.state {
            GameState::Playing(ref mut rouge) => {
                diff = rouge.world.difficulty;
                if let Some(inv) = rouge.at_inventory {
                    rouge.world.draw(size);
                    draw_inventory(inv, rouge, size);
                } else {
                    rouge.world.tick();
                    rouge.world.update_scroll(size);
                    rouge.world.draw(size);
                }

                while let Ok(action) = rouge.action_receiver.try_recv() {
                    actions_to_process.push(action);
                }
            }
            GameState::Menu(difficulty) => {
                draw_menu(difficulty, size);
            }
            GameState::GameOver(_, msg) => {
                draw_game_over(msg, size);
            }
        }
        ext::flip();

        for action in actions_to_process {
            match action {
                MetaAction::Die => {
                    game.state = GameState::GameOver(diff, RestartMessage::Died);
                }
                MetaAction::Win => {
                    game.state = GameState::GameOver(diff, RestartMessage::Won);
                }
            }
        }
    }
}

#[no_mangle]
pub fn resize(width: u16, height: u16) {
    if let Ok(mut game) = GAME.try_lock() {
        game.size = (width, height);
        ext::clear();
    }
}

fn draw_menu(difficulty: Difficulty, size: (u16, u16)) {
    ext::clear();

    // Border
    for i in 0..size.0 {
        ext::put_char((i as u16, 0), &Shape::new('=', (255, 255, 255), (0, 0, 0)));
    }
    for i in 0..size.0 {
        ext::put_char((i as u16, size.1 - 1), &Shape::new('=', (255, 255, 255), (0, 0, 0)));
    }
    for i in 0..size.1 {
        ext::put_char((0, i as u16), &Shape::new('|', (255, 255, 255), (0, 0, 0)));
    }
    for i in 0..size.1 {
        ext::put_char((size.0 - 1, i as u16), &Shape::new('|', (255, 255, 255), (0, 0, 0)));
    }

    // Title
    ext::put_text(((size.0 - TITLE.chars().count() as u16) / 2, 0), TITLE, (255, 255, 0), (255, 0, 0));

    ext::put_text((1, 3), &format!("Diffiulty: {}", difficulty.to_string()), (255, 255, 255), (0, 0, 0));

    ext::put_text((1, 6), "Press enter to start!", (255, 255, 255), (0, 0, 0));
}

fn draw_game_over(msg: RestartMessage, _size: (u16, u16)) {
    ext::clear();

    ext::put_text((0, 3), "game over lol. press enter to continue", (255, 255, 255), (0, 0, 0));

    let (text, col) = match msg {
        RestartMessage::Died => (&"u ded lol!", (255, 0, 0)),
        RestartMessage::Won  => (&"gj", (0, 255, 0)),
    };

    ext::put_text((0, 0), text, col, (0, 0, 0));


}

fn draw_inventory(inv: AtInventory, ww: &mut WorldWrapper, size: (u16, u16)) {
    // Border
    for i in INVENTORY_INDENT..size.0-INVENTORY_INDENT {
        ext::put_char((i as u16, INVENTORY_INDENT), &Shape::new('=', (255, 255, 255), (0, 0, 0)));
    }
    for i in INVENTORY_INDENT..size.0-INVENTORY_INDENT {
        ext::put_char((i as u16, size.1 - INVENTORY_INDENT - 1), &Shape::new('=', (255, 255, 255), (0, 0, 0)));
    }
    for i in INVENTORY_INDENT..size.1-INVENTORY_INDENT {
        ext::put_char((INVENTORY_INDENT, i as u16), &Shape::new('|', (255, 255, 255), (0, 0, 0)));
    }
    for i in INVENTORY_INDENT..size.1-INVENTORY_INDENT {
        ext::put_char((size.0 - INVENTORY_INDENT - 1, i as u16), &Shape::new('|', (255, 255, 255), (0, 0, 0)));
    }

    // Clear inside
    for x in INVENTORY_INDENT+1..size.0-INVENTORY_INDENT-1 {
        for y in INVENTORY_INDENT+1..size.1-INVENTORY_INDENT-1 {
            ext::put_char((x, y), &Shape::new(' ', (0, 0, 0), (0, 0, 0)))
        }
    }

    // Draw title
    ext::put_text(
        ((size.0 - INVENTORY_TITLE.chars().count() as u16) / 2, INVENTORY_INDENT),
        INVENTORY_TITLE,
        (255, 255, 0), (255, 0, 0));

    // Draw bar
    for i in INVENTORY_INDENT+1..size.1-INVENTORY_INDENT-1 {
        ext::put_char((size.0 / 2, i), &Shape::new('|', (255, 255, 255), (0, 0, 0)));
    }

    // Draw titles
    ext::put_text(
        ((size.0 / 2 - INVENTORY_INVENTORY.chars().count() as u16) / 2, INVENTORY_INDENT + 1),
        INVENTORY_INVENTORY,
        (255, 255, 255), (0, 0, 0));

    // Draw titles
    ext::put_text(
        ((size.0 / 2 * 3 - INVENTORY_CRAFTING.chars().count() as u16) / 2, INVENTORY_INDENT + 1),
        INVENTORY_CRAFTING,
        (255, 255, 255), (0, 0, 0));

    // Draw inventory
    if let Some(entity::EntityWrapper::WPlayer(ref player)) =
        ww.world.get_player_id().and_then(|x| ww.world.entities.get(&x)) {
        for (i, (item, count)) in player.inventory.iter().enumerate() {
            ext::put_char(
                (INVENTORY_INDENT + 1, INVENTORY_INDENT + i as u16 + 2),
                &item.get_shape());
            ext::put_text(
                (INVENTORY_INDENT + 2, INVENTORY_INDENT + i as u16 + 2),
                &format!("x{}", count),
                (255, 255, 255), (0, 0, 0));
        }
    }

    // Helpers to keep in bounds. Returns:
    //     None - The text was placed
    //     Some(false) - The text is to far up to be placed
    //     Some(true) - The text is to far down to be placed
    let draw_crafting_str = move |pos: (u16, u16), text: &str, fg: (u8, u8, u8), bg: (u8, u8, u8)| {
        let pos_ = (pos.0 + size.0 / 2, INVENTORY_INDENT + pos.1 - inv.scroll);
        if pos_.1 <= INVENTORY_INDENT + 1 {
            return Some(false);
        }
        if pos_.1 >= size.1 - INVENTORY_INDENT - 1 {
            return Some(true);
        }
        ext::put_text(pos_, text, fg, bg);
        None
    };

    let draw_crafting_shape = move |pos: (u16, u16), sh: &Shape| {
        let pos_ = (pos.0 + size.0 / 2, INVENTORY_INDENT + pos.1 - inv.scroll);
        if pos_.1 <= INVENTORY_INDENT + 1 {
            return Some(false);
        }
        if pos_.1 >= size.1 - INVENTORY_INDENT - 1 {
            return Some(true);
        }
        ext::put_char(pos_, sh);
        None
    };

    let mut scroll_move: i16 = 0;

    let mut y = 2;
    // Draw recipes
    for (i, recipe) in crafting::RECIPES.iter().enumerate() {
        let mut drawn: Option<bool> = None;
        drawn = drawn.or(
            draw_crafting_shape((1, y), &recipe.out.get_shape())
            );
        if i == inv.selected_recipe {
            drawn = drawn.or(
                draw_crafting_str(
                    (4, y),
                    &recipe.out.get_name(),
                    (255, 255, 255),
                    (0, 0, 0))
                );
            let desc = recipe.out.get_desc();
            let mut desc_words = desc.split(" ");

            y += 1;
            let mut x = 2;

            while let Some(word) = desc_words.next() {
                if x + word.len() as u16 + size.0 / 2 >= size.0 - INVENTORY_INDENT - 1 {
                    x = 2;
                    y += 1;
                }
                drawn = drawn.or(draw_crafting_str((x, y), word, (150, 150, 255), (0, 0, 0)));
                x += word.len() as u16 + 1;
            }
            for (needed, amount) in recipe.needed.iter() {
                y += 1;
                drawn = drawn.or(draw_crafting_shape((3, y), &needed.get_shape()));
                drawn = drawn.or(
                    draw_crafting_str(
                              (4, y),
                              &format!("x{}", amount),
                              (255, 255, 255),
                              (0, 0, 0)
                          ));
            }
            match drawn {
                Some(false) => scroll_move = -1,
                Some(true)  => scroll_move = 1,
                None => {}
            }
        } else {
            draw_crafting_str(
                (4, y),
                &recipe.out.get_name(),
                (120, 120, 120),
                (0, 0, 0));
        }
        y += 3;
    }
    if let Some(ref mut i) = ww.at_inventory {
        if scroll_move < 0 {
            i.scroll = i.scroll.saturating_sub(-scroll_move as u16);
        } else {
            i.scroll = i.scroll.saturating_add(scroll_move as u16);
        }
        ext::log(&format!("Scroll: {}", i.scroll));
    }
}

pub fn init_game(difficulty: Difficulty) {
    if let Ok(mut game) = GAME.try_lock() {
        let (send, recv) = channel::<MetaAction>();

        let mut rouge = WorldWrapper {
            world: World::empty(difficulty, send),
            action_receiver: recv,
            keys_down: HashSet::new(),
            at_inventory: None,
        };

        rouge.world.generate(WORLD_SIZE.0, WORLD_SIZE.1);;

        rouge.world.draw(game.size);

        game.state = GameState::Playing(rouge);
    }
}

#[no_mangle]
pub fn key_down(key_code: u8) {
    match key::parse_key(key_code) {
        Some(key) => {
            ext::log(&format!("Pressed key: {} -> {:?}", key_code, key));
            if let Ok(mut game) = GAME.try_lock() {
                match game.state {
                    GameState::Playing(ref mut rouge) => {

                        if let Some(ref cont) = controls::parse_control(&key, &rouge.keys_down) {
                            ext::log(&format!("Control: {:?}", cont));
                            if let controls::Action::ToggleInventory = cont.action {
                                if rouge.at_inventory.is_some() {
                                    rouge.at_inventory = None;
                                } else {
                                    rouge.at_inventory = Some(AtInventory::default());
                                }
                            }
                            if rouge.at_inventory.is_none() {
                                rouge.world.do_action(&cont.action);
                            } else if let Some(ref mut inv) = rouge.at_inventory {
                                match cont.action {
                                    controls::Action::Move(MoveDir::Up) if inv.selected_recipe > 0 => {
                                        inv.selected_recipe -= 1;
                                    }
                                    controls::Action::Move(MoveDir::Down) if inv.selected_recipe < crafting::RECIPES.len() - 1 => {
                                        inv.selected_recipe += 1;
                                    }
                                    controls::Action::Select => {
                                        let curr_recipe = &crafting::RECIPES[inv.selected_recipe];
                                        if let Some(entity::EntityWrapper::WPlayer(player)) =
                                            rouge.world.get_player_id().and_then(|x| rouge.world.entities.get_mut(&x)) {
                                            player.craft(curr_recipe);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        rouge.keys_down.insert(key);
                    }
                    _ => {

                    }
                }
            }
        }
        None => {
            ext::log(&format!("Pressed key: {}", key_code));
        }
    }
}

#[no_mangle]
pub fn key_up(key_code: u8) {
    let mut start: Option<Difficulty> = None;
    let mut next_state: Option<GameState> = None;

    match key::parse_key(key_code) {
        Some(key) => {
            // log(&format!("Released key: {} -> {:?}", key_code, key));

            if let Ok(mut game) = GAME.try_lock() {
                match game.state {
                    GameState::Playing(ref mut rouge) => {
                        rouge.keys_down.remove(&key);
                    }
                    GameState::Menu(ref mut difficulty) => {
                        match key {
                            key::Key::Right => { *difficulty = difficulty.harder() }
                            key::Key::Left  => { *difficulty = difficulty.easier() }
                            key::Key::Enter => { start = Some(*difficulty); }
                            _ => {}
                        }
                    }
                    GameState::GameOver(difficulty, _) => {
                        match key {
                            key::Key::Enter => { next_state = Some(GameState::Menu(difficulty)); }
                            _ => {}
                        }
                    }
                }
            }
        }
        None => {
            // log(&format!("Released key: {}", key_code));
        }
    }
    if let Some(next_state) = next_state {
        if let Ok(mut game) = GAME.try_lock() {
            game.state = next_state;
        }
    }
    if let Some(difficulty) = start {
        init_game(difficulty);
    }
}

#[no_mangle]
pub fn redraw() {
    if let Ok(game) = GAME.try_lock() {
        ext::clear();
        match game.state {
            GameState::Playing(ref rouge) => {
                rouge.world.draw(game.size);
            }
            _ => { }
        }
        ext::flip();
    }
}
