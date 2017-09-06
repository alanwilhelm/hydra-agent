extern crate sys_info;

fn main() {

    match sys_info::cpu_num() {
        Ok(cnt) => println!("got me a cpu count: {}", cnt.to_string()),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    match sys_info::cpu_speed() {
        Ok(cnt) => println!("got me a cpu speed: {}", cnt.to_string()),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    match sys_info::hostname() {
        Ok(cnt) => println!("got me a hostname: {}", cnt),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    match sys_info::os_type() {
        Ok(cnt) => println!("got me an OS Type: {}", cnt),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    match sys_info::os_release() {
        Ok(cnt) => println!("got me an OS Release: {}", cnt),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    match sys_info::mem_info() {
        Ok(cnt) => println!("got me some mem_info stats: {:?}", cnt),
        Err(e) => println!("operational problem encountered: {}", e),
    }

}
