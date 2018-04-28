#![feature(vec_remove_item, nll)]

#[macro_use]
extern crate lazy_static;
extern crate rand;

mod ext;
mod key;
mod world;
mod controls;
mod block;
mod entity;
mod shape;

use world::*;

use std::sync::Mutex;
use std::collections::HashSet;
use std::panic::set_hook;

struct SovietSim {
    world: World,
    keys_down: HashSet<key::Key>,
    size: (u16, u16)
}

lazy_static! {
    static ref SOVIET_SIM: Mutex<SovietSim> = Mutex::new(SovietSim {
        world: World::empty(),
        keys_down: HashSet::new(),
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


    if let Ok(mut rouge) = SOVIET_SIM.try_lock() {
        rouge.size = (width, height);
        rouge.world.generate(width as usize, height as usize - 2);

        rouge.world.draw(&rouge.size);
    }
}

// Called 60 times every second from JavaScript
#[no_mangle]
pub fn tick() {
    if let Ok(mut rouge) = SOVIET_SIM.try_lock() {
        rouge.world.tick();
        rouge.world.draw(&rouge.size);
    }
}

#[no_mangle]
pub fn key_down(key_code: u8) {
    match key::parse_key(key_code) {
        Some(key) => {
            ext::log(&format!("Pressed key: {} -> {:?}", key_code, key));
            if let Ok(mut rouge) = SOVIET_SIM.try_lock() {

                if let Some(ref cont) = controls::parse_control(&key, &rouge.keys_down) {
                    ext::log(&format!("Control: {:?}", cont));
                    rouge.world.do_action(&cont.action);
                }

                rouge.keys_down.insert(key);
                rouge.world.draw(&rouge.size);
            }
        }
        None => {
            ext::log(&format!("Pressed key: {}", key_code));
        }
    }
}

#[no_mangle]
pub fn key_up(key_code: u8) {
    match key::parse_key(key_code) {
        Some(key) => {
            // log(&format!("Released key: {} -> {:?}", key_code, key));
            if let Ok(mut rouge) = SOVIET_SIM.try_lock() {
                rouge.keys_down.remove(&key);
            }
        }
        None => {
            // log(&format!("Released key: {}", key_code));
        }
    }
}

#[no_mangle]
pub fn redraw() {
    if let Ok(rouge) = SOVIET_SIM.try_lock() {
        ext::clear();
        rouge.world.draw(&rouge.size);
    }
}
