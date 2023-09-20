use logger_rs::{debug, error, info, trace, warn};
fn main() {
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
}
