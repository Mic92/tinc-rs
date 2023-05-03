use clap::Parser;
use anyhow::{Context, Result, bail};
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use crate::config::read_server_config;
use tokio::runtime::Runtime;

mod config;
mod connection;
mod protocol;
mod exit_signal_handler;

// Usage: tincd [option]...
//
//  -c, --config=DIR              Read configuration options from DIR.
//  -D, --no-detach               Don't fork and detach.
//  -d, --debug[=LEVEL]           Increase debug level or set it to LEVEL.
//  -n, --net=NETNAME             Connect to net NETNAME.
//  -L, --mlock                   Lock tinc into main memory.
//      --logfile[=FILENAME]      Write log entries to a logfile.
//  -s  --syslog                  Use syslog instead of stderr with --no-detach.
//      --pidfile=FILENAME        Write PID and control socket cookie to FILENAME.
//      --bypass-security         Disables meta protocol security, for debugging.
//  -o, --option[HOST.]KEY=VALUE  Set global/host configuration value.
//  -R, --chroot                  chroot to NET dir at startup.
//  -U, --user=USER               setuid to given USER at startup.
//      --help                    Display this help and exit.
//      --version                 Output version information and exit.
//
//Report bugs to tinc@tinc-vpn.org.


#[derive(Parser, Debug)] // requires `derive` feature
struct Args {
    /// Read configuration options from DIR.
    #[arg(long, short = 'c', default_value = "/etc/tinc")]
    config: PathBuf,

    /// Connect to net NETNAME.
    #[arg(long, short = 'n', default_value = ".")]
    network: String,

    /// Output version information and exit.
    #[arg(long)]
    version: bool,
}

// control characters are not allowed
// cannot contain a / or \
fn check_netname(val: &str) -> Result<()> {
    if val.is_empty() {
        bail!("Empty netname");
    }
    if val.contains(|c: char| 
                    c == '/' || 
                    c == '\\' || 
                    c.is_ascii_control()) {
        bail!("Invalid netname '{}'", val);
    }
    Ok(())
}

fn parse_args() -> Result<Args> {
    let args = Args::parse();
    if args.version {
        println!("tinc-rs 0.0.1");
        std::process::exit(0);
    }
    check_netname(&args.network)?;
    Ok(args)
}

async fn main_loop() -> Result<()> {
    let mut exit_signal_handler = exit_signal_handler::ExitSignalHandler::new().context("Cannot create exit signal handler")?;
    loop {
        tokio::select! {
            _ = exit_signal_handler.recv() => {
                break;
            }
        };
    };
    Ok(())
}

//fn main_loop(void) {
//	last_periodic_run_time = now;
//	timeout_add(&pingtimer, timeout_handler, &pingtimer, &(struct timeval) {
//		pingtimeout, jitter()
//	});
//	timeout_add(&periodictimer, periodic_handler, &periodictimer, &(struct timeval) {
//		0, 0
//	});
//
//#ifndef HAVE_WINDOWS
//	signal_t sighup = {0};
//	signal_t sigterm = {0};
//	signal_t sigquit = {0};
//	signal_t sigint = {0};
//	signal_t sigalrm = {0};
//
//	signal_add(&sighup, sighup_handler, &sighup, SIGHUP);
//	signal_add(&sigterm, sigterm_handler, &sigterm, SIGTERM);
//	signal_add(&sigquit, sigterm_handler, &sigquit, SIGQUIT);
//	signal_add(&sigint, sigterm_handler, &sigint, SIGINT);
//	signal_add(&sigalrm, sigalrm_handler, &sigalrm, SIGALRM);
//#endif
//
//	if(!event_loop()) {
//		logger(DEBUG_ALWAYS, LOG_ERR, "Error while waiting for input: %s", sockstrerror(sockerrno));
//		return 1;
//	}
//
//#ifndef HAVE_WINDOWS
//	signal_del(&sighup);
//	signal_del(&sigterm);
//	signal_del(&sigquit);
//	signal_del(&sigint);
//	signal_del(&sigalrm);
//#endif
//
//	timeout_del(&periodictimer);
//	timeout_del(&pingtimer);
//
//	return 0;
//}


fn tincd() -> Result<()> {
    let args = parse_args()?;
    let path = Path::new(&args.config).join(&args.network);
    let config = read_server_config(&path)?;
    let rt = Runtime::new().context("Cannot create tokio runtime")?;
    rt.block_on(main_loop())?;
    Ok(())
}

fn main() {
    if let Err(e) = tincd() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
