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
    Menu(Difficulty, Option<RestartMessage>),
}

#[derive(Clone, Copy)]
enum RestartMessage {
    Died, Won
}

struct WorldWrapper {
    world: World,
    action_receiver: Receiver<MetaAction>,
    keys_down: HashSet<key::Key>,
}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(
        Game {
            state: GameState::Menu(Difficulty::Easy, None),
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

                while let Ok(action) = rouge.action_receiver.try_recv() {
                    actions_to_process.push(action);
                }
            }
            GameState::Menu(difficulty, msg) => {
                draw_menu(difficulty, msg, size);
            }
        }
        for action in actions_to_process {
            match action {
                MetaAction::Die => {
                    game.state = GameState::Menu(Difficulty::Easy, Some(RestartMessage::Died));
                }
                MetaAction::Win => {
                    game.state = GameState::Menu(Difficulty::Easy, Some(RestartMessage::Won));
                }
            }
        }
    }
}

fn draw_menu(difficulty: Difficulty, msg: Option<RestartMessage>, size: (u16, u16)) {
    ext::clear();


    // Title
    for (i, ch) in TITLE.chars().enumerate() {
        ext::put_char((i as u16 + (size.0 - TITLE.len() as u16) / 2, 0), &Shape::new(ch, (255, 255, 0), (255, 0, 0)));
    }

    for (i, ch) in format!("Diffiulty: {}", difficulty.to_string()).chars().enumerate() {
        ext::put_char((i as u16, 3), &Shape::new(ch, (255, 255, 255), (0, 0, 0)));
    }

    for (i, ch) in "Press enter to start!".chars().enumerate() {
        ext::put_char((i as u16, 6), &Shape::new(ch, (255, 255, 255), (0, 0, 0)));
    }

    if let Some(msg) = msg {
        let (text, col) = match msg {
            RestartMessage::Died => (&"You died!", (255, 0, 0)),
            RestartMessage::Won  => (&"You won!", (0, 255, 0)),
        };

        for (i, ch) in text.chars().enumerate() {
            ext::put_char((i as u16, 1), &Shape::new(ch, col, (0, 0, 0)));
        }
    }
}

pub fn init_game(difficulty: Difficulty) {
    if let Ok(mut game) = GAME.try_lock() {
        let (send, recv) = channel::<MetaAction>();

        let mut rouge = WorldWrapper {
            world: World::empty(difficulty, send),
            action_receiver: recv,
            keys_down: HashSet::new(),
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
    match key::parse_key(key_code) {
        Some(key) => {
            // log(&format!("Released key: {} -> {:?}", key_code, key));
            if let Ok(mut game) = GAME.try_lock() {
                match game.state {
                    GameState::Playing(ref mut rouge) => {
                        rouge.keys_down.remove(&key);
                    }
                    GameState::Menu(ref mut difficulty, _) => {
                        match key {
                            key::Key::Right => { *difficulty = difficulty.harder() }
                            key::Key::Left  => { *difficulty = difficulty.easier() }
                            key::Key::Enter => { start = Some(*difficulty); }
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