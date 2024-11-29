fn square_it(num: i32) -> i32{
	num.pow(2)
}

fn main() {
	let num:i32 = 5;
	let result:i32 = square_it(num);
	println!("{} squared is {}", num, result);
}
