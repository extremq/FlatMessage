use super::mem_alignament::MemAlignment;
use common::data_format::DataFormat;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::DeriveInput;

pub struct Flags {
    name: syn::Ident,
    sealed: bool,
    flags: Vec<String>,
    repr_size: u8,
}
impl Flags {
    fn compute_hash(&self) -> u32 {
        if self.sealed {
            let mut name = self.name.to_string();
            for flag in self.flags.iter() {
                name.push_str(flag);
                name.push(',');
            }
            common::hashes::crc32(name.as_bytes())
        } else {
            common::hashes::crc32(self.name.to_string().as_bytes())
        }        
    }
    fn data_format(&self) -> proc_macro2::TokenStream {
        match self.repr_size {
            1 => quote! {DataFormat::U8},
            2 => quote! {DataFormat::U16},
            4 => quote! {DataFormat::U32},
            8 => quote! {DataFormat::U64},
            16 => quote! {DataFormat::U128},
            _ => quote! {},
        }
    }
    fn repr_type(&self) -> proc_macro2::TokenStream {
        match self.repr_size {
            1 => quote! {u8},
            2 => quote! {u16},
            4 => quote! {u32},
            8 => quote! {u64},
            16 => quote! {u128},
            _ => quote! {},
        }
    }
    fn generate_const_assertion_functions(&self) -> proc_macro2::TokenStream {
        // let name = &self.name;
        // //let align_size = self.mem_alignment.align_size();
        // let const_ident = format_ident!("_CONST_ASSERT_ALIGN_{}", name.to_string().to_uppercase());
        // quote! {
        //     const #const_ident: () = {
        //         if std::mem::align_of::<#name>() != #align_size {
        //             panic!(concat!(
        //                 "Incorrect representation for struct `",
        //                 stringify!(#name),
        //                 "`! Please check the #[repr(C, align(...))] attribute and make sure it matches std::mem::align_of::<",
        //                 stringify!(#name),
        //                 ">()"
        //             ));
        //         }
        //     };
        // }
        quote! {}
    }

    fn generate_serde_implementation(&self) -> TokenStream {
        let name = &self.name;
        let name_hash = self.compute_hash();
        let data_format = self.data_format();
        let repr_type = self.repr_type();

        quote! {
            unsafe impl<'a> SerDe<'a> for #name {
                const DATA_FORMAT: flat_message::DataFormat = #data_format;
                #[inline(always)]
                unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                    unsafe {
                        let ptr = buf.as_ptr().add(pos+4) as *const Self;
                        std::ptr::read_unaligned(ptr)
                    }
                }
                #[inline(always)]
                fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
                    if pos + std::mem::size_of::<#repr_type>() + 4 > buf.len() {
                        None
                    } else {
                        unsafe {
                            let hash = (buf.as_ptr().add(pos) as *const u32).read_unaligned();
                            if hash != #name_hash {
                                return None;
                            }
                            let value = ((buf.as_ptr().add(pos+4) as *const #repr_type)).read_unaligned();
                            Self::from_value(value)
                        }
                    }
                }
                #[inline(always)]
                unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                    unsafe {
                        std::ptr::write_unaligned(p.add(pos) as *mut u32, #name_hash);
                        std::ptr::write_unaligned(p.add(pos+4) as *mut #repr_type, *obj as #repr_type);
                        pos + std::mem::size_of::<#repr_type>()+4
                    }
                }
                #[inline(always)]
                fn size(_: &Self) -> usize {
                    std::mem::size_of::<#repr_type>()+4 /* name hash + value */
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

impl TryFrom<syn::DeriveInput> for Flags {
    type Error = String;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {
        let mut repr = false;
        let mut sealed = false;
        let mut flags = Vec::<String>::new();
        for attr in input.attrs.iter() {
            if attr.path().is_ident("repr") {
                let s = attr.to_token_stream().to_string().replace(" ", "");
                if s != "#[repr(transparent)]" {
                    return Err("You can only use the `repr(transparent)` attribute for the struct to be serializable/deserializable as a flags object. ".to_string());
                }
                repr = true;
            }
            if attr.path().is_ident("sealed") {
                sealed = true;
            }
            if attr.path().is_ident("flags") {
                let s = attr.to_token_stream().to_string().replace(" ", "");
                if s.starts_with("#[flags(") && s.ends_with(")]") {
                    let flags_str = s.replace("#[flags(", "").replace(")]", "");
                    flags = flags_str.split(",").map(|f| f.trim().to_string()).collect();
                }
            }
        }
        // Extract the inner type from the struct's generic parameter
        let type_name = if let syn::Data::Struct(data_struct) = input.data {
            if let syn::Fields::Unnamed(fields) = data_struct.fields {
                if fields.unnamed.len() == 1 {
                    if let syn::Type::Path(type_path) = &fields.unnamed[0].ty {
                        if let Some(segment) = type_path.path.segments.last() {
                            segment.ident.to_string()
                        } else {
                            return Err("Invalid type parameter".to_string());
                        }
                    } else {
                        return Err("Invalid type parameter".to_string());
                    }
                } else {
                    return Err("Struct must have exactly one unnamed field".to_string());
                }
            } else {
                return Err("Struct must have unnamed fields".to_string());
            }
        } else {
            return Err("Only structs are supported".to_string());
        };
        let repr_size = match type_name.as_str() {
            "u8" => 1,
            "u16" => 2,
            "u32" => 4,
            "u64" => 8,
            "u128" => 16,
            _ => return Err("You need to add a type parameter to the struct to be serializable/deserializable as a flags object. ".to_string()),
        };
        if !repr {
            return Err("You need to add #[repr(transparent)] attribute to the struct to be serializable/deserializable as a flags object. ".to_string());
        }
        if flags.is_empty() {
            return Err("You need to add at least one flag in the #[flags(...)] attribute to the struct to be serializable/deserializable as a flags object. ".to_string());
        }
        flags.sort();
        Ok(Self {
            name: input.ident,
            sealed,
            flags,
            repr_size
        })
    }
}
