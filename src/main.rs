use clap::Parser;

/// A wayland compositor that implements a full screen kiosk shell
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(last(true), required(true))]
    executable: Vec<String>,
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
    run_udev(&cli.executable);
}
