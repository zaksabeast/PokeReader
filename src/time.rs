use crate::ctr;
use chrono::{DateTime, NaiveDateTime};

const THIRTY_YEARS_MS: u64 = 946684800000;
/// Milliseconds between the NTP epoch (1st Jan 1900) and 1st Jan 2000, which is what the games use.
const NTP_TO_2000_MS: u64 = 3155673600000;

static mut GAME_START_MS: u64 = 0;

pub fn datetime_from_console_ms(console_ms: u64) -> NaiveDateTime {
    let standard_ms = console_ms.saturating_add(THIRTY_YEARS_MS);
    let ms: i64 = standard_ms.try_into().unwrap_or_default();
    DateTime::from_timestamp_millis(ms)
        .map(|datetime| datetime.naive_utc())
        .unwrap_or_default()
}

/// Records the current console time as the game's start time.
/// Should be called once when the plugin starts.
pub fn set_game_start_ms() {
    unsafe { GAME_START_MS = ctr::os_get_time().saturating_sub(NTP_TO_2000_MS) };
}

pub fn game_start_ms() -> u64 {
    unsafe { GAME_START_MS }
}

pub fn os_time() -> NaiveDateTime {
    datetime_from_console_ms(ctr::os_get_time().saturating_sub(NTP_TO_2000_MS))
}
