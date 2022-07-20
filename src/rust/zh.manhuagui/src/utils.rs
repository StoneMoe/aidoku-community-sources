use aidoku::std::net::{HttpMethod, Request};
use alloc::string::String;
use alloc::vec::Vec;

pub fn encode_uri(string: &String) -> String {
	let mut result: Vec<u8> = Vec::with_capacity(string.len() * 3);
	let hex = "0123456789abcdef".as_bytes();
	let bytes = string.as_bytes();

	for byte in bytes {
		let curr = *byte;
		if (b'a'..=b'z').contains(&curr)
			|| (b'A'..=b'Z').contains(&curr)
			|| (b'0'..=b'9').contains(&curr)
			|| (curr == b';'
				|| curr == b',' || curr == b'/'
				|| curr == b'?' || curr == b':'
				|| curr == b'@' || curr == b'&'
				|| curr == b'=' || curr == b'+'
				|| curr == b'$')
			|| (curr == b'-'
				|| curr == b'_' || curr == b'.'
				|| curr == b'!' || curr == b'~'
				|| curr == b'*' || curr == b'\''
				|| curr == b'(' || curr == b')')
			|| (curr == b'#')
		{
			result.push(curr);
		} else {
			result.push(b'%');
			result.push(hex[curr as usize >> 4]);
			result.push(hex[curr as usize & 15]);
		}
	}

	String::from_utf8(result).unwrap_or_default()
}

fn i32_to_string(mut integer: i32) -> String {
	if integer == 0 {
		return String::from("0");
	}
	let mut string = String::with_capacity(11);
	let pos = if integer < 0 {
		string.insert(0, '-');
		1
	} else {
		0
	};
	while integer != 0 {
		let mut digit = integer % 10;
		if pos == 1 {
			digit *= -1;
		}
		string.insert(pos, char::from_u32((digit as u32) + ('0' as u32)).unwrap());
		integer /= 10;
	}
	string
}

pub fn get(url: &str) -> Request {
	Request::new(url, HttpMethod::Get)
        .header("Referer", "https://www.manhuagui.com/")
        .header("User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.0.0 Safari/537.36 Aidoku/1.0")
        .header("Cookie", "country=HK; isAdult=1")
}
