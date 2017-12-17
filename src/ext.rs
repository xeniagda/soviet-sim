use std::os::raw::c_char;

extern {
    // Unsafe, use `put_char`
    fn u_put_char(x: u16, y: u16, ch: usize, r: u8, g: u8, b: u8);
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

pub fn put_char(pos: (u16, u16), ch: char, rgb: (u8, u8, u8)) {
    unsafe {
        u_put_char(pos.0, pos.1, ch as usize, rgb.0, rgb.1, rgb.2);
    }
}

pub fn rand() -> f64 {
    unsafe { u_rand() }
}
