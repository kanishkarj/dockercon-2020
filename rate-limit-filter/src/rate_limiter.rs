use proxy_wasm;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::{Deserialize, Serialize};

const LIMIT: u32 = 5;
const KEY: &str = "WASM_RATELIM";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimiter {
    RPM: Option<u32>,
    Min: i32,
    Count: u32,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            RPM: Some(LIMIT),
            Min: -1,
            Count: 0,
        }
    }
    pub fn get() -> Self {
        if let Ok(data) = proxy_wasm::hostcalls::get_shared_data(KEY) {
            if let Some(data) = data.0 {
                let data: Option<Self> = bincode::deserialize(&data).unwrap_or(None);
                if let Some(mut obj) = data {
                    return obj;
                }
            }
        }
        return Self::new();
    }
    pub fn set(&self) {
        let target: Option<Self> = Some(self.clone());
        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
        proxy_wasm::hostcalls::set_shared_data(KEY, Some(&encoded), None);
    }
    pub fn update(&mut self, time: i32) -> bool {
        if self.Min != time {
            self.Min = time;
            self.Count = 0;
        }
        self.Count += 1;
        proxy_wasm::hostcalls::log(
            LogLevel::Debug,
            format!("Obj {:?} {:?}", self.Count, self.RPM).as_str(),
        );
        if let Some(sm) = self.RPM {
            if self.Count > sm {
                return false;
            }
        }
        return true;
    }
}
