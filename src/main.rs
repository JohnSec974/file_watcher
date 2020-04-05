use std::process;

mod workers;


fn main() {
    let v = workers::helper::read_config_file();

    println!("Hello, world!");
}
