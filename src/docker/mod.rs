extern crate shiplift;


/// Docker control module
pub fn status() {
    println!("Listing docker images");
    let docker = shiplift::Docker::new();
    let images = docker.images();
    for i in images.list(&Default::default()).unwrap() {
        println!("-> {:?}", i);
    }
}
