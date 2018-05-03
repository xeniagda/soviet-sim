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

enum GameState {
    Playing(SovietSim),
    Menu(Difficulty, Option<RestartMessage>),
}

enum RestartMessage {
    Died, Won
}

struct SovietSim {
    world: World,
    action_receiver: Receiver<MetaAction>,
    keys_down: HashSet<key::Key>,
    size: (u16, u16),
}

lazy_static! {
    static ref SOVIET_SIM: Mutex<GameState> = Mutex::new(GameState::Menu(Difficulty::Easy, None));
    static ref GAME_SIZE: Mutex<(u16, u16)> = Mutex::new((0, 0));
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


    if let Ok(mut size) = GAME_SIZE.try_lock() {
        *size = (width, height);
    }
}

// Called 60 times every second from JavaScript
#[no_mangle]
pub fn tick() {
    if let Ok(mut state) = SOVIET_SIM.try_lock() {
        let mut actions_to_process = vec![];
        match *state {
            GameState::Playing(ref mut rouge) => {
                rouge.world.tick();
                rouge.world.draw(&rouge.size);

                while let Ok(action) = rouge.action_receiver.try_recv() {
                    actions_to_process.push(action);
                }
            }
            GameState::Menu(difficulty, ref msg) => {
                ext::clear();

                for (i, ch) in format!("{:?}", difficulty).chars().enumerate() {
                    ext::put_char((i as u16, 0), &Shape::new(ch, (255, 255, 255), (0, 0, 0)));
                }

                if let Some(msg) = msg {
                    let (text, col) = match msg {
                        RestartMessage::Died => (&"You died!", (255, 0, 0)),
                        RestartMessage::Won  => (&"You won!", (0, 255, 0)),
                    };

                    for (i, ch) in text.chars().enumerate() {
                        ext::put_char((i as u16, 10), &Shape::new(ch, col, (0, 0, 0)));
                    }
                }
            }
        }
        for action in actions_to_process {
            match action {
                MetaAction::Die => {
                    *state = GameState::Menu(Difficulty::Easy, Some(RestartMessage::Died));
                }
                MetaAction::Win => {
                    *state = GameState::Menu(Difficulty::Easy, Some(RestartMessage::Won));
                }
            }
        }
    }
}

pub fn init_game(difficulty: Difficulty) {
    if let Ok(mut state) = SOVIET_SIM.try_lock() {
        if let Ok(size) = GAME_SIZE.try_lock() {
            let (send, recv) = channel::<MetaAction>();

            let mut rouge = SovietSim {
                world: World::empty(difficulty, send),
                action_receiver: recv,
                size: *size,
                keys_down: HashSet::new(),
            };

            rouge.world.generate(size.0 as usize, (size.1 - world::INVENTORY_HEIGHT) as usize);

            rouge.world.draw(&rouge.size);

            *state = GameState::Playing(rouge);
        }
    }
}

#[no_mangle]
pub fn key_down(key_code: u8) {
    match key::parse_key(key_code) {
        Some(key) => {
            ext::log(&format!("Pressed key: {} -> {:?}", key_code, key));
            if let Ok(mut state) = SOVIET_SIM.try_lock() {
                match *state {
                    GameState::Playing(ref mut rouge) => {
                        if let Some(ref cont) = controls::parse_control(&key, &rouge.keys_down) {
                            ext::log(&format!("Control: {:?}", cont));
                            rouge.world.do_action(&cont.action);
                        }

                        rouge.keys_down.insert(key);
                        rouge.world.draw(&rouge.size);
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
            if let Ok(mut state) = SOVIET_SIM.try_lock() {
                match *state {
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
    if let Ok(state) = SOVIET_SIM.try_lock() {
        ext::clear();
        match *state {
            GameState::Playing(ref rouge) => {
                rouge.world.draw(&rouge.size);
            }
            _ => {

            }
        }
    }
}