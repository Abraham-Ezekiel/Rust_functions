
fn test(){
	println!("my second function above the main");
}
fn main(){
	test();
	println!("Hello, world!");
	add_numbers(20, 30);
	
	let number = {
		let x =3;
		x + 1
	};
	println!("{}", number);

	let result = add_numbers2(12, 8);
	println!("{}", result);

}

fn add_numbers(x: i32, y: i32){
	println!("The sum is: {}", x*y + 2*y);
}
fn add_numbers2(num1: i32, num2: i32) ->i32 {
	//return num1 + num2;

	let result = num1 + num2;
	if result > 10 {
		return result - 10;
	}
	result

}
