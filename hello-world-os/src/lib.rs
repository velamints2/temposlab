#![no_std]
#![deny(unsafe_code)]

use ostd::prelude::*;
use log::{Log, Metadata, Record, error, warn, info, debug, trace, Level};
use owo_colors::OwoColorize;

struct ColoredLogger;

impl Log for ColoredLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => {
                    println!("[{}] {}", "ERROR".red(), record.args());
                }
                Level::Warn => {
                    println!("[{}] {}", "WARN".yellow(), record.args());
                }
                Level::Info => {
                    println!("[{}] {}", "INFO".cyan(), record.args());
                }
                Level::Debug => {
                    println!("[{}] {}", "DEBUG".blue(), record.args());
                }
                Level::Trace => {
                    println!("[{}] {}", "TRACE".bright_black(), record.args());
                }
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: ColoredLogger = ColoredLogger;

#[ostd::main]
fn kernel_main() {
    // 注册自定义彩色日志记录器
    ostd::logger::inject_logger(&LOGGER);

    println!("Hello world from guest kernel!");

    // 练习 2.1: 使用 5 种日志等级输出内容
    error!("This is an error message (Red)");
    warn!("This is a warning message (Yellow)");
    info!("This is an info message (Cyan)");
    debug!("This is a debug message (Blue)");
    trace!("This is a trace message (Grey)");

    // 保持循环以防止立即崩溃（可选，但推荐用于观察输出）
    // loop {}
}

// 练习 1: 编写 ktest 测试用例
#[cfg(ktest)]
mod tests {
    use super::*;

    #[ktest]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }

    #[ktest]
    fn test_log_levels() {
        // 在测试中也可以使用日志
        info!("Running kernel mode unit test...");
        assert!(true);
    }
}
