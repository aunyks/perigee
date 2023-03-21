use log::{error, set_logger, set_max_level, warn, Level, LevelFilter, Log, Metadata, Record};

/// Set a [PerigeeLogger](crate::logger::PerigeeLogger) as the global logging implementation
/// if none is set.
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
    fn on_error(string_ptr: *const u8, string_len: usize);
    fn on_warn(string_ptr: *const u8, string_len: usize);
    fn on_debug(string_ptr: *const u8, string_len: usize);
    fn on_info(string_ptr: *const u8, string_len: usize);
    fn on_trace(string_ptr: *const u8, string_len: usize);
}

#[cfg(not(feature = "ffi"))]
fn on_error(msg: &str) {
    println!("[ERROR] {}", msg);
}

#[cfg(not(feature = "ffi"))]
fn on_warn(msg: &str) {
    println!("[WARN] {}", msg);
}

#[cfg(not(feature = "ffi"))]
fn on_debug(msg: &str) {
    println!("[DEBUG] {}", msg);
}

#[cfg(not(feature = "ffi"))]
fn on_info(msg: &str) {
    println!("[INFO] {}", msg);
}

#[cfg(not(feature = "ffi"))]
fn on_trace(msg: &str) {
    println!("[TRACE] {}", msg);
}

/// A global logger implementation designed
/// for use in WebAssembly.
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
                let msg_string_len = msg_string.len();
                unsafe {
                    match record.level() {
                        Level::Error => on_error(msg_string.as_ptr(), msg_string_len),
                        Level::Warn => on_warn(msg_string.as_ptr(), msg_string_len),
                        Level::Info => on_info(msg_string.as_ptr(), msg_string_len),
                        Level::Debug => on_debug(msg_string.as_ptr(), msg_string_len),
                        Level::Trace => on_trace(msg_string.as_ptr(), msg_string_len),
                    };
                };
            }
            #[cfg(not(feature = "ffi"))]
            {
                match record.level() {
                    Level::Error => on_error(&msg_string),
                    Level::Warn => on_warn(&msg_string),
                    Level::Info => on_info(&msg_string),
                    Level::Debug => on_debug(&msg_string),
                    Level::Trace => on_trace(&msg_string),
                };
            }
        }
    }

    fn flush(&self) {}
}
