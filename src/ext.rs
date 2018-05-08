use shape::Shape;

use std::os::raw::c_char;
use std::sync::Mutex;

extern {
    fn u_put_char(x: u16, y: u16, ch: usize, fr: u8, fg: u8, fb: u8, br: u8, bg: u8, bb: u8);
    fn u_clear();
    fn u_log(msg: c_char);
    fn u_rand() -> f64;
}

// Safe wrappers

pub fn rand() -> f64 {
    unsafe { u_rand() }
}

pub fn log(x: &str) {
    unsafe {
        x.chars().for_each(|c| u_log(c as c_char));
        u_log(10 as c_char); // Newline
    }
}

lazy_static! {
    static ref SCREEN: Mutex<Vec<Vec<Shape>>> = Mutex::new(vec![]);
}

pub fn put_char(pos: (u16, u16), shape: &Shape) {
    let should_draw = match SCREEN.try_lock() {
        Ok(mut screen) => {
            while screen.len() <= pos.0 as usize {
                screen.push(vec![]);
            }
            while screen[pos.0 as usize].len() <= pos.1 as usize {
                screen[pos.0 as usize].push(Shape::new(' ', (0,0,0), (0,0,0)));
            }

            if screen[pos.0 as usize][pos.1 as usize] != *shape {
                screen[pos.0 as usize][pos.1 as usize] = *shape;
                true
            } else {
                false
            }
        }
        _ => true
    };

    if should_draw {
        unsafe {
            u_put_char(pos.0, pos.1, shape.ch as usize, shape.col.0, shape.col.1, shape.col.2, shape.bg.0, shape.bg.1, shape.bg.2);
        }
    }
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
                screen[pos.0 as usize].push(Shape::new(' ', (0,0,0), (0,0,0)));
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
        unsafe {
            u_clear();
        }
    }
}
