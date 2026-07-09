#[cfg(unix)]
pub fn get_username() -> String {
    use libc::{getpwuid_r, getuid, passwd};
    use std::ffi::CStr;

    unsafe {
        let uid = getuid();
        let mut pwd: passwd = std::mem::zeroed();
        let mut buf = vec![0u8; 4096];
        let mut result: *mut passwd = std::ptr::null_mut();

        let ret = getpwuid_r(
            uid,
            &mut pwd,
            buf.as_mut_ptr() as *mut i8,
            buf.len(),
            &mut result,
        );

        if ret != 0 || result.is_null() {
            return "unknown".into();
        }

        CStr::from_ptr(pwd.pw_name)
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(windows)]
pub fn get_username() -> String {
    std::env::var("USERNAME").unwrap_or_else(|_| "unknown".into())
}
