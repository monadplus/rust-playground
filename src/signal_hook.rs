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

// #[cfg(test)]
mod tests_async {
    use std::{process, time::Duration};

    use signal_hook::consts::signal::*;
    use signal_hook_tokio::Signals;

    use futures::stream::StreamExt;
    use std::process::Command;

    async fn handle_signals(mut signals: Signals) {
        while let Some(signal) = signals.next().await {
            match signal {
                SIGHUP => {
                    // Reload configuration
                    // Reopen the log file
                }
                SIGTERM | SIGINT | SIGQUIT => {
                    // Shutdown the system;
                }
                _ => unreachable!(),
            }
            println!("Received signal: {signal:?}");
        }
    }

    async fn main_thread() -> Result<(), Box<dyn std::error::Error>> {
        for i in 1..=20 {
            if i % 10 == 0 {
                Command::new("kill")
                    .args(["-s", "TERM", &process::id().to_string()])
                    .spawn()?;
            }
            println!("Sleeping for 100ms..");
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok(())
    }

    #[tokio::test]
    async fn signal_hook_tokio_test() -> Result<(), Box<dyn std::error::Error>> {
        let signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
        let handle = signals.handle();

        let signals_task = tokio::spawn(handle_signals(signals));

        main_thread().await?;

        handle.close();
        signals_task.await?;

        Ok(())
    }
}
