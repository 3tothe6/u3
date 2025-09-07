use clap::{Parser, ValueEnum};
use tracing_subscriber::fmt::time::{ChronoLocal, OffsetTime};
use u3::tracing::fmt_default::WithU3Default;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    baseline: Option<Baseline>,
}

#[derive(Clone, Copy, ValueEnum)]
enum Baseline {
    A,
    B,
    C,
}

fn main() {
    match Args::parse().baseline {
        None => tracing_subscriber::fmt().with_u3_default().init(),
        Some(Baseline::A) => tracing_subscriber::fmt().init(),
        Some(Baseline::B) => tracing_subscriber::fmt().with_timer(ChronoLocal::rfc_3339()).init(),
        Some(Baseline::C) => {
            tracing_subscriber::fmt().with_timer(OffsetTime::local_rfc_3339().unwrap()).init()
        }
    }
    (0..100).for_each(|_| tracing::info!("info"));
}
