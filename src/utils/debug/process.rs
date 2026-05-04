use crate::State;

pub fn debug_println(state: &State, message: &str) {
    if state.debug {
        println!("[DEBUG] {}", message);
    }
}