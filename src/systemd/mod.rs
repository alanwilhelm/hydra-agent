extern crate shiplift;
use std::process::Command;

//! systemd control module
pub fn nginx_restart() {
    println!("nginx_restart");
    nginx_stop();
    nginx_start();
}

fn nginx_start() {
    println!("nginx_start");
    let output = Command::new("sh")
                         .arg("-c")
                         .arg("sudo")
                         .arg("service")
                         .arg("nginx")
                         .arg("restart")
                         .output()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let result = output.stdout;

}

fn nginx_stop() {
    println!("nginx_stop");
    let output = Command::new("sudo").arg("service").arg("nginx").arg("stop").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
