extern crate v4l2loopback_lib;

fn main() {
	println!("Create device 102");
	let v4l2loopback = v4l2loopback_lib::V4L2Loopback::new();
	v4l2loopback.add(102);
	v4l2loopback.query(102);
}	
