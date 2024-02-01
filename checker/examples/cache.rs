//! This generates a cache from a definition file

use std::{
	env,
	fs::{read_to_string, write},
};

use ezno_checker::{generate_cache, synthesis::EznoParser};

fn main() {
	let input = env::args().nth(1).expect("expected path to definition file");
	let output = env::args().nth(2).expect("expected output path");

	let reader = |path: &std::path::Path| read_to_string(path).ok();
	let cache = generate_cache::<_, EznoParser>(input.into(), reader, ());
	write(output, cache).unwrap();
	eprintln!("Cache generated 🏧💵✅")
}
