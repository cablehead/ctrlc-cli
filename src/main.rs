use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })
    .expect("Error setting Ctrl-C handler");

    eprintln!("Waiting for Ctrl-C...");
    match rx.recv() {
        Ok(()) => eprintln!("CTRL-C received."),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("Exiting...");
}
