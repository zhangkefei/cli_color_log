//! # cli_color_log
//! 
//! A convenient color log tool for cli.
//! 
//! ## usage
//! ```
//! use cli_color_log::Logger;
//! use cli_color_log::LogType;
//! 
//! let logger = Logger::new(12);
//! logger.info("This is a info message.");
//! logger.warn("This is a warning message.");
//! logger.error("This is a error message.");
//! logger.style_log(LogType::Info, "Download", "This is a custom style message.");
//! logger.style_log(LogType::Plain, "Console", "This is a plain style message.");
//! logger.log("This is a plain log message.");
//! ```
use console::Style;

pub struct Logger {
    length: usize, //the notice length
    info_style: Style,
    warn_style: Style,
    error_style: Style,
}

pub enum LogType {
    Info,
    Warn,
    Error,
    Plain,
}

impl Logger {
    pub fn new(length: usize) -> Logger {
        Logger {
            length,
            info_style: Style::new().bold().green(),
            warn_style: Style::new().bold().yellow(),
            error_style: Style::new().bold().red(),
        }
    }

    pub fn info(&self, msg: &str) {
        self.style_log(LogType::Info, "Info", msg);
    }

    pub fn warn(&self, msg: &str) {
        self.style_log(LogType::Warn, "Warn", msg);
    }

    pub fn error(&self, msg: &str) {
        self.style_log(LogType::Error, "Error", msg);
    }

    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }

    pub fn style_log(&self, style: LogType, tag: &str, msg: &str) {
        match style {
            LogType::Info => {
                println!("{} {}", self.info_style.apply_to(self._left_space(tag)), msg);
            },
            LogType::Warn => {
                println!("{} {}", self.warn_style.apply_to(self._left_space(tag)), msg);
            },
            LogType::Error => {
                println!("{} {}", self.error_style.apply_to(self._left_space(tag)), msg);
            },
            LogType::Plain => {
                println!("{} {}", self._left_space(tag), msg);
            },
        }
    }

    fn _left_space(&self, msg: &str) -> String {
        let mut result = String::new();
        let msg_length = msg.len();
        if self.length > msg_length {
            let left_count = self.length - msg_length;
            for _number in 0..left_count {
                result.push(' ');
            }
        }
        result.push_str(msg);
        result
    }
}
