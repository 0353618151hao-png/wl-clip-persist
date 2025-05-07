mod async_io;
mod logger;
mod protocol_traits;
mod settings;
mod signal;
mod states;
mod wayland;
mod zeroize;

use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::logger::log_default_target;
use crate::settings::{NumberOrInf, Settings, get_settings};
use crate::signal::shutdown_token;
use crate::states::WaylandError;
use crate::wayland::GracefulWaylandShutdown;

/// The exit code
struct ExitCode(i32);

// One worker thread for writing the clipboard data
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    // Explicit scope to drop values before exit call
    let exit_code = {
        let settings = get_settings();
        let task_tracker = TaskTracker::new();
        let shutdown_token = shutdown_token();

        let exit_code = run(settings, &task_tracker, shutdown_token.child_token()).await;

        // Manually cancel all clipboard write tasks in case no shutdown request was received,
        // but an error occurred
        shutdown_token.cancel();

        // Wait until all clipboard write tasks have been finished
        task_tracker.close();
        task_tracker.wait().await;

        exit_code
    };

    std::process::exit(exit_code.0);
}

async fn run(settings: Settings, task_tracker: &TaskTracker, shutdown_token: CancellationToken) -> ExitCode {
    let mut is_reconnect = false;
    let mut connection_tries = 0;

    loop {
        match wayland::run(task_tracker, shutdown_token.child_token(), &settings, is_reconnect).await {
            Ok(GracefulWaylandShutdown) => return ExitCode(0),
            Err(WaylandError::NoDataControlManagerGlobalFound) => return ExitCode(1),
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
                        return ExitCode(1);
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

                        tokio::select! {
                            () = tokio::time::sleep(settings.reconnect_delay) => {},
                            () = shutdown_token.cancelled() => return ExitCode(0),
                        }
                    }
                } else {
                    log::error!(
                        target: log_default_target(),
                        "Failed to connect to wayland server: {}",
                        err,
                    );
                    return ExitCode(1);
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
                    return ExitCode(1);
                }
            }
            Err(WaylandError::DataControlManagerGlobalRemoved) => {
                if matches!(settings.reconnect_tries, NumberOrInf::Inf | NumberOrInf::Number(1..)) {
                    is_reconnect = true;
                    connection_tries = 0;

                    match settings.reconnect_tries {
                        NumberOrInf::Number(reconnect_tries) => {
                            log::error!(
                                target: log_default_target(),
                                "Wayland error: data control manager global was removed\nAttempt {}/{} to reconnect will start immediately...",
                                connection_tries + 1,
                                reconnect_tries,
                            );
                        }
                        NumberOrInf::Inf => {
                            log::error!(
                                target: log_default_target(),
                                "Wayland error: data control manager global was removed\nAttempt to reconnect will start immediately...",
                            );
                        }
                    }
                } else {
                    log::error!(
                        target: log_default_target(),
                        "Wayland error: data control manager global was removed",
                    );
                    return ExitCode(1);
                }
            }
        }
    }
}
