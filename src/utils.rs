
pub fn log_warning<T: AsRef<str>>(m: T) {
    //println!("[WARNING] {}", m.as_ref());
}

pub fn log_debug<T: AsRef<str>>(m: T) {
    println!("[DEBUG] {}", m.as_ref());
}

pub fn log_special<T: AsRef<str>>(m: T) {
    //println!("[SPECIAL] {}", m.as_ref());
}