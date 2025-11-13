use std::sync::OnceLock;

use chrono::prelude::*;

pub fn now_stable() -> &'static DateTime<Local> {
    NOW_STABLE.get().unwrap()
}

pub fn now_stable_init() {
    NOW_STABLE.set(Local::now()).unwrap()
}

static NOW_STABLE: OnceLock<DateTime<Local>> = OnceLock::new();
