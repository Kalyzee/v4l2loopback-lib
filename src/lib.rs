use core::ffi::c_int;
use core::ffi::c_char;
use core::ffi::c_uint;
use std::fs::{File};
use std::os::fd::AsRawFd;


use nix::{ioctl_read_bad, ioctl_write_int_bad, ioctl_write_ptr_bad};

const V4L2LOOPBACK_CTL_CTRL_DEVICE: &str = "/dev/v4l2loopback";
const V4L2LOOPBACK_CTL_ADD: u32 = 0x4C80;
const V4L2LOOPBACK_CTL_QUERY: u32 = 0x4C82;
const V4L2LOOPBACK_CTL_REMOVE: u32 = 0x4C81;

#[repr(C)]
#[derive(Debug)]
struct V4l2LoopbackCtl {
    pub output_nr: c_int,
    pub capture_nr: c_int,
	pub card_label: *mut c_char,
	pub min_width: c_uint,
	pub max_width : c_uint,
	pub min_height : c_uint,
	pub max_height : c_uint,
	pub max_buffers : c_int,
	pub max_openers : c_int,
	pub debug : c_int,
	pub announce_all_caps : c_int,
}



pub struct V4L2Loopback{
	file: File
}

impl V4L2Loopback{
	pub fn new() -> V4L2Loopback{
		V4L2Loopback { file: File::options()            
			.read(true)
			.write(true)
			.create(false)
			.open(V4L2LOOPBACK_CTL_CTRL_DEVICE).unwrap()
		}
	}
	
	pub fn add(&self, device_id: c_int){
		let _ = unsafe { v4l2loopback_add(self.file.as_raw_fd(), &mut V4l2LoopbackCtl{
				output_nr: device_id,
				capture_nr: 0,
				card_label: "v4l2loopback".as_ptr() as *mut c_char,
				min_width: 0,
				max_width: 0,
				min_height: 0,
				max_height: 0,
				max_buffers: 0,
				max_openers: 0,
				debug: 0,
				announce_all_caps: 0,
			} as *mut V4l2LoopbackCtl)};
	}	

	pub fn query(&self, device_id: c_int){

		let mut v4l2loopbackctl = V4l2LoopbackCtl{
				output_nr: device_id,
				capture_nr: 0,
				card_label: "v4l2loopback".as_ptr() as *mut c_char,
				min_width: 0,
				max_width: 0,
				min_height: 0,
				max_height: 0,
				max_buffers: 0,
				max_openers: 0,
				debug: 0,
				announce_all_caps: 0,
			};

		let _ = unsafe { v4l2loopback_query(self.file.as_raw_fd(), &mut v4l2loopbackctl as *mut V4l2LoopbackCtl)};
		println!("{:#?}", v4l2loopbackctl);
	}

	pub fn remove(&self, device_id: c_int){
		let _ = unsafe { v4l2loopback_remove(self.file.as_raw_fd(), device_id.try_into().unwrap())};
	}
}

ioctl_write_ptr_bad!(v4l2loopback_add, V4L2LOOPBACK_CTL_ADD, V4l2LoopbackCtl);
ioctl_read_bad!(v4l2loopback_query, V4L2LOOPBACK_CTL_QUERY, V4l2LoopbackCtl);
ioctl_write_int_bad!(v4l2loopback_remove, V4L2LOOPBACK_CTL_REMOVE);

