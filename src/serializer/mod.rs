extern crate serde;
extern crate serde_json;

use std::io::{Error};
use profiler;

pub fn write_json(profile: profiler::SystemProfile) -> Result<(String), Error> {
    let mut s: String = String::new();
    let s = serde_json::to_string(&profile)?;
    Ok(s)
}
