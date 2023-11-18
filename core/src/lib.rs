use std::mem::MaybeUninit;

use arcdps_imgui::{Context, Ui};
use raw_structs::AddonAPI;

pub mod raw_structs;

#[cfg(feature = "log")]
impl From<log::Level> for raw_structs::ELogLevel {
    fn from(level: log::Level) -> Self {
        use log::Level;
        match level {
            Level::Error => Self::CRITICAL,
            Level::Warn => Self::WARNING,
            Level::Info => Self::INFO,
            Level::Debug => Self::DEBUG,
            Level::Trace => Self::TRACE,
        }
    }
}

static mut ADDON_API: MaybeUninit<&'static AddonAPI> = MaybeUninit::uninit();
static mut CTX: MaybeUninit<Context> = MaybeUninit::uninit();
static mut UI: MaybeUninit<Ui> = MaybeUninit::uninit();

pub struct AddonApi;

impl AddonAPI {
    pub fn load(cb: fn()) {}
}
