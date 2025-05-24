use std::ffi::CStr;

use chrono::Local;
use env_logger::Builder;
use env_logger::fmt::style::{AnsiColor, Color, Style};
use log::{Level, LevelFilter};

use crate::protocol_traits::DataControlV1;
use crate::states::MimeTypesWithData;

pub(crate) fn init_logger(with_timestamps: bool) {
    custom_logger_builder("%Y-%m-%dT%H:%M:%S.%3f", with_timestamps)
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init()
}

fn custom_logger_builder(fmt: &'static str, with_timestamps: bool) -> Builder {
    let mut builder = Builder::new();

    builder.format(move |f, record| {
        use std::io::Write as _;

        let target = record.target();

        if !target.starts_with(clap::crate_name!()) {
            return Ok(());
        }

        let time = with_timestamps.then(|| Local::now().format(fmt));
        let level = record.level();
        let level_text = level_text(&level);
        let level_style = level_style(&level);
        let display_target = target.strip_prefix(concat!(clap::crate_name!(), ' ')).unwrap_or("main");
        let message = record.args().to_string().lines().collect::<Vec<_>>().join("\n  ");

        if let Some(time) = time {
            writeln!(
                f,
                "{} {}{}{} {} > {}",
                time,
                level_style.render(),
                level_text,
                level_style.render_reset(),
                display_target,
                message
            )
        } else {
            writeln!(
                f,
                "{}{}{} {} > {}",
                level_style.render(),
                level_text,
                level_style.render_reset(),
                display_target,
                message
            )
        }
    });

    builder
}

fn level_text(level: &Level) -> &'static str {
    match level {
        Level::Trace => "TRACE",
        Level::Debug => "DEBUG",
        Level::Info => "INFO ",
        Level::Warn => "WARN ",
        Level::Error => "ERROR",
    }
}

fn level_style(level: &Level) -> Style {
    Style::new().fg_color(Some(match level {
        Level::Trace => Color::Ansi(AnsiColor::Magenta),
        Level::Debug => Color::Ansi(AnsiColor::Blue),
        Level::Info => Color::Ansi(AnsiColor::Green),
        Level::Warn => Color::Ansi(AnsiColor::Yellow),
        Level::Error => Color::Ansi(AnsiColor::Red),
    }))
}

/// Returns a formatted target for logging purposes.
pub(crate) const fn log_default_target() -> &'static str {
    clap::crate_name!()
}

/// Returns a formatted target for logging purposes.
pub(crate) fn log_seat_target(seat_name: u32) -> String {
    format!("{} Seat {}", clap::crate_name!(), seat_name)
}

/// If title_case is false, lower case is used instead of title case.
pub(crate) const fn get_clipboard_type_str(is_primary_clipboard: bool, title_case: bool) -> &'static str {
    match (is_primary_clipboard, title_case) {
        (true, true) => "Primary",
        (true, false) => "primary",
        (false, true) => "Regular",
        (false, false) => "regular",
    }
}

/// Logs the successfully read data for text mime types. If the data is too long, it is truncated.
pub(crate) fn log_text_data<DataControl: DataControlV1>(mime_types_with_data: &MimeTypesWithData<'_, DataControl>) {
    const TEXT_MIME_TYPES: &[&CStr] = &[
        c"text/plain;charset=utf-8",
        c"text/plain",
        c"UTF8_STRING",
        c"COMPOUND_TEXT",
        c"STRING",
        c"TEXT",
    ];
    const TRUNCATED_DATA_COUNT: usize = 30;

    for &text_mime_type in TEXT_MIME_TYPES {
        let Some(data) = mime_types_with_data.data.get(&Box::from(text_mime_type)) else {
            continue;
        };

        // Truncate data if necessary
        let mut truncated_data = String::with_capacity(
            const {
                1 // [
                + TRUNCATED_DATA_COUNT * 3 // "255" or "..."
                + (TRUNCATED_DATA_COUNT - 1) * 2 // ", "
                + 1 // ]
            },
        );
        truncated_data.push('[');
        if data.len() <= TRUNCATED_DATA_COUNT {
            let mut is_first = true;
            for x in data.iter() {
                if !is_first {
                    truncated_data.push_str(", ");
                }
                truncated_data.push_str(&x.to_string());
                is_first = false;
            }
        } else {
            let mut is_first = true;
            for x in data.iter().take(TRUNCATED_DATA_COUNT - 1) {
                if !is_first {
                    truncated_data.push_str(", ");
                }
                truncated_data.push_str(&x.to_string());
                is_first = false;
            }
            truncated_data.push_str(", ...");
        };
        truncated_data.push(']');

        // Log data
        log::trace!(
            target: &log_seat_target(mime_types_with_data.seat_name),
            "{} clipboard successfully read data for mime type {:?}: {}",
            mime_types_with_data.selection_type.get_clipboard_type_str(true),
            text_mime_type,
            truncated_data,
        );
    }
}
