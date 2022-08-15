fn main() {
	let fizz: &str = "Fizz";
	let buzz: &str = "Buzz";

	for i in 1..100 {
		let mut result: String = "".to_owned();
		if i % 3 == 0 { result.push_str(fizz); }
		if i % 5 == 0 { result.push_str(buzz); }
		if result.is_empty() { result.push_str(&i.to_string()); }
		print!("{}\n",result);
	}
		}
