use shape::Shape;

use std::os::raw::c_char;
use std::sync::Mutex;

extern {
    fn u_put_char(x: u16, y: u16, ch: usize, fr: u8, fg: u8, fb: u8, br: u8, bg: u8, bb: u8);
    fn u_clear();
    #[allow(unused)]
    fn u_log(msg: c_char);
    fn u_rand() -> f64;
}

// Safe wrappers

pub fn rand() -> f64 {
    unsafe { u_rand() }
}

#[cfg(debug_assertions)]
pub fn log(x: &str) {
    unsafe {
        x.chars().for_each(|c| u_log(c as c_char));
        u_log(10 as c_char); // Newline
    }
}

#[cfg(not(debug_assertions))]
pub fn log(_x: &str) {
}

lazy_static! {
    static ref SCREEN: Mutex<Vec<Vec<Shape>>> = Mutex::new(vec![]);
    static ref UNFLIPPED: Mutex<Vec<Vec<Shape>>> = Mutex::new(vec![]);
}

pub fn flip() {
    if let Ok(mut screen) = SCREEN.lock() {
        if let Ok(unflipped) = UNFLIPPED.lock() {
            for (x, col) in unflipped.iter().enumerate() {
                for (y, shape) in col.iter().enumerate() {
                    if Some(shape) != screen.get(x as usize).and_then(|col| col.get(y as usize)) {
                        unsafe {
                            u_put_char(
                                x as u16, y as u16, shape.ch as usize,
                                shape.col.0, shape.col.1, shape.col.2, shape.bg.0, shape.bg.1, shape.bg.2);
                        }
                    }
                }
            }
            *screen = unflipped.clone();
        }
    }
}

pub fn put_char(pos: (u16, u16), shape: &Shape) {
    if let Ok(mut unflipped) = UNFLIPPED.lock() {
        while unflipped.len() <= pos.0 as usize {
            unflipped.push(vec![]);
        }
        while unflipped[pos.0 as usize].len() <= pos.1 as usize {
            unflipped[pos.0 as usize].push(Shape::empty());
        }

        if unflipped[pos.0 as usize][pos.1 as usize] != *shape {
            unflipped[pos.0 as usize][pos.1 as usize] = *shape;
        }
    }
}

pub fn erase(pos: (u16, u16)) {
    put_char(pos, &Shape::empty());
}

pub fn put_text(pos: (u16, u16), text: &str, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
    for (i, ch) in text.chars().enumerate() {
        put_char((pos.0 + i as u16, pos.1), &Shape::new(ch, fg, bg));
    }
}

pub fn recolor(pos: (u16, u16), fg: (u8, u8, u8), bg: (u8, u8, u8)) {
    match SCREEN.try_lock() {
        Ok(mut screen) => {
            while screen.len() <= pos.0 as usize {
                screen.push(vec![]);
            }
            while screen[pos.0 as usize].len() <= pos.1 as usize {
                screen[pos.0 as usize].push(Shape::empty());
            }
            let current = screen[pos.0 as usize][pos.1 as usize];

            screen[pos.0 as usize][pos.1 as usize].col = fg;
            screen[pos.0 as usize][pos.1 as usize].bg = bg;
            unsafe {
                // Put a space instead
                u_put_char(pos.0, pos.1, current.ch as usize, fg.0, fg.1, fg.2, bg.0, bg.1, bg.2);
            }
        }
        Err(_) => {
            unsafe {
                // Put a space instead
                u_put_char(pos.0, pos.1, 32, fg.0, fg.1, fg.2, bg.0, bg.1, bg.2);
            }
        }
    }
}

pub fn clear() {
    if let Ok(ref mut screen) = SCREEN.try_lock() {
        screen.clear();
    }
    if let Ok(ref mut unflipped) = UNFLIPPED.try_lock() {
        unflipped.clear();
    }
    unsafe {
        u_clear();
    }
}
