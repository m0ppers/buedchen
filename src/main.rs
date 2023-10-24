static POSSIBLE_BACKENDS: &[&str] = &[
    "--winit : Run trinkhalle as a X11 or Wayland client using winit.",
    "--tty-udev : Run trinkhalle as a tty udev client (requires root if without logind).",
];

fn main() {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt()
            .compact()
            .with_env_filter(env_filter)
            .init();
    } else {
        tracing_subscriber::fmt().compact().init();
    }

    profiling::register_thread!("Main Thread");

    let arg = ::std::env::args().nth(1);
    match arg.as_ref().map(|s| &s[..]) {
        Some("--winit") => {
            tracing::info!("Starting trinkhalle with winit backend");
            trinkhalle::winit::run_winit();
        }
        Some("--tty-udev") => {
            tracing::info!("Starting trinkhalle on a tty using udev");
            trinkhalle::udev::run_udev();
        }
        Some(other) => {
            tracing::error!("Unknown backend: {}", other);
        }
        None => {
            println!("USAGE: trinkhalle --backend");
            println!();
            println!("Possible backends are:");
            for b in POSSIBLE_BACKENDS {
                println!("\t{}", b);
            }
        }
    }
}
