use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

#[derive(Debug)]
pub struct Unique {
    pub global_const: String,
}

///
/// Get the singleton that is only once initialized
pub fn get_singleton() -> &'static Mutex<Unique> {
    static mut UNIQ: MaybeUninit<Mutex<Unique>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe {
        UNIQ.as_mut_ptr().write(Mutex::new(Unique {
            global_const: "Global string".to_string(),
        }));
    });

    unsafe { &*UNIQ.as_ptr() }
}

pub fn set_singleton(value: &str, singleton_instance: &Mutex<Unique>) {
    let mut new_value = singleton_instance.lock().unwrap();
    new_value.global_const = value.to_string();
}
