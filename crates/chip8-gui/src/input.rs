use std::{sync::RwLock, time::Instant};

use macroquad::prelude::{is_key_down, is_key_pressed, KeyCode};
use once_cell::sync::Lazy;

#[derive(Debug)]
struct AutoKeyContext {
    last_before_time: Instant,
    last_during_time: Instant,
    last_key: Option<KeyCode>,
    before_ms: usize,
    during_ms: usize,
}

impl AutoKeyContext {
    pub fn new() -> Self {
        Self {
            last_key: None,
            last_before_time: Instant::now(),
            last_during_time: Instant::now(),
            before_ms: 300,
            during_ms: 40,
        }
    }

    pub fn reset(&mut self, key: KeyCode, instant: Instant) {
        self.last_key = Some(key);
        self.last_before_time = instant;
        self.last_during_time = instant;
    }
}

static AUTO_KEY_CONTEXT: Lazy<RwLock<AutoKeyContext>> =
    Lazy::new(|| RwLock::new(AutoKeyContext::new()));

pub fn ui_is_key_auto_pressed(code: KeyCode) -> bool {
    if is_key_pressed(code) {
        let mut ctx = AUTO_KEY_CONTEXT.write().unwrap();
        let now = Instant::now();

        ctx.reset(code, now);
        return true;
    }

    if is_key_down(code) {
        let mut ctx = AUTO_KEY_CONTEXT.write().unwrap();
        let now = Instant::now();

        if let Some(last_key) = ctx.last_key {
            if code == last_key {
                if (now - ctx.last_before_time).as_millis() > ctx.before_ms as u128
                    && (now - ctx.last_during_time).as_millis() > ctx.during_ms as u128
                {
                    ctx.last_during_time = now;
                    return true;
                }
            } else {
                ctx.reset(code, now);
            }
        } else {
            ctx.reset(code, now);
        }
    }

    false
}
