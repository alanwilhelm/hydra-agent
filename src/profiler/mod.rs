extern crate sys_info;

use std::io::{Error, ErrorKind};
use std::u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemProfile {
    cpu_num: String,
    cpu_speed: String,
    hostname: String,
    os_type: String,
    os_release: String,
}

pub fn get_stats() -> Result<(SystemProfile), Error> {
    let z: u32 = u32::min_value();
    let json_z: String = 0.to_string();
    let undef: String = "undefined".to_string();

    let cpu_count: String = match sys_info::cpu_num() {
        Ok(cnt) => cnt.to_string(),
        Err(e) => undef.clone(),
    };

    let cpu_speed: String = match sys_info::cpu_speed() {
        Ok(speed) => speed.to_string(),
        Err(e) => json_z.clone(),
    };

    let hostname: String = match sys_info::hostname() {
        Ok(name) => name,
        Err(e) => undef.clone(),
    };

    let os_type: String = match sys_info::os_type() {
        Ok(t) => t,
        Err(e) => undef.clone(),
    };

    let os_release: String = match sys_info::os_release() {
        Ok(rel) => rel,
        Err(e) => undef.clone(),
    };

    // match sys_info::mem_info() {
    //     Ok(cnt) => println!("got me some mem_info stats: {:?}", cnt),
    //     Err(e) => println!("operational problem encountered: {}", e),
    // }

    let stats: SystemProfile = SystemProfile {
        cpu_num: cpu_count,
        cpu_speed: cpu_speed,
        hostname: hostname,
        os_type: os_type,
        os_release: os_release,
    };

    Ok(stats)
}
