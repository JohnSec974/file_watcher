struct EmailMedia {
	address: String,
	password: String,
	host: String,
	port: u32,
	use_tls: bool,
	subject_prefix: String
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
