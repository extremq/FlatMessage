use super::SerDe;
use super::SerDeSlice;
use crate::size;
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

unsafe impl<'a> SerDeSlice<'a> for Ipv4Addr {
    const DATA_FORMAT: DataFormat = DataFormat::IPv4;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> &'a [Self] {
        let p = buf.as_ptr();
        let (len, buf_len) = size::read_unchecked(p, pos, size::Format::U8withExtension);
        std::slice::from_raw_parts(p.add(pos + buf_len) as *const Ipv4Addr, len)
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<&'a [Self]> {
        let (len, buf_len) =
            size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)?;
        let end = pos + buf_len + len;
        if end > buf.len() {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(
                    buf.as_ptr().add(pos + buf_len) as *const Ipv4Addr,
                    len,
                )
            })
        }
    }
    #[inline(always)]
    unsafe fn write(obj: &[Self], p: *mut u8, pos: usize) -> usize {
        let len = obj.len() as u32;
        unsafe {
            let buf_len = size::write(p, pos, len, size::Format::U8withExtension);
            std::ptr::copy_nonoverlapping(
                obj.as_ptr() as *mut u8,
                p.add(pos + buf_len),
                obj.len(),
            );
            pos + buf_len + len as usize
        }
    }
    #[inline(always)]
    fn size(obj: &[Self]) -> usize {
        size::len(obj.len() as u32, size::Format::U8withExtension) + obj.len()
    }
}
    


IMPLEMENT_SERDE_FOR_IP_TYPE!(Ipv4Addr, IPv4);
IMPLEMENT_SERDE_FOR_IP_TYPE!(Ipv6Addr, IPv6);