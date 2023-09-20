use env_logger::{fmt::Color, Builder};
use log::{Level, LevelFilter};
use std::cell::RefCell;
use std::io::Write;

thread_local! {
    static LOGGER_INITIALIZED: RefCell<bool> = RefCell::new(false);
}

// pub fn ensure_logger_initialized() {
//     LOGGER_INITIALIZED.with(|initialized| {
//         if !*initialized.borrow() {
//             init_logger(); // ロギングの初期化を行う関数を呼び出す
//             *initialized.borrow_mut() = true;
//         }
//     });
// }

pub fn init_logger(filters: &[(Option<&str>, LevelFilter)]) {
    let mut builder = Builder::new();
    builder.format(|buf, record| {
        let level_color = match record.level() {
            Level::Trace => Color::White,
            Level::Debug => Color::Blue,
            Level::Info => Color::Green,
            Level::Warn => Color::Yellow,
            Level::Error => Color::Red,
        };
        let mut level_style = buf.style();
        level_style.set_color(level_color);
        let mut file_style = buf.style();
        file_style.set_color(Color::Cyan);
        let mut line_style = buf.style();
        line_style.set_color(Color::Magenta);

        writeln!(
            buf,
            "[{level}] {file}:{line} {args}",
            level = level_style.value(record.level()),
            args = record.args(),
            file = file_style.value(&record.file().unwrap_or("____unknown")[4..]),
            line = line_style.value(record.line().unwrap_or(0)),
        )
    });

    for (module, level_filter) in filters {
        match module {
            Some(module_name) => {
                builder.filter_module(module_name, *level_filter);
            }
            None => {
                builder.filter(None, *level_filter);
            }
        }
    }

    builder.write_style(env_logger::WriteStyle::Auto);

    builder.init();
}

// ログモジュールのフィルタリングを設定する関数

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        {
            // $crate::ensure_logger_initialized();
            log::trace!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            // $crate::ensure_logger_initialized();
            log::debug!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            // $crate::ensure_logger_initialized();
            log::info!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            // $crate::ensure_logger_initialized();
            log::warn!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            // $crate::ensure_logger_initialized();
            log::error!($($arg)*);
        }
    };
}
