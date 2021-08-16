pub fn decode_cipher(cipher: String) -> String {
	cipher.chars().map(|f| -> char {
		if f.is_ascii_alphabetic() {
			let t_f = if f.is_ascii_lowercase() { b'a' } else { b'A' };
			let first = t_f as i8;
			let mut sub = f as i8 - 5 - first;

			if sub < 0 {
				sub += 26;
			};
			((first + sub % 26) as u8) as char
		} else {
			f
		}
	}).collect::<String>()	
}


pub fn encode_cipher(cipher: String) -> String {
	cipher.chars().map(|f| -> char {
		if f.is_ascii_alphabetic() {
			let first = if f.is_ascii_lowercase() { b'a' } else { b'A' };
			(first + (f as u8 + 5 - first) % 26) as char
		} else {
			f
		}
	}).collect::<String>()	
}