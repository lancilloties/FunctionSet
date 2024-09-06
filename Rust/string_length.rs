// rustc -O -C prefer-dynamic

fn main(){
	let text = "A secret phrase!"; // 16 characters
	
	println!("The text is: {}", text);
	println!("Byte length: {}", text.len());
	println!("Character length: {}", text.chars().count());
}
