use libc::{
    freeifaddrs, getifaddrs, ifaddrs, sockaddr_in, AF_INET, IFF_LOOPBACK, IFF_UP,
};
use std::ffi::CStr;
use std::net::Ipv4Addr;

pub fn get_ips() -> String {
    unsafe {
        let mut ifap: *mut ifaddrs = std::ptr::null_mut();
        let ret = getifaddrs(&mut ifap);
        if ret != 0 || ifap.is_null() {
            return String::new();
        }

        let mut ips = Vec::new();
        let mut curr = ifap;

        while !curr.is_null() {
            let ifa = &*curr;

            if ifa.ifa_flags as u32 & IFF_UP as u32 != 0
                && ifa.ifa_flags as u32 & IFF_LOOPBACK as u32 == 0
                && !ifa.ifa_addr.is_null()
                && (*ifa.ifa_addr).sa_family as i32 == AF_INET
            {
                let name = CStr::from_ptr(ifa.ifa_name).to_string_lossy();
                let name_str = name.as_ref();

                if !name_str.starts_with("docker")
                    && !name_str.starts_with("virbr")
                    && !name_str.starts_with("br-")
                {
                    let sin = &*(ifa.ifa_addr as *const sockaddr_in);
                    let ip = Ipv4Addr::from(sin.sin_addr.s_addr.to_ne_bytes());
                    ips.push(ip.to_string());
                }
            }

            curr = ifa.ifa_next;
        }

        freeifaddrs(ifap);
        ips.join(" ")
    }
}
