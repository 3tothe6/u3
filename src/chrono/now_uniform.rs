use std::sync::OnceLock;

use chrono::prelude::*;

pub fn now_uniform() -> &'static DateTime<Local> {
    NOW_UNIFORM.get().unwrap()
}

pub fn now_uniform_init() {
    NOW_UNIFORM.set(Local::now()).unwrap()
}

static NOW_UNIFORM: OnceLock<DateTime<Local>> = OnceLock::new();
