#[cfg(test)]
mod tests {
    use signal_hook::iterator::Signals;
    use signal_hook::{consts::*, low_level};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    #[ignore = "requires to send a SIGTERM manually"]
    fn signal_hook_test() -> Result<(), Box<dyn std::error::Error>> {
        let term = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
        while !term.load(Ordering::Relaxed) {
            println!("Sleeping for 100 ms...");
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    #[test]
    #[ignore = "does not terminate"]
    fn signal_hook_test2() -> Result<(), Box<dyn std::error::Error>> {
        const SIGNALS: &[std::ffi::c_int] = &[
            SIGTERM, SIGQUIT, SIGINT, SIGTSTP, SIGWINCH, SIGHUP, SIGCHLD, SIGCONT,
        ];
        let mut sigs = Signals::new(SIGNALS)?;
        for signal in &mut sigs {
            eprintln!("Received signal {signal:?}");
            low_level::emulate_default_handler(signal)?;
        }
        Ok(())
    }
}
