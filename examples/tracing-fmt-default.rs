use tracing::Subscriber;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::{ChronoLocal, OffsetTime};
use u3::tracing::fmt_default::WithU3Default;

fn main() {
    println!("============================== fmt() ==============================");
    show(fmt().finish());
    println!("============ fmt().with_timer(ChronoLocal::rfc_3339()) ============");
    show(fmt().with_timer(ChronoLocal::rfc_3339()).finish());
    println!("===== fmt().with_timer(OffsetTime::local_rfc_3339().unwrap()) =====");
    show(fmt().with_timer(OffsetTime::local_rfc_3339().unwrap()).finish());
    println!("===================== fmt().with_u3_default() =====================");
    show(fmt().with_u3_default().finish());
}

fn show(subscriber: impl Subscriber + Send + Sync + 'static) {
    tracing::subscriber::with_default(subscriber, || (0..100).for_each(|_| tracing::info!("info")));
}
