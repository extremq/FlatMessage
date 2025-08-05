use std::mem::MaybeUninit;

use super::SerDe;
use crate::size;
use common::data_format::DataFormat;

unsafe impl<'a, const N: usize> SerDe<'a> for [u8; N] {
    const DATA_FORMAT: DataFormat = DataFormat::FixArray;

    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self
    where
        Self: Sized,
    {
        let p = buf.as_ptr();
        let (count, slen) = size::read_unchecked(p, pos, size::Format::U8withExtension);
        if count != N {
            [0; N]
        } else {
            let mut result: MaybeUninit<[u8; N]> = MaybeUninit::uninit();
            unsafe {
                std::ptr::copy_nonoverlapping(
                    buf.as_ptr().add(pos + slen),
                    result.as_mut_ptr() as *mut u8,
                    N,
                );
            }
            unsafe { result.assume_init() }
        }
    }

    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self>
    where
        Self: Sized,
    {
        let (count, slen) =
            size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)?;
        if count != N {
            None
        } else {
            let mut result: MaybeUninit<[u8; N]> = MaybeUninit::uninit();
            unsafe {
                std::ptr::copy_nonoverlapping(
                    buf.as_ptr().add(pos + slen),
                    result.as_mut_ptr() as *mut u8,
                    N,
                );
            }
            let result = unsafe { result.assume_init() };
            Some(result)
        }
    }

    unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
        unsafe {
            let slen = size::write(p, pos, N as u32, size::Format::U8withExtension);
            std::ptr::copy_nonoverlapping(obj.as_ptr(), p.add(pos + slen), obj.len());
            pos + slen + N as usize
        }
    }

    fn size(_: &Self) -> usize {
        crate::size::len(N as u32, crate::size::Format::U8withExtension) + N
    }
}
