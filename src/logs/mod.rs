mod local;

use std::io::Write;

use chrono::Utc;
use env_logger::{
    builder,
    fmt::{Color, Style},
};
use log::{Level, LevelFilter};

pub fn bootstrap_logs() {
    builder()
        .format(|buf, record| {
            let mut level_style = buf.style();
            let level = record.level();
            set_level_color(&mut level_style, &level);

            let date = Utc::now();

            let mut target_style = buf.style();
            target_style.set_color(Color::Cyan);

            writeln!(
                buf,
                "[{}] [{}] [{}]: {}",
                date.format("%Y-%m-%d %H:%M:%S").to_string(),
                target_style.value(record.target()),
                level_style.value(level),
                record.args()
            )
        })
        .filter_level(LevelFilter::Off)
        .filter_module("poise", LevelFilter::Warn)
        .filter_module("serenity", LevelFilter::Warn)
        .filter_module("sophy_bot", LevelFilter::Debug)
        .filter_module("sqlx", LevelFilter::Trace)
        .init();
}

fn set_level_color(style: &mut Style, level: &Level) {
    match level {
        Level::Error => style.set_color(Color::Red).set_bold(true),
        Level::Warn => style.set_color(Color::Yellow),
        Level::Info => style.set_color(Color::Green),
        Level::Debug => style.set_color(Color::Blue),
        Level::Trace => style.set_color(Color::Magenta),
    };
}
