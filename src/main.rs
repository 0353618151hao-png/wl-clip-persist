mod async_io;
mod logger;
mod protocol_traits;
mod settings;
mod states;
mod wayland;

use std::time::Duration;

use settings::NumberOrInf;

use crate::logger::log_default_target;
use crate::settings::get_settings;
use crate::states::WaylandError;

// One worker thread for writing the clipboard data
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let settings = get_settings();
    let mut is_reconnect = false;
    let mut connection_tries = 0;

    loop {
        match wayland::run(settings.clone(), is_reconnect).await {
            Err(WaylandError::ConnectError(err)) => {
                if is_reconnect {
                    if matches!(settings.reconnect_tries, NumberOrInf::Number(_)) {
                        connection_tries += 1;
                    }

                    if NumberOrInf::Number(connection_tries) == settings.reconnect_tries {
                        log::error!(
                            target: log_default_target(),
                            "Wayland connect error: {}",
                            err,
                        );
                        std::process::exit(1);
                    } else if settings.reconnect_delay == Duration::ZERO {
                        match settings.reconnect_tries {
                            NumberOrInf::Number(reconnect_tries) => {
                                log::error!(
                                    target: log_default_target(),
                                    "Wayland connect error: {}\nAttempt {}/{} to reconnect will start immediately...",
                                    err,
                                    connection_tries + 1,
                                    reconnect_tries,
                                );
                            }
                            NumberOrInf::Inf => {
                                log::error!(
                                    target: log_default_target(),
                                    "Wayland connect error: {}\nAttempt to reconnect will start immediately...",
                                    err,
                                );
                            }
                        }
                    } else {
                        match settings.reconnect_tries {
                            NumberOrInf::Number(reconnect_tries) => {
                                log::error!(
                                    target: log_default_target(),
                                    "Wayland connect error: {}\nAttempt {}/{} to reconnect in {} ms...",
                                    err,
                                    connection_tries + 1,
                                    reconnect_tries,
                                    settings.reconnect_delay.as_millis().max(1)
                                );
                            }
                            NumberOrInf::Inf => {
                                log::error!(
                                    target: log_default_target(),
                                    "Wayland connect error: {}\nAttempt to reconnect in {} ms...",
                                    err,
                                    settings.reconnect_delay.as_millis().max(1)
                                );
                            }
                        }

                        tokio::time::sleep(settings.reconnect_delay).await;
                    }
                } else {
                    log::error!(
                        target: log_default_target(),
                        "Failed to connect to wayland server: {}",
                        err,
                    );
                    std::process::exit(1);
                }
            }
            Err(WaylandError::IoError(err)) => {
                if matches!(settings.reconnect_tries, NumberOrInf::Inf | NumberOrInf::Number(1..)) {
                    is_reconnect = true;
                    connection_tries = 0;

                    match settings.reconnect_tries {
                        NumberOrInf::Number(reconnect_tries) => {
                            log::error!(
                                target: log_default_target(),
                                "Wayland IO error: {}\nAttempt {}/{} to reconnect will start immediately...",
                                err,
                                connection_tries + 1,
                                reconnect_tries,
                            );
                        }
                        NumberOrInf::Inf => {
                            log::error!(
                                target: log_default_target(),
                                "Wayland IO error: {}\nAttempt to reconnect will start immediately...",
                                err,
                            );
                        }
                    }
                } else {
                    log::error!(
                        target: log_default_target(),
                        "Wayland IO error: {}",
                        err,
                    );
                    std::process::exit(1);
                }
            }
        }
    }
}
