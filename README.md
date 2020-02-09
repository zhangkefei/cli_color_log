# cli_color_log

A convenient color log tool for cli.

## usage
```
use cli_color_log::Logger;
use cli_color_log::LogType;

fn main() {
    let logger = Logger::new(12);
    logger.info("This is a info message.");
    logger.warn("This is a warning message.");
    logger.error("This is a error message.");
    logger.style_log(LogType::Info, "Download", "This is a custom style message.");
    logger.style_log(LogType::Plain, "Console", "This is a plain style message.");
    logger.log("This is a plain log message.");
}
```