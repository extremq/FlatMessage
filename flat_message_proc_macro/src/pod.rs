use super::mem_alignament::MemAlignment;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::DeriveInput;

pub struct POD {
    name: syn::Ident,
    mem_alignment: MemAlignment,
}
impl POD {
    fn compute_hash(&self) -> u32 {
        common::hashes::crc32(self.name.to_string().as_bytes())
    }
    fn generate_const_assertion_functions(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let align_size = self.mem_alignment.align_size();
        let const_ident = format_ident!("_CONST_ASSERT_ALIGN_{}", name.to_string().to_uppercase());
        quote! {
            const #const_ident: () = {
                if std::mem::align_of::<#name>() != #align_size {
                    panic!(concat!(
                        "Incorrect representation for struct `",
                        stringify!(#name),
                        "`! Please check the #[repr(C, align(...))] attribute and make sure it matches std::mem::align_of::<",
                        stringify!(#name),
                        ">()"
                    ));
                }
            };
        }
    }

    fn generate_serde_implementation(&self) -> TokenStream {
        let name = &self.name;
        let name_hash = self.compute_hash();
        let align_mask = self.mem_alignment.align_size() - 1;
        let data_format = self.mem_alignment.data_format();

        quote! {
            unsafe impl<'a> SerDe<'a> for #name {
                const DATA_FORMAT: flat_message::DataFormat = #data_format;
                #[inline(always)]
                unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                    unsafe {
                        let (_, slen) = flat_message::size::read_unchecked(buf.as_ptr(), pos+4, flat_message::size::Format::U8withExtension);
                        let new_pos = ((pos + 4 + slen) + #align_mask) & !#align_mask;
                        let ptr = buf.as_ptr().add(new_pos) as *const Self;
                        std::ptr::read_unaligned(ptr)
                    }
                }
                #[inline(always)]
                fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
                    // read hash
                    if pos  + 4 > buf.len() {
                        None
                    } else {
                        unsafe {
                            let hash = (buf.as_ptr().add(pos) as *const u32).read_unaligned();
                            // check hash
                            if hash != #name_hash {
                                return None;
                            }
                            // read size
                            let (struct_size, slen) = flat_message::size::read(buf.as_ptr(), pos+4, buf.len(), flat_message::size::Format::U8withExtension)?;
                            if struct_size != std::mem::size_of::<Self>() {
                                None
                            } else {
                                let new_pos = ((pos + 4 + slen) + #align_mask) & !#align_mask;
                                if new_pos + struct_size > buf.len() {
                                    None
                                } else {
                                    unsafe {
                                        let ptr = buf.as_ptr().add(new_pos) as *const Self;
                                        Some(*ptr)
                                    }
                                }
                            }
                        }
                    }
                }
                #[inline(always)]
                unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                    unsafe {
                        std::ptr::write_unaligned(p.add(pos) as *mut u32, #name_hash);
                        let slen = flat_message::size::write(p, pos+4, std::mem::size_of::<Self>() as u32, flat_message::size::Format::U8withExtension);
                        let new_pos = ((pos + 4 + slen) + #align_mask) & !#align_mask;
                        std::ptr::write_unaligned(p.add(new_pos) as *mut Self, *obj as Self);
                        new_pos + std::mem::size_of::<Self>()
                    }
                }
                #[inline(always)]
                fn size(obj: &Self) -> usize {
                    let size = std::mem::size_of::<Self>();
                    let slen = flat_message::size::len(size as u32, flat_message::size::Format::U8withExtension);
                    let new_pos = ((4 + slen) + #align_mask) & !#align_mask;
                    new_pos + size
                }
            }
        }
    }
    pub fn generate_code(&self) -> TokenStream {
        let serde_code = self.generate_serde_implementation();
        let const_assertion_code = self.generate_const_assertion_functions();
        let name = &self.name;
        // let slice_code = self.generate_slice_serde_implementation();
        // let vec_code = self.generate_vector_serde_implementation();
        quote! {
            impl flat_message::FlatMessageCopy for #name {}
            #const_assertion_code
            #serde_code
            // for slices
            // #slice_code
            // for vectors
            // #vec_code
        }
    }
}

impl TryFrom<syn::DeriveInput> for POD {
    type Error = String;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {
        let alignment = 'main_loop: loop {
            for attr in input.attrs.iter() {
                if attr.path().is_ident("repr") {
                    let s = attr.to_token_stream().to_string().replace(" ", "");
                    if s.starts_with("#[repr(C,align(") && s.ends_with("))]") {
                        let aligament = s.replace("#[repr(C,align(", "").replace("))]", "");
                        break 'main_loop MemAlignment::try_from(aligament.as_str());
                    } else {
                        break 'main_loop Err("You can only use the `repr` attribute with FlatMessage if the `repr` attribute provides enforces `C` layout and specifies the alignment.  You can use one of the following: #[repr(C, align(1))], #[repr(C, align(2))], #[repr(C, align(4))], #[repr(C, align(8))] or #[repr(C, align(16))], ".to_string());                        
                    }
                }
            }
            break Err("You need to provide a repr attribute for the struct to be serializable/deserializable with FlatMessage. You can use one of the following: #[repr(C, align(1))], #[repr(C, align(2))], #[repr(C, align(4))], #[repr(C, align(8))] or #[repr(C, align(16))], ".to_string());
        }?;
        Ok(Self {
            name: input.ident,
            mem_alignment: alignment,
        })
    }
}
