use std::num::NonZeroU64;
use std::str::FromStr;
use std::time::Duration;

use clap::builder::NonEmptyStringValueParser;
use clap::{Arg, ArgAction, Command, arg, crate_description, crate_name, crate_version, value_parser};
use fancy_regex::Regex;

use crate::logger::{self, log_default_target};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub(crate) enum ClipboardType {
    Regular,
    Primary,
    Both,
}

impl ClipboardType {
    /// Whether the primary selection is activated.
    pub(crate) const fn primary(&self) -> bool {
        match self {
            Self::Primary | Self::Both => true,
            Self::Regular => false,
        }
    }

    /// Whether the regular selection is activated.
    pub(crate) const fn regular(&self) -> bool {
        match self {
            Self::Regular | Self::Both => true,
            Self::Primary => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NumberOrInf<T: Clone + Copy> {
    Number(T),
    Inf,
}

impl<T: Clone + Copy + FromStr> FromStr for NumberOrInf<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("inf") {
            Ok(NumberOrInf::Inf)
        } else {
            s.parse::<T>().map(NumberOrInf::Number)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum ClipboardProtocol {
    ExtDataControlV1,
    WlrDataControlUnstableV1,
}

/// The settings the program was started with.
#[derive(Debug, Clone)]
pub(crate) struct Settings {
    /// The clipboard types which are activated.
    pub(crate) clipboard_type: ClipboardType,
    /// The write timeout when writing the clipboard data to other clients.
    pub(crate) write_timeout: Duration,
    /// Whether selection events should be ignored when at least one error occurred.
    pub(crate) ignore_selection_event_on_error: bool,
    /// The selection size limit in bytes, or [`None`] if no limit.
    pub(crate) selection_size_limit_bytes: Option<NonZeroU64>,
    /// If [`None`], the selection events should not be filtered by a [`Regex`].
    /// Otherwise, all mime types have to match the regex for it to be not ignored.
    pub(crate) all_mime_type_regex: Option<Regex>,
    /// The number of times a reconnect to the Wayland server should be tried after an error.
    pub(crate) reconnect_tries: NumberOrInf<u64>,
    /// The delay between two reconnect tries to the Wayland server.
    pub(crate) reconnect_delay: Duration,
    /// If [`Some`], force this specific clipboard protocol, otherwise this is handled automatically.
    pub(crate) force_protocol: Option<ClipboardProtocol>,
    /// If `true`, do not ignore specific mime types as workaround.
    pub(crate) disable_workaround_ignore_mime_types: bool,
    /// If `true`, do not request mime types in specific order as workaround.
    pub(crate) disable_workaround_order_mime_type_requests: bool,
}

/// Get the settings for the program.
pub(crate) fn get_settings() -> Settings {
    let mut command = Command::new(crate_name!()).version(crate_version!());
    let description = crate_description!();

    if !description.is_empty() {
        command = command.about(description);
    }

    let matches = command
        .arg(
            arg!(
                -c --clipboard <TYPE> "The clipboard type to operate on"
            )
            .required(true)
            .value_parser(value_parser!(ClipboardType)),
        )
        .arg(
            arg!(
                -w --"write-timeout" <MILLISECONDS> "Timeout for trying to send the current clipboard to other programs"
            )
            .required(false)
            .value_parser(clap::value_parser!(u64).range(1..=i32::MAX as u64))
            .default_value("3000"),
        )
        .arg(
            arg!(
                -e --"ignore-event-on-error" "Only handle selection events where no error occurred"
            )
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                -l --"selection-size-limit" <BYTES> "Only handle selection events whose total data size does not exceed the size limit"
            )
            .required(false)
            .value_parser(clap::value_parser!(NonZeroU64)),
        )
        .arg(
            arg!(
                -f --"all-mime-type-regex" <REGEX> "Only handle selection events where all offered MIME types have a match for the regex"
            )
            .required(false)
            .value_parser(NonEmptyStringValueParser::new()),
        )
        .arg(
            Arg::new("reconnect-tries")
            .long("reconnect-tries")
            .value_name("NUMBER|INF")
            .help("Limit the number of tries to reconnect to the Wayland server after a Wayland error")
            .required(false)
            .value_parser(clap::value_parser!(NumberOrInf<u64>))
            .default_value("0"),
        )
        .arg(
            arg!(
                --"reconnect-delay" <MILLISECONDS> "The delay between reconnect tries to the Wayland server"
            )
            .required(false)
            .value_parser(clap::value_parser!(u64).range(0..=i32::MAX as u64))
            .default_value("100"),
        )
        .arg(
            arg!(
                --"disable-timestamps" "Do not show timestamps in the log messages"
            )
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                --"force-protocol" <PROTOCOL> "Force specific clipboard protocol to be used"
            )
            .required(false)
            .hide(true)
            .value_parser(value_parser!(ClipboardProtocol)),
        )
        .arg(
            arg!(
                --"disable-workaround-ignore-mime-types" "Do not ignore specific mime types as workaround"
            )
            .required(false)
            .hide(true)
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                --"disable-workaround-order-mime-type-requests" "Do not request mime types in specific order as workaround"
            )
            .required(false)
            .hide(true)
            .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Initialize the logger here, because log is used to inform about invalid settings
    let disable_timestamps = matches.get_flag("disable-timestamps");
    logger::init_logger(!disable_timestamps);

    let clipboard_type = *matches.get_one::<ClipboardType>("clipboard").unwrap();
    let write_timeout = Duration::from_millis(*matches.get_one::<u64>("write-timeout").unwrap());
    let ignore_selection_event_on_error = matches.get_flag("ignore-event-on-error");
    let selection_size_limit_bytes = matches.get_one::<NonZeroU64>("selection-size-limit").copied();
    let all_mime_type_regex = matches
        .get_one::<String>("all-mime-type-regex")
        .map(|s| match Regex::new(s) {
            Ok(regex) => regex,
            Err(err) => {
                log::error!(
                    target: log_default_target(),
                    "Failed to parse the mime type regex: {}",
                    err
                );
                std::process::exit(1);
            }
        });
    let reconnect_tries = *matches.get_one::<NumberOrInf<u64>>("reconnect-tries").unwrap();
    let reconnect_delay = Duration::from_millis(*matches.get_one::<u64>("reconnect-delay").unwrap());
    let force_protocol = matches.get_one::<ClipboardProtocol>("force-protocol").copied();
    let disable_workaround_ignore_mime_types = matches.get_flag("disable-workaround-ignore-mime-types");
    let disable_workaround_order_mime_type_requests = matches.get_flag("disable-workaround-order-mime-type-requests");

    Settings {
        clipboard_type,
        write_timeout,
        ignore_selection_event_on_error,
        selection_size_limit_bytes,
        all_mime_type_regex,
        reconnect_tries,
        reconnect_delay,
        force_protocol,
        disable_workaround_ignore_mime_types,
        disable_workaround_order_mime_type_requests,
    }
}
