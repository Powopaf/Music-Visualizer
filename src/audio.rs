use libpulse_binding::stream::Direction;
use libpulse_binding::sample::{Format, Spec};
use libpulse_simple_binding::Simple;

pub fn get_audio()-> Simple {
    let spec = Spec {
        format: Format::S16NE,
        rate: 44100,
        channels: 2,
    };

    assert!(spec.is_valid());

    Simple::new(
        None,
        "Music-Visualizer",
        Direction::Record,
        Some("@DEFAULT_SINK@.monitor"),
    "capture",
    &spec,
    None,
    None,)
        .expect("Failed to open audio stream")
}