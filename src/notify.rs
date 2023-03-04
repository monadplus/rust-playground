#[cfg(test)]
mod tests {
    use notify::*;
    use serde::{Deserialize, Serialize};
    use std::path::Path;
    use std::result::Result as StdResult;
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    struct Config {
        host: String,
        port: u16,
    }

    fn load_config(path: &str) -> StdResult<Config, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    fn write_config(config: &Config, path: &str) -> StdResult<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, config)?;
        Ok(())
    }

    fn watch<P, F>(path: P, on_change: F) -> Result<INotifyWatcher>
    where
        P: AsRef<Path>,
        F: Fn() + Send + 'static,
    {
        let mut watcher = recommended_watcher(move |event: Result<Event>| {
            let event = event.unwrap();
            if event.kind.is_modify() {
                on_change()
            } else {
                dbg!(event);
            }
        })?;
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        Ok(watcher)
    }

    #[test]
    fn notify_test() -> StdResult<(), Box<dyn std::error::Error>> {
        const CONFIG_PATH: &str = "./config/example.json";
        let init_config: Config = load_config(CONFIG_PATH)?;
        let init_config_clone: Config = init_config.clone();
        let config = Arc::new(Mutex::new(init_config.clone()));
        let config_clone = config.clone();

        // You need to keep the watcher alive for this to work.
        let _watcher = watch(CONFIG_PATH, move || match load_config(CONFIG_PATH) {
            Ok(new_config) => {
                *config_clone.lock().unwrap() = new_config;
                println!("New config loaded.");
            }
            Err(err) => eprintln!("Error loading the config: {err:?}"),
        })?;

        // Update the config file.
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(100));
            let mut new_config = init_config_clone.clone();
            new_config.host = "google.com".to_string();
            write_config(&new_config, CONFIG_PATH).unwrap();
        });

        // You could also use a channel.
        while *config.lock().unwrap() == init_config {
            println!("Waiting for host != localhost...");
            thread::sleep(std::time::Duration::from_secs(1))
        }

        // Reset the file to make test idempotent
        write_config(&init_config, CONFIG_PATH)?;

        Ok(())
    }
}
