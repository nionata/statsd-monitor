//! Simple statsd client example binary. A single gauge, [`MAIN_LOOP_TICKS_METRIC_NAME`], is published out at [`PUBLISH_FREQUENCY`].
//!
//! ```bash
//! $ nc -ul 8125
//! example-client.main.loop.ticks:1|g
//! example-client.main.loop.ticks:2|g
//! example-client.main.loop.ticks:3|g
//! example-client.main.loop.ticks:4|g
//! example-client.main.loop.ticks:5|g
//! ```

use metrics_exporter_statsd::StatsdBuilder;
use std::thread;
use std::time::Duration;

/// The metric name for the number of main loop ticks.
pub const MAIN_LOOP_TICKS_METRIC_NAME: &str = "main.loop.ticks";
/// The frequency at which the main loop will tick.
pub const PUBLISH_FREQUENCY: f64 = 10.0;
const TICK_DURATION: Duration = Duration::from_millis(((1.0 / PUBLISH_FREQUENCY) * 1000.0) as u64);

fn main() {
    let recorder = StatsdBuilder::from("0.0.0.0", 8125)
        .build(Some("example-client"))
        .expect("Failed to init statsd");

    metrics::set_global_recorder(recorder).expect("Failed to init recorder");

    println!(
        "Running main loop at {}Hz or every {:?}",
        PUBLISH_FREQUENCY, TICK_DURATION
    );

    let mut i = 0;
    let tick_gauage = metrics::gauge!(MAIN_LOOP_TICKS_METRIC_NAME);

    loop {
        thread::sleep(TICK_DURATION);

        i += 1;

        tick_gauage.set(i);
    }
}
