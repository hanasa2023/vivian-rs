//! 初始化和配置自定义格式的日志记录器。
//!
//! 本模块提供了一个函数 [`init_logger`]，用于设置一个全局的日志记录器。
//! 这个记录器会以特定的彩色格式输出日志，包括时间戳、日志级别、日志来源模块（target）以及日志消息本身。
//! 日志级别可以通过环境变量 `RUST_LOG` 或函数参数进行配置。

use ansi_term::Colour;
use chrono::Local;
use log::{Level, LevelFilter};
use pretty_env_logger::env_logger::fmt::Color as EnvColor;
use pretty_env_logger::formatted_builder;
use std::env;
use std::io::Write;

/// 初始化自定义格式的日志记录器。
///
/// 此函数会配置并初始化一个全局日志记录器，该记录器将日志消息格式化为：
/// `[MM-DD HH:MM:SS] [级别] [模块路径] > 消息内容`
///
/// 日志级别根据其严重性以不同颜色显示：
/// - ERROR: 红色
/// - WARN:  黄色
/// - INFO:  绿色
/// - DEBUG: 青色 (Cyan)
/// - TRACE: 紫色
///
/// 时间戳以青色显示，模块路径以洋红色显示。
///
/// 日志过滤级别可以通过以下方式设置（优先级从高到低）：
/// 1. 环境变量 `RUST_LOG` (如果已设置)。
/// 2. `filter` 参数 (如果提供了 `Some(LevelFilter)`)。
/// 3. 默认级别 `LevelFilter::Info` (如果 `RUST_LOG` 未设置且 `filter` 参数为 `None`)。
///
/// # 参数
/// * `filter`: 可选的日志级别过滤器 (`LevelFilter`)。如果为 `None` 且 `RUST_LOG` 环境变量未设置，
///             则默认使用 `LevelFilter::Info`。
pub fn init_logger(filter: Option<LevelFilter>) {
    let mut builder = formatted_builder();

    builder.format(|buf, record| {
        let level_str = match record.level() {
            Level::Error => Colour::Red.paint("[ERROR]").to_string(),
            Level::Warn => Colour::Yellow.paint("[WARN]").to_string(),
            Level::Info => Colour::Green.paint("[INFO]").to_string(),
            Level::Debug => Colour::Cyan.paint("[DEBUG]").to_string(),
            Level::Trace => Colour::Purple.paint("[TRACE]").to_string(),
        };

        let time_str = Local::now().format("%m-%d %H:%M:%S");
        let target_str = record.target();

        writeln!(
            buf,
            "{} {} {} > {}",
            buf.style().set_color(EnvColor::Cyan).value(time_str),
            level_str,
            buf.style().set_color(EnvColor::Magenta).value(target_str),
            record.args()
        )
    });

    if env::var("RUST_LOG").is_err() {
        if let Some(f) = filter {
            builder.filter(None, f);
        } else {
            builder.filter(None, LevelFilter::Info);
        }
    }

    builder.init();
}
