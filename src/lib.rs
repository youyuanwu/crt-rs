pub mod gen;

use std::ffi::c_void;

// dynamic link
// #[cfg(not(target_feature = "crt-static"))]
// #[link(name = "libvcruntimed")]
// #[link(name = "ucrtd")]

pub type ReportFile = *const c_void;
pub const _CRTDBG_FILE_STDERR: ReportFile = -5_i64 as ReportFile;
pub const _CRTDBG_HFILE_ERROR: ReportFile = -2_i64 as ReportFile;

#[cfg(test)]
mod tests {
    use std::os::raw::c_void;

    use crate::{
        gen::{
            _CrtDumpMemoryLeaks, _CrtMemCheckpoint, _CrtMemDumpAllObjectsSince, _CrtMemState,
            _CrtSetDbgFlag, _CrtSetReportFile, _CRTDBG_CHECK_CRT_DF, _CRT_WARN,
        },
        _CRTDBG_FILE_STDERR,
    };

    #[test]
    fn basic_test() {
        unsafe { _CrtSetDbgFlag(_CRTDBG_CHECK_CRT_DF as i32) };
        let file =
            unsafe { _CrtSetReportFile(_CRT_WARN as i32, _CRTDBG_FILE_STDERR as *mut c_void) };
        println!("file is {:?}", file);

        let s = String::from("hello");
        println!("{}", s);

        let _leak = unsafe { _CrtDumpMemoryLeaks() };
        assert_eq!(_leak, 1); // we have mem leak from crt setups.
    }

    #[test]
    fn basic2_test() {
        let mut state = _CrtMemState {
            pBlockHeader: std::ptr::null_mut(),
            lCounts: Default::default(),
            lSizes: Default::default(),
            lHighWaterCount: Default::default(),
            lTotalCount: Default::default(),
        };

        unsafe { _CrtMemCheckpoint(std::ptr::addr_of_mut!(state)) };

        // rust uses system heap not libc malloc, so std lib object will not have crt footprint.
        let bad_vec = vec![0u8; 1024 * 1024];
        std::mem::forget(bad_vec);

        // c leaked mem will be detected.
        let _ptr = unsafe { libc::malloc(5) };

        unsafe { _CrtMemDumpAllObjectsSince(std::ptr::addr_of!(state)) };
    }
}
