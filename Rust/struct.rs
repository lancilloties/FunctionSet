// rustc -O -C prefer-dynamic
struct PeerStruct {
	active: bool,
	friends: u8,
	enemies: u8,
	secrets: u32,
	name: String
}

fn main(){
	let me = PeerStruct {
		active: true,
		friends: 1,
		enemies: 2,
		secrets: 2000,
		name: "Xilo".to_string()
	};
	
	println!("My name is {}", me.name);
	println!("I have {} friends.", me.friends);
	println!("I have {} enemies.", me.enemies);
	println!("My active state: {}", me.active);
	println!("I have {} secrets.", me.secrets);
}
