#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use tracing::{debug, debug_span, info, info_span, instrument, span, trace, warn, Level};
    use tracing_futures::Instrument;
    use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

    #[instrument]
    fn suggest_band(name: String) -> String {
        debug!(suggested = name, "Suggesting a band.");
        String::from(format!("{} Fighters", name))
    }

    // $ RUST_LOG=trace cargo test tracing_test
    #[test]
    fn tracing_test() {
        let _subscriber = tracing_subscriber::fmt()
            .compact()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let rec_depth = 1;
        let span = span!(Level::INFO, "get_band_rec", %rec_depth);
        let _enter = span.enter();

        let band = suggest_band("Foo".to_string());
        info!(message = "Got a recommendation!", %band);
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    struct MyStruct {
        a: u8,
        b: Vec<u8>,
    }

    #[instrument]
    async fn suggest_band_async(name: String) -> String {
        debug!(suggested = name, "Suggesting a band.");
        String::from(format!("{} Fighters", name))
    }

    // $ cargo test tracing_async_test
    #[tokio::test]
    async fn tracing_async_test() {
        let _subscriber = tracing_subscriber::fmt()
            .compact()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_max_level(Level::TRACE)
            .try_init();

        let mut my_struct = MyStruct::default();
        my_struct.b.extend_from_slice(&[1, 2, 3, 4]);

        info_span!("external crate").in_scope(|| suggest_band("Foo".to_string()));

        let _enter = span!(Level::INFO, "get_band_rec", ?my_struct).entered();

        let band = suggest_band_async("Foo".to_string())
            .instrument(debug_span!("my_future"))
            .await;
        info!(message = "Got a recommendation!", %band);

        let future = async {
            tracing::debug!("this event will occur inside `my_span`");
        };
        tokio::spawn(future.in_current_span()).await.unwrap();

        for i in 0..10 {
            let span = span!(Level::TRACE, "my_loop", iteration = i);
            let _enter = span.enter();
            trace!("Go sports!");
        }
    }

    // $ cargo test tracing_log_test
    #[test]
    fn tracing_log_test() {
        let _subscriber = tracing_subscriber::fmt()
            .compact()
            .with_max_level(Level::TRACE)
            .init();

        log::info!("Got a recommendation!");
    }

    // $ RUST_LOG=trace cargo test tracing_log2_test
    #[test]
    fn tracing_log2_test() {
        // requires tracing::log feature.
        env_logger::init();

        log::info!("Got a recommendation!");
        let band = "Artic Monkeys";
        tracing::info!(band, "Got a recommendation!");
    }

    // $ RUST_LOG=trace cargo test tracing_test_log_test -- --nocapture
    #[test_log::test]
    fn tracing_test_log_test() {
        let band = "Artic Monkeys";
        tracing::info!(band, "Got a recommendation!");
    }

    use opentelemetry::global;
    use std::{error::Error, thread, time::Duration};
    use tracing_subscriber::layer::SubscriberExt;

    #[instrument]
    #[inline]
    fn expensive_work() -> &'static str {
        span!(tracing::Level::INFO, "expensive_step_1", status = "200")
            .in_scope(|| thread::sleep(Duration::from_millis(25)));
        span!(tracing::Level::INFO, "expensive_step_2")
            .in_scope(|| thread::sleep(Duration::from_millis(25)));

        "success"
    }

    // $ docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
    // $ RUST_LOG=trace cargo test tracing_opentelemetry
    // $ firefox http://localhost:16686
    #[test]
    fn tracing_opentelemetry() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name("report_example")
            .install_simple()?;

        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(opentelemetry)
            .with(tracing_subscriber::fmt::layer())
            .try_init()?;

        {
            let root = span!(tracing::Level::INFO, "app_start", work_units = 2);
            let _enter = root.enter();

            let work_result = expensive_work();

            span!(tracing::Level::INFO, "faster_work")
                .in_scope(|| thread::sleep(Duration::from_millis(10)));

            warn!("About to exit!");
            trace!("status: {}", work_result);
        } // Once this scope is closed, all spans inside are closed as well

        global::shutdown_tracer_provider();

        Ok(())
    }
}
