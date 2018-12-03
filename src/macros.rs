use std::sync::Mutex;
use std::time::SystemTime;

lazy_static! {
    pub static ref LOGFILE: Mutex<std::fs::File> =
        { Mutex::new(std::fs::File::create("log").unwrap()) };
    pub static ref START_TIME: SystemTime = SystemTime::now();
}

macro_rules! log {
    ($msg:expr) => {{
        use std::io::Write;
        let now = crate::macros::START_TIME.elapsed().unwrap();
        let time = format!("[{}] ", now.as_secs());

        crate::macros::LOGFILE
            .lock()
            .unwrap()
            .write_all(time.as_bytes());
        crate::macros::LOGFILE
            .lock()
            .unwrap()
            .write_all($msg.as_bytes());
        crate::macros::LOGFILE.lock().unwrap().write(b"\n");
    }};
}
