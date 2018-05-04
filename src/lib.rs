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

use world::*;
use difficulty::Difficulty;
use shape::Shape;

use std::sync::Mutex;
use std::sync::mpsc::{Receiver, channel};
use std::collections::HashSet;
use std::panic::set_hook;

const TITLE: &str = "☭☭☭ COMMUNISM SIMULATOR ☭☭☭";

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
    at_inventory: bool
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
        let mut actions_to_process = vec![];
        let size = game.size;
        match game.state {
            GameState::Playing(ref mut rouge) => {
                rouge.world.tick();
                rouge.world.draw(size);
                if rouge.at_inventory {
                    draw_inventory(size);
                }

                while let Ok(action) = rouge.action_receiver.try_recv() {
                    actions_to_process.push(action);
                }
            }
            GameState::Menu(difficulty) => {
                draw_menu(difficulty, size);
            }
            GameState::GameOver(difficulty, msg) => {
                draw_game_over(difficulty, msg, size);
            }

        }
        for action in actions_to_process {
            match action {
                MetaAction::Die => {
                    game.state = GameState::GameOver(Difficulty::Easy, RestartMessage::Died);
                }
                MetaAction::Win => {
                    game.state = GameState::GameOver(Difficulty::Easy, RestartMessage::Won);
                }
            }
        }
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
    for (i, ch) in TITLE.chars().enumerate() {
        ext::put_char(
            (i as u16 + (size.0 - TITLE.chars().count() as u16) / 2, 0),
            &Shape::new(ch, (255, 255, 0), (255, 0, 0))
            );
    }


    for (i, ch) in format!("Diffiulty: {}", difficulty.to_string()).chars().enumerate() {
        ext::put_char((i as u16 + 1, 3), &Shape::new(ch, (255, 255, 255), (0, 0, 0)));
    }

    for (i, ch) in "Press enter to start!".chars().enumerate() {
        ext::put_char((i as u16 + 1, 6), &Shape::new(ch, (255, 255, 255), (0, 0, 0)));
    }
}

fn draw_game_over(difficulty: Difficulty, msg: RestartMessage, size: (u16, u16)) {
    ext::clear();

    for (i, ch) in "game over lol. press enter to continue".chars().enumerate() {
        ext::put_char((i as u16, 0), &Shape::new(ch, (255, 0, 0), (0, 0, 0)));
    }

    let (text, col) = match msg {
        RestartMessage::Died => (&"u ded lol!", (255, 0, 0)),
        RestartMessage::Won  => (&"gj", (0, 255, 0)),
    };

    for (i, ch) in text.chars().enumerate() {
        ext::put_char((i as u16, 1), &Shape::new(ch, col, (0, 0, 0)));
    }
}

fn draw_inventory(size: (u16, u16)) {
    ext::clear();

    ext::put_char((0, 0), &Shape::new('I', (255, 255, 255), (0, 0, 0)))
}

pub fn init_game(difficulty: Difficulty) {
    if let Ok(mut game) = GAME.try_lock() {
        let (send, recv) = channel::<MetaAction>();

        let mut rouge = WorldWrapper {
            world: World::empty(difficulty, send),
            action_receiver: recv,
            keys_down: HashSet::new(),
            at_inventory: false,
        };

        rouge.world.generate(game.size.0 as usize, (game.size.1 - world::INVENTORY_HEIGHT) as usize);

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
                let size = game.size;
                match game.state {
                    GameState::Playing(ref mut rouge) => {
                        if let Some(ref cont) = controls::parse_control(&key, &rouge.keys_down) {
                            ext::log(&format!("Control: {:?}", cont));
                            if let controls::Action::ToggleInventory = cont.action {
                                rouge.at_inventory = !rouge.at_inventory;
                            }
                            rouge.world.do_action(&cont.action);
                        }


                        rouge.keys_down.insert(key);
                        rouge.world.draw(size);
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
            _ => {

            }
        }
    }
}
