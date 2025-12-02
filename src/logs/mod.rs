use tracing_subscriber::{EnvFilter, fmt::layer, prelude::*, registry};

pub fn logger() {
    let filter = EnvFilter::new("info,tower_http=debug");
    registry().with(filter).with(layer()).init();
}
