use log::{error, set_logger, set_max_level, warn, Level, LevelFilter, Log, Metadata, Record};
#[cfg(feature = "ffi")]
use std::ffi::{c_char, CString};

pub fn init_perigee_logger() {
    #[cfg(debug_assertions)]
    let max_level = LevelFilter::Trace;
    #[cfg(not(debug_assertions))]
    let max_level = LevelFilter::Error;

    // If the below fails even when a logger has never been set
    // then we're SoL
    if let Ok(_) = set_logger(&PerigeeLogger).map(|_| set_max_level(max_level)) {
        std::panic::set_hook(Box::new(|panic_info| {
            error!("{}", panic_info.to_string());
        }));
    } else {
        warn!("Perigee logger already set!");
    }
}

#[cfg(feature = "ffi")]
extern "C" {
    fn on_error(string_ptr: *const c_char);
    fn on_warn(string_ptr: *const c_char);
    fn on_debug(string_ptr: *const c_char);
    fn on_info(string_ptr: *const c_char);
    fn on_trace(string_ptr: *const c_char);
}

#[cfg(not(feature = "ffi"))]
fn on_error(msg: String) {
    println!("[ERROR] {}", msg);
}

#[cfg(not(feature = "ffi"))]
fn on_warn(msg: String) {
    println!("[WARN] {}", msg);
}

#[cfg(not(feature = "ffi"))]
fn on_debug(msg: String) {
    println!("[DEBUG] {}", msg);
}

#[cfg(not(feature = "ffi"))]
fn on_info(msg: String) {
    println!("[INFO] {}", msg);
}

#[cfg(not(feature = "ffi"))]
fn on_trace(msg: String) {
    println!("[TRACE] {}", msg);
}

struct PerigeeLogger;

impl Log for PerigeeLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg_string = format!("{}", record.args());
            #[cfg(feature = "ffi")]
            {
                let msg_cstring = CString::new(msg_string)
                    .unwrap_or(CString::new("Unknown log received. Something's wrong").unwrap());
                unsafe {
                    match record.level() {
                        Level::Error => on_error(msg_cstring.as_ptr()),
                        Level::Warn => on_warn(msg_cstring.as_ptr()),
                        Level::Info => on_info(msg_cstring.as_ptr()),
                        Level::Debug => on_debug(msg_cstring.as_ptr()),
                        Level::Trace => on_trace(msg_cstring.as_ptr()),
                    };
                };
            }
            #[cfg(not(feature = "ffi"))]
            {
                match record.level() {
                    Level::Error => on_error(msg_string),
                    Level::Warn => on_warn(msg_string),
                    Level::Info => on_info(msg_string),
                    Level::Debug => on_debug(msg_string),
                    Level::Trace => on_trace(msg_string),
                };
            }
        }
    }

    fn flush(&self) {}
}
