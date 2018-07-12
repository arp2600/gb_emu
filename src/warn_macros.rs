// usage:
//   eprintln_once("string literal {}", "format args");
//
// this will only print once
// useful for not spamming duplicate warnings
macro_rules! eprintln_once {
    ($($args:tt)*) => {
        unsafe {
            static mut PRINTED: bool = false;
            if !PRINTED {
                eprintln!($($args)*);
                PRINTED = true;
            }
        }
    };
}

// usage:
//   eprintln_once_per_key(key, key_type, "string literal {}", "format args");
//
// this will only print once per key
// useful for not spamming duplicate warnings
macro_rules! eprintln_once_per_key {
    ($key:expr, $type:ty, $($args:tt)*) => {
        use std::sync::Mutex;
        lazy_static! {
            static ref PRINTED_MAP: Mutex<HashSet<$type>> = {
                Mutex::new(HashSet::new())
            };
        }
        let mut map = PRINTED_MAP.lock().unwrap();
        if !map.contains(&$key) {
            eprintln!($($args)*);
            map.insert($key);
        }
    };
}
