extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod serializer;
mod profiler;
mod docker;

fn main() {

    // testing
    docker::status();

    match profiler::get_stats() {
        Ok(profile) =>
            match serializer::write_json(profile) {
                Ok(json) => println!("{}", json),
                Err(e) => println!("{}", "{\"error\" : \"error\"}"),
            }
        Err(e) => println!("{}", "{\"error\" : \"error\"}"),
    }
}
