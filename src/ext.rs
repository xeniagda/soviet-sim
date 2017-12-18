use std::os::raw::c_char;

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

pub fn put_char(pos: (u16, u16), ch: char, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
    unsafe {
        u_put_char(pos.0, pos.1, ch as usize, fg.0, fg.1, fg.2, bg.0, bg.1, bg.2);
    }
}

pub fn rand() -> f64 {
    unsafe { u_rand() }
}
