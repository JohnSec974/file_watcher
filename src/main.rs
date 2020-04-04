fn main() {
    println!("Hello, world!");
}


struct Config {
	config_name: String,
	folder_path: String,
	file_name: String,
	last_line: u32
}


struct EmailMedia {
	address: String,
	password: String,
	host: String,
	port: u32,
	use_tls: bool,
	subject_prefix: String
}

impl Config {
	fn new() {
	
	}
}

impl EmailMedia {
	fn send() {
	
	}
}

