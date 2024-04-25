#[derive(Debug, Eq, PartialEq, Clone)]

pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
		let lumiere = Light {
		alias : alias.to_string(),
		brightness : 0,
	};

		lumiere
	}
}

// pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8) {
// }