#[allow(dead_code)]
pub fn log_warning<T: AsRef<str>>(m: T) {
    println!("[WARNING] {}", m.as_ref());
}

#[allow(dead_code)]
pub fn log_debug<T: AsRef<str>>(m: T) {
    println!("[DEBUG] {}", m.as_ref());
}

#[allow(dead_code)]
pub fn log_special<T: AsRef<str>>(m: T) {
    println!("[SPECIAL] {}", m.as_ref());
}

#[allow(dead_code)]
pub fn oob_write_error(address: usize, value: &[u8]) -> String {
    format!("[ERROR] OOB Write: Trying to write {:?} at address {}", value, address)
}

#[allow(dead_code)]
pub fn oob_read_error(address: usize, size: usize) -> String {
    format!("[ERROR] OOB Read: Trying to read {} bytes at address {}", size, address)
}

#[allow(dead_code)]
pub fn integer_overflow_error(val1: usize, val2: usize) -> String {
    format!("[ERROR] Integer overflow: Trying to add {} to {}", val2, val1)
}

#[allow(dead_code)]
pub fn register_error(reg_index: usize) -> String {
    format!("[ERROR] Tried to read from non existing register: V{}", reg_index)
}

#[allow(dead_code)]
pub fn stack_pop_error() -> String {
    format!("[ERROR] Tried to pop value from empty stack")
}
