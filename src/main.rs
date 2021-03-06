#[warn(unused_must_use)]
extern crate libc; 
extern crate jsonnet;
use libc::c_char;
use std::str;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::CStr;
use std::ffi::CString;
use jsonnet::ffi::command::{ Jsonnet };
pub type JsonnetResult = Result<String, String>;
pub fn ctos(msg_buf : *const c_char)-> String{
	let msg_str: &CStr = unsafe { CStr::from_ptr(msg_buf) };
	let buf: &[u8] = msg_str.to_bytes();
	let str_buf: &str = str::from_utf8(buf).unwrap();
	let msg_data: String = str_buf.to_owned();
	return msg_data;
}
pub fn version(){
	let msg_buf: *const c_char = Jsonnet::version();
	let msg_data: String = ctos(msg_buf);
	println!("{:?}", msg_data); 
}



pub fn evaluate_file(){  
	let filename : *const libc::c_char = CString::new("/data2/rust-php-extension/git-pro/rust-jsonnet/example/t.jsonnet") .unwrap().as_ptr(); 
	let json = match Jsonnet::evaluate_file(filename) {
		Ok(json) => json,
		Err(e) => panic!("{:?}", e)
	}; 
	println!("{:?}", json);
}


pub fn evaluate_snippet(){
	let path = Path::new("/data2/rust-php-extension/git-pro/rust-jsonnet/example/t.jsonnet");
	let display = path.display();
	let mut file = match File::open(&path) { 
		Err(why) => panic!("couldn't open {}: {}", display,
			Error::description(&why)),
		Ok(file) => file,
	};
	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();
	let json_tpl : *const c_char = s.as_ptr() as *const c_char;
	let json = match Jsonnet::evaluate_snippet(json_tpl) {
		Ok(json) => json,
		Err(e) => panic!("{:?}", e)
	}; 
	println!("{:?}", json);
}

fn main() { 
	version();
	evaluate_file(); 
	evaluate_file(); 
	evaluate_file(); 
	Jsonnet::destroy();
	evaluate_snippet();
}