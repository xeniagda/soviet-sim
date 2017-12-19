use std::os::raw::c_char;
use std::sync::Mutex;

use shape::Shape;

extern {
    // Unsafe, use `put_char`
    fn u_put_char(x: u16, y: u16, ch: usize, fr: u8, fg: u8, fb: u8, br: u8, bg: u8, bb: u8);
    fn u_log(msg: c_char);
    fn u_rand() -> f64;
}


// Safe wrappers

pub fn log(x: &str) {
    unsafe {
        for ch in x.chars() {
            u_log(ch as c_char);
        }
        u_log(10 as c_char); // Newline
    }
}

lazy_static! {
    static ref SCREEN: Mutex<Vec<Vec<Shape>>> = Mutex::new(vec![]);
}

pub fn put_char(pos: (u16, u16), shape: &Shape) {
    let mut screen = SCREEN.lock().unwrap();

    while screen.len() <= pos.0 as usize {
        screen.push(vec![]);
    }
    while screen[pos.0 as usize].len() <= pos.1 as usize {
        screen[pos.0 as usize].push(Shape::new(' ', (0,0,0), (0,0,0)));
    }

    if screen[pos.0 as usize][pos.1 as usize] != *shape {
        screen[pos.0 as usize][pos.1 as usize] = *shape;
        unsafe {
            u_put_char(pos.0, pos.1, shape.ch as usize, shape.col.0, shape.col.1, shape.col.2, shape.bg.0, shape.bg.1, shape.bg.2);
        }
    }
}

pub fn rand() -> f64 {
    unsafe { u_rand() }
}
