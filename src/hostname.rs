#[cfg(unix)]
pub fn get_hostname() -> String {
    use libc::gethostname;
    use std::ffi::CStr;

    let mut buf = vec![0u8; 256];
    let ret = unsafe { gethostname(buf.as_mut_ptr() as *mut i8, buf.len()) };

    if ret != 0 {
        return "unknown".into();
    }

    let hostname = unsafe {
        CStr::from_ptr(buf.as_ptr() as *const i8)
            .to_string_lossy()
            .into_owned()
    };

    match hostname.split_once('.') {
        Some((short, _)) => short.to_string(),
        None => hostname,
    }
}

#[cfg(windows)]
pub fn get_hostname() -> String {
    std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".into())
}
