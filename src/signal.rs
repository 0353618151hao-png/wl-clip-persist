use tokio::signal::unix::SignalKind;
use tokio_util::sync::CancellationToken;

const SIGINT_EXIT_CODE: i32 = 128 + libc::SIGINT;
const SIGTERM_EXIT_CODE: i32 = 128 + libc::SIGTERM;

/// Creates a shutdown token that gets triggered once a shutdown request is received.
/// This function also handles an incoming second shutdown request resulting in a
/// forceful exit of the program.
pub(crate) fn shutdown_token() -> CancellationToken {
    let token = CancellationToken::new();
    let child_token = token.child_token();

    tokio::spawn(async move {
        match (
            tokio::signal::unix::signal(SignalKind::interrupt()),
            tokio::signal::unix::signal(SignalKind::terminate()),
        ) {
            (Ok(mut sigint), Ok(mut sigterm)) => {
                // Initiate graceful shutdown via token notification on first request
                tokio::select! {
                    _ = sigint.recv() => {},
                    _ = sigterm.recv() => {},
                };
                token.cancel();

                // Force exit on second request
                enum ReceivedSignal {
                    SigInt,
                    SigTerm,
                }
                let received_signal = tokio::select! {
                    _ = sigint.recv() => ReceivedSignal::SigInt,
                    _ = sigterm.recv() => ReceivedSignal::SigTerm,
                };
                std::process::exit(match received_signal {
                    ReceivedSignal::SigInt => SIGINT_EXIT_CODE,
                    ReceivedSignal::SigTerm => SIGTERM_EXIT_CODE,
                });
            }
            (Ok(mut sigint), Err(sigterm_error)) => {
                log::debug!("Failed to setup SIGTERM listener: {sigterm_error}");

                // Initiate graceful shutdown via token notification on first request
                sigint.recv().await;
                token.cancel();

                // Force exit on second request
                sigint.recv().await;
                std::process::exit(SIGINT_EXIT_CODE);
            }
            (Err(sigint_error), Ok(mut sigterm)) => {
                log::debug!("Failed to setup SIGINT listener: {sigint_error}");

                // Initiate graceful shutdown via token notification on first request
                sigterm.recv().await;
                token.cancel();

                // Force exit on second request
                sigterm.recv().await;
                std::process::exit(SIGTERM_EXIT_CODE);
            }
            (Err(sigint_error), Err(sigterm_error)) => {
                log::debug!("Failed to setup SIGINT listener: {sigint_error}");
                log::debug!("Failed to setup SIGTERM listener: {sigterm_error}");
            }
        };
    });

    child_token
}
