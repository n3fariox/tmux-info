#[cfg(unix)]
mod inner {
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
}

#[cfg(windows)]
mod inner {
    use std::net::Ipv4Addr;
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GetAdaptersAddresses, GAA_FLAG_INCLUDE_PREFIX,
    };
    use windows_sys::Win32::Networking::WinSock::SOCKADDR_IN;

    unsafe fn ucstr_to_string(ptr: *const u16) -> String {
        if ptr.is_null() {
            return String::new();
        }
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len))
    }

    pub fn get_ips() -> String {
        unsafe {
            let mut buf_len: u32 = 0;

            // First call: get required buffer size
            let ret = GetAdaptersAddresses(
                2, // AF_INET
                GAA_FLAG_INCLUDE_PREFIX,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut buf_len,
            );

            // ERROR_BUFFER_OVERFLOW = 111
            if ret != 111 {
                return String::new();
            }

            let mut buf = vec![0u8; buf_len as usize];
            let adapters = buf.as_mut_ptr()
                as *mut windows_sys::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH;

            let ret = GetAdaptersAddresses(
                2, // AF_INET
                GAA_FLAG_INCLUDE_PREFIX,
                std::ptr::null_mut(),
                adapters,
                &mut buf_len,
            );

            // ERROR_SUCCESS = 0
            if ret != 0 {
                return String::new();
            }

            let mut ips = Vec::new();
            let mut curr = adapters;

            while !curr.is_null() {
                let adapter = &*curr;

                // IfOperStatusUp == 1
                if adapter.OperStatus == 1 {
                    let name = ucstr_to_string(adapter.FriendlyName as *const u16);

                    // 24 = IF_TYPE_SOFTWARE_LOOPBACK
                    if adapter.IfType != 24
                        && !name.starts_with("docker")
                        && !name.starts_with("virbr")
                        && !name.starts_with("br-")
                    {
                        let mut addr = adapter.FirstUnicastAddress;
                        while !addr.is_null() {
                            let unicast = &*addr;
                            let sockaddr = unicast.Address.lpSockaddr;
                            if !sockaddr.is_null()
                                && (*sockaddr).sa_family == 2 // AF_INET
                            {
                                let sin = &*(sockaddr as *const SOCKADDR_IN);
                                let ip = Ipv4Addr::from(
                                    sin.sin_addr.S_un.S_addr.to_ne_bytes(),
                                );
                                ips.push(ip.to_string());
                            }
                            addr = unicast.Next;
                        }
                    }
                }

                curr = adapter.Next;
            }

            ips.join(" ")
        }
    }
}

pub use inner::get_ips;
