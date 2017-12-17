#![feature(lang_items, alloc, exclusive_range_pattern, inclusive_range_pattern, vec_remove_item)]

#[macro_use]
extern crate lazy_static;

mod ext;
mod key;
mod world;
mod controls;
mod block;

use world::*;
use ext::log;
use std::sync::Mutex;

struct Rougelike {
    world: World,
    keys_down: Vec<key::Key>
}

lazy_static! {
    static ref ROUGE: Mutex<Rougelike> = Mutex::new(Rougelike {
        world: World::new(
            vec![], (2, 2)
        ),
        keys_down: vec![]
    });
}


#[no_mangle]
pub fn start(width: u16, height: u16) {
    let mut rouge = ROUGE.lock().unwrap();
    rouge.world.generate(width as usize, height as usize);

    rouge.world.draw();
}

// Called 60 times every second from JavaScript
#[no_mangle]
pub fn tick() {
    let mut rouge = ROUGE.lock().unwrap();
    rouge.world.tick();
}

#[no_mangle]
pub fn key_down(key_code: u8) {
    match key::parse_key(key_code) {
        Some(key) => {
            log(&format!("Pressed key: {} -> {:?}", key_code, key));
            let mut rouge = ROUGE.lock().unwrap();

            if let Some(ref cont) = controls::parse_control(&key, &rouge.keys_down) {
                log(&format!("Control: {:?}", cont));
                rouge.world.do_action(&cont.action);
            }

            rouge.keys_down.push(key);
            rouge.world.draw();
        }
        None => { log(&format!("Pressed key: {}", key_code)); }
    }
}

#[no_mangle]
pub fn key_up(key_code: u8) {
    match key::parse_key(key_code) {
        Some(key) => {
            log(&format!("Released key: {} -> {:?}", key_code, key));
            let mut rouge = ROUGE.lock().unwrap();

            rouge.keys_down.remove_item(&key);
        }
        None => { log(&format!("Released key: {}", key_code)); }
    }
}
