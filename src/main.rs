use signal_hook::iterator::Signals;
use std::io::Error;

fn handle_signals() -> Result<(), Error> {
    let mut signals = Signals::new(&[
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGPIPE,
        signal_hook::consts::SIGHUP,
    ])?;

    eprintln!("waiting for signals...");
    for signal in signals.forever() {
        eprintln!(
            "received {}",
            signal_hook::low_level::signal_name(signal).unwrap_or(&format!("unknown: {}", signal))
        );
        match signal {
            signal_hook::consts::SIGTERM | signal_hook::consts::SIGINT => {
                break;
            }
            signal_hook::consts::SIGHUP => (),
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn main() {
    eprintln!("ctrlc pid: {}", std::process::id());
    if let Err(e) = handle_signals() {
        eprintln!("error handling signals: {}", e);
    }
}
