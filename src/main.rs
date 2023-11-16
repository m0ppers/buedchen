mod client;

use clap::{Parser, ValueEnum};
use tracing::info;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Backend {
    Auto,
    Winit,
    Tty,
}

/// A wayland compositor that implements a full screen kiosk shell
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long, value_enum, default_value_t = Backend::Auto)]
    backend: Backend,

    #[arg(last(true), required(true))]
    executable: Vec<String>,
}

fn run_winit(executable: &[String]) {
    tracing::info!("Starting buedchen with winit backend");
    buedchen::winit::run_winit(executable);
}

fn run_udev(executable: &[String]) {
    tracing::info!("Starting buedchen on a tty using udev");
    buedchen::udev::run_udev(executable);
}

fn main() {
    profiling::register_thread!("Main Thread");

    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt()
            .compact()
            .with_env_filter(env_filter)
            .init();
    } else {
        tracing_subscriber::fmt().compact().init();
    }

    let cli = Cli::parse();
    match cli.backend {
        Backend::Auto => match std::env::var("WAYLAND_DISPLAY") {
            Ok(_) => run_winit(&cli.executable),
            Err(_) => run_udev(&cli.executable),
        },
        Backend::Winit => run_winit(&cli.executable),
        Backend::Tty => run_udev(&cli.executable),
    }
}
