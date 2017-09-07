extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod serializer;
mod profiler;
mod docker;
mod git;

fn main() {

    // Add cmd line argument parser
    
    // testing
    // docker::status();

    match profiler::get_stats() {
        Ok(profile) =>
            match serializer::write_json(profile) {
                Ok(json) => println!("{}", json),
                Err(e) => println!("{}", "{\"error\" : \"error\"}"),
            }
        Err(e) => println!("{}", "{\"error\" : \"error\"}"),
    }
}
