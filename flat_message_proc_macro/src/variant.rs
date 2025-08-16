use crate::data_type::DataType;
use super::utils;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields};

pub struct Variant {
    name: syn::Ident,
    variants: Vec<(String, Option<DataType>)>,
    sealed_enum: bool,
}

impl Variant {
    // fn compute_hash(&self) -> u32 {
    //     if self.sealed_enum {
    //         let mut name = self.name.to_string();
    //         let mut v = self.variants.clone();
    //         v.sort_by(|a, b| a.0.cmp(&b.0));
    //         for (variant_name, value) in v {
    //             name.push_str(variant_name.as_str());
    //             name.push_str(value.to_string().as_str());
    //         }
    //         common::hashes::crc32(name.as_bytes())
    //     } else {
    //         let name = self.name.to_string();
    //         common::hashes::crc32(name.as_bytes())
    //     }
    // }
    // fn generate_variant_validation_match(&self, generate_value: bool) -> TokenStream {
    //     let mut first = true;
    //     let variants: Vec<_> = self
    //         .variants
    //         .iter()
    //         .map(|(name, value)| {
    //             let name = syn::Ident::new(name, proc_macro2::Span::call_site());
    //             let value = proc_macro2::Literal::i128_unsuffixed(*value);
    //             if generate_value {
    //                 quote! { #value => Some(Self::#name), }
    //             } else if first {
    //                 first = false;
    //                 quote! { #value }
    //             } else {
    //                 quote! { | #value }
    //             }
    //         })
    //         .collect();
    //     if generate_value {
    //         quote! {
    //             match value {
    //                 #(#variants)*
    //                 _ => None,
    //             }
    //         }
    //     } else {
    //         quote! {
    //             match value {
    //                 #(#variants)* => {},
    //                 _ => return None,
    //             }
    //         }
    //     }
    // }

    // fn generate_serde_implementation(&self) -> TokenStream {
    //     let name = &self.name;
    //     let data_format = self.repr.data_format();
    //     let repr_type = self.repr.repr_type();
    //     let name_hash = self.compute_hash();
    //     let variant_validation = self.generate_variant_validation_match(true);

    //     quote! {
    //         unsafe impl<'a> SerDe<'a> for #name {
    //             const DATA_FORMAT: flat_message::DataFormat = #data_format;
    //             #[inline(always)]
    //             unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
    //                 unsafe {
    //                     let ptr = buf.as_ptr().add(pos+4) as *const Self;
    //                     std::ptr::read_unaligned(ptr)
    //                 }
    //             }
    //             #[inline(always)]
    //             fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
    //                 if pos + std::mem::size_of::<#repr_type>() + 4 > buf.len() {
    //                     None
    //                 } else {
    //                     unsafe {
    //                         let hash = (buf.as_ptr().add(pos) as *const u32).read_unaligned();
    //                         if hash != #name_hash {
    //                             return None;
    //                         }
    //                         let value = ((buf.as_ptr().add(pos+4) as *const #repr_type)).read_unaligned();
    //                         #variant_validation
    //                     }
    //                 }
    //             }
    //             #[inline(always)]
    //             unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
    //                 unsafe {
    //                     std::ptr::write_unaligned(p.add(pos) as *mut u32, #name_hash);
    //                     std::ptr::write_unaligned(p.add(pos+4) as *mut #repr_type, *obj as #repr_type);
    //                     pos + std::mem::size_of::<#repr_type>()+4
    //                 }
    //             }
    //             #[inline(always)]
    //             fn size(_: &Self) -> usize {
    //                 std::mem::size_of::<#repr_type>()+4 /* name hashe */
    //             }
    //         }
    //     }
    // }
    pub fn generate_code(&self) -> TokenStream {
        //let serde_code = self.generate_serde_implementation();
        quote! {
            //#serde_code
        }
    }
}

impl TryFrom<syn::DeriveInput> for Variant {
    type Error = String;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {

        let mut sealed_enum = false;
        for attr in input.attrs.iter() {
            if attr.path().is_ident("sealed") {
                sealed_enum = true;
            }
        }

        let mut variants = Vec::new();
        let data_enum = match &input.data {
            Data::Enum(data_enum) => data_enum,
            _ => return Err("FlatMessageVariant can only be used on enums with variants of multiple types".to_string()),
        };

        for v in &data_enum.variants {
            let name = v.ident.clone();
            
            match &v.fields {
                Fields::Unit => {
                    variants.push((name.to_string(), None));
                }
                Fields::Unnamed(fields) => {
                    if fields.unnamed.len() != 1 {
                        return Err(format!(
                            "Variant `{}` must have exactly one Type associated !",
                            name
                        ));
                    }
                    let ty = fields.unnamed[0].ty.clone();
                    let ty_str = quote! {#ty}.to_string();
                    let ty_str_clone = ty_str.clone();
                    let mut dt = DataType::new(ty, ty_str);
                    let name_str = name.to_string();
                    for attr in fields.unnamed[0].attrs.iter() {
                        dt.parse_attr(attr, &name_str)?;
                    }
                    println!("Variant {} -> type: {} -> {:?}",name, ty_str_clone, dt.data_format);
                    variants.push((name.to_string(), Some(dt)));
                }
                Fields::Named(_) => {
                    return Err(format!(
                        "Variant `{}` must be unit (e.g. Variant) or a single-field tuple variant (e.g. Variant(Type) )",
                        name
                    ));
                }
            }
        }
        Ok(Self {
            name: input.ident,
            variants,
            sealed_enum,
        })
    }
}
