#[cfg(test)]
mod tests {

    use std::{thread, time::Duration};

    use opentelemetry::{global, sdk::export::trace::stdout, trace::Tracer};

    // $ docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
    // $ cargo test opentelemetry_jaeger_test
    // $ firefox http://localhost:16686
    #[test]
    fn opentelemetry_jaeger_test() {
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name("example")
            .install_simple()
            .expect("opentelemetry_jaeger init failed");

        tracer.in_span("main", |ctx| {
            thread::sleep(Duration::from_millis(25));

            tracer.in_span("small load", |ctx| {
                thread::sleep(Duration::from_millis(20));
            });

            tracer.in_span("big load", |ctx| {
                thread::sleep(Duration::from_millis(100));
            });
        });

        // Shutdown trace pipeline
        global::shutdown_tracer_provider();
    }
}
