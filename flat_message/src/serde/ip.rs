use super::SerDe;
use common::data_format::DataFormat;
use std::ptr;
use std::net::{Ipv4Addr, Ipv6Addr};

macro_rules! IMPLEMENT_SERDE_FOR_IP_TYPE {
    ($t:ty, $data_format:ident) => {
        unsafe impl<'a> SerDe<'a> for $t {
            const DATA_FORMAT: DataFormat = DataFormat::$data_format;
            #[inline(always)]
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                unsafe {
                    let ptr = buf.as_ptr().add(pos) as *const $t;
                    std::ptr::read_unaligned(ptr)
                }
            }
            #[inline(always)]
            fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
                if pos + std::mem::size_of::<$t>() > buf.len() {
                    None
                } else {
                    unsafe {
                        let ptr = buf.as_ptr().add(pos) as *const $t;
                        Some(std::ptr::read_unaligned(ptr))
                    }
                }
            }
            #[inline(always)]
            unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                unsafe {
                    ptr::write_unaligned(p.add(pos) as *mut $t, *obj);
                    pos + std::mem::size_of::<$t>()
                }
            }
            #[inline(always)]
            fn size(_: &Self) -> usize {
                std::mem::size_of::<$t>()
            }
        }
    };
}

IMPLEMENT_SERDE_FOR_IP_TYPE!(Ipv4Addr, IPv4);
IMPLEMENT_SERDE_FOR_IP_TYPE!(Ipv6Addr, IPv6);