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
        let mut file_style = buf.style();
        let mut line_style = buf.style();
        level_style.set_color(level_color);
        file_style.set_color(Color::Cyan);
        line_style.set_color(Color::Magenta);

        let args_string = record.args().to_string();
        let lines: Vec<&str> = args_string.lines().collect();
        let lines_len = lines.len();
        let mut new_args_string = String::new();

        if lines_len == 1 {
            new_args_string = args_string;
        } else {
            let output_len = format!(
                "[{level}] {file}:{line}",
                level = record.level(),
                file = record.file().unwrap_or("____unknown")[4..].to_string(),
                line = record.line().unwrap_or(0)
            )
            .chars()
            .count();
            let spaces = " ".repeat(output_len);

            for (index, line) in lines.iter().enumerate() {
                if index == 0 {
                    new_args_string.push_str(&(line.to_string() + "\n"));
                } else if index + 1 == lines_len {
                    new_args_string = new_args_string.trim_end_matches("\n").to_string();
                } else {
                    new_args_string.push_str(&format!(" {}{}\n", spaces, line));
                }
            }
        }

        writeln!(
            buf,
            "[{level}] {file}:{line} {args}",
            level = level_style.value(record.level()),
            args = new_args_string,
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
            // $create::ensure_logger_initialized();
            log::trace!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            // $create::ensure_logger_initialized();
            log::debug!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            // $create::ensure_logger_initialized();
            log::info!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            // $create::ensure_logger_initialized();
            log::warn!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            // $create::ensure_logger_initialized();
            log::error!($($arg)*);
        }
    };
}
