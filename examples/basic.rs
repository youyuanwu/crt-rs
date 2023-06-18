use std::os::raw::c_void;

use crt_rs::{gen::*, _CRTDBG_FILE_STDERR};

fn main() {
    unsafe { _CrtSetDbgFlag(_CRTDBG_CHECK_CRT_DF as i32) };
    let file = unsafe { _CrtSetReportFile(_CRT_WARN as i32, _CRTDBG_FILE_STDERR as *mut c_void) };
    println!("file is {:?}", file);
    {
        let s = String::from("hello");
        println!("{}", s);
    }
    let _leak = unsafe { _CrtDumpMemoryLeaks() };
    assert_eq!(_leak, 0);
}
