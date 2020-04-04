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
	/// Initializes possibly instances from config file
	fn new() {
	
	}
}


impl EmailMedia {
	/// Initializes possibly instances from config file
	fn new() {
		// TODO
	}

	/// Sends email using current information
	fn send(&self) {
		// TODO: how to do?
	}
}


/// Reads contents from config file and returns as JSON
fn read_config_file() {
	// TODO: read contents from config file as string
	// TODO: get JSON from contents
}


/// Writes message in stderr
/// Writes message in /var/log/file_watcher.log for persistence
fn write_error_log(message: &str) {
	// TODO
}

