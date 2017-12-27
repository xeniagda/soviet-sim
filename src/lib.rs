#![feature(vec_remove_item, nll, universal_impl_trait, conservative_impl_trait)]

#[macro_use]
extern crate lazy_static;

mod ext;
mod key;
mod world;
mod controls;
mod block;
mod entity;
mod shape;

use world::*;

use std::sync::Mutex;
use std::panic::set_hook;

struct Rougelike<'a> {
    world: World<'a>,
    keys_down: Vec<key::Key>
}

lazy_static! {
    static ref ROUGELIKE: Mutex<Rougelike<'static>> = Mutex::new(Rougelike {
        world: World::empty(),
        keys_down: vec![]
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


    if let Ok(mut rouge) = ROUGELIKE.try_lock() {
        rouge.world.generate(width as usize, height as usize);

        rouge.world.draw();
    }
}

// Called 60 times every second from JavaScript
#[no_mangle]
pub fn tick() {
    if let Ok(mut rouge) = ROUGELIKE.try_lock() {
        rouge.world.tick();
    }
}

#[no_mangle]
pub fn key_down(key_code: u8) {
    match key::parse_key(key_code) {
        Some(key) => {
            // log(&format!("Pressed key: {} -> {:?}", key_code, key));
            if let Ok(mut rouge) = ROUGELIKE.try_lock() {

                if let Some(ref cont) = controls::parse_control(&key, &rouge.keys_down) {
                    // log(&format!("Control: {:?}", cont));
                    rouge.world.do_action(&cont.action);
                }

                rouge.keys_down.push(key);
                rouge.world.draw();
            }
        }
        None => {
            // log(&format!("Pressed key: {}", key_code));
        }
    }
}

#[no_mangle]
pub fn key_up(key_code: u8) {
    match key::parse_key(key_code) {
        Some(key) => {
            // log(&format!("Released key: {} -> {:?}", key_code, key));
            if let Ok(mut rouge) = ROUGELIKE.try_lock() {
                rouge.keys_down.remove_item(&key);
            }
        }
        None => {
            // log(&format!("Released key: {}", key_code));
        }
    }
}
