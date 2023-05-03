//! Allow to listen to common OS exit signals.
use anyhow::{Context, Result};
use log::info;
use tokio::signal::unix::{signal, Signal, SignalKind};

/// An exit signal handler
#[derive(Debug)]
pub(crate) struct ExitSignalHandler {
    sigterm: Signal,
    sigint: Signal,
    sigpipe: Signal,
    sigquit: Signal,
}

impl ExitSignalHandler {
    /// Returns a new exit handler
    pub(crate) fn new() -> Result<ExitSignalHandler> {
        Ok(ExitSignalHandler {
            sigterm: signal(SignalKind::terminate()).context("Cannot register SIGTERM handler")?,
            sigint: signal(SignalKind::interrupt()).context("Cannot register SIGINT handler")?,
            sigpipe: signal(SignalKind::pipe()).context("Cannot register SIGPIPE handler")?,
            sigquit: signal(SignalKind::quit()).context("Cannot register SIGQUIT handler")?,
        })
    }
    /// Receives one of the registered exit signals.
    pub(crate) async fn recv(&mut self) {
        tokio::select! {
            _ = self.sigterm.recv() => { info!("SIGTERM received") }
            _ = self.sigint.recv() => { info!("SIGINT received") }
            _ = self.sigpipe.recv() => { info!("SIGPIPE received") }
            _ = self.sigquit.recv() => { info!("SIGQUIT received") }
        };
    }
}
