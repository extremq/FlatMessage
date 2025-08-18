use super::utils;
use crate::data_type::DataType;
use common::data_format::DataFormat;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput, Fields};

struct VariantItem {
    name: String,
    name_ident: syn::Ident,
    data_type: Option<DataType>,
    serde_trait: syn::Ident,
    extra_size: usize,
    hash: u32,
}
pub struct Variant {
    name: syn::Ident,
    variants: Vec<VariantItem>,
    sealed_enum: bool,
    data_format: DataFormat,
}

impl Variant {
    fn compute_hash(&self) -> u32 {
        if self.sealed_enum {
            let mut name = self.name.to_string();
            let mut v: Vec<_> = self.variants.iter().map(|v| v.name.as_str()).collect();
            v.sort();
            for variant_name in v {
                name.push_str(variant_name);
                name.push_str(",");
            }
            common::hashes::crc32(name.as_bytes())
        } else {
            let name = self.name.to_string();
            common::hashes::crc32(name.as_bytes())
        }
    }
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

    fn generate_serde_size(&self) -> TokenStream {
        let struct_name = self.name.clone();
        let mut v = Vec::new();
        for variant in &self.variants {
            let name = variant.name_ident.clone();
            let serde_trait = variant.serde_trait.clone();
            let extra_size = variant.extra_size;
            if let Some(_) = &variant.data_type {
                v.push(quote! {
                    #struct_name::#name(obj) => ::flat_message::#serde_trait::size(obj) + #extra_size,
                });
            } else {
                v.push(quote! {
                    #struct_name::#name => 8,
                });
            }
        }
        quote! {
            fn size(obj: &Self) -> usize {
                match obj {
                    #(#v)*
                }
            }
        }
    }
    fn generate_serde_write(&self) -> TokenStream {
        let struct_name = self.name.clone();
        let variant_name_hash = self.compute_hash();
        let mut v = Vec::new();
        for variant in &self.variants {
            let name = variant.name_ident.clone();
            let serde_trait = variant.serde_trait.clone();
            let extra_size = variant.extra_size;
            let hash = variant.hash;
            if let Some(_) = &variant.data_type {
                v.push(quote! {
                    #struct_name::#name(obj) => {
                        std::ptr::write_unaligned(p.add(pos+4) as *mut u32, #hash);
                        ::flat_message::#serde_trait::write(obj,p,pos+#extra_size)
                    }
                });
            } else {
                v.push(quote! {
                    #struct_name::#name => {
                        std::ptr::write_unaligned(p.add(pos+4) as *mut u32, #hash);
                        pos+8
                    }
                });
            }
        }
        quote! {
            unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                std::ptr::write_unaligned(p.add(pos) as *mut u32, #variant_name_hash);
                match obj {
                    #(#v)*
                }
            }
        }
    }
    fn generate_serde_from_buffer(&self) -> TokenStream {
        let variant_name_hash = self.compute_hash();
        let mut v = Vec::new();
        for variant in &self.variants {
            let name = variant.name_ident.clone();
            let serde_trait = variant.serde_trait.clone();
            let extra_size = variant.extra_size;
            let hash = variant.hash;
            if let Some(dt) = &variant.data_type {
                let ty = dt.ty.clone();
                v.push(quote! {
                    #hash => {
                        let obj: #ty = ::flat_message::#serde_trait::from_buffer(buf, pos+#extra_size)?;
                        Some(Self::#name(obj))
                    }
                });
            } else {
                v.push(quote! {
                    #hash=> Some(Self::#name),
                });
            }
        }

        quote! {
            fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
                if pos + 8 >= buf.len() {
                    return None;
                }
                let p = buf.as_ptr();
                let hash = unsafe { std::ptr::read_unaligned(p.add(pos) as *const u32) };
                if hash != #variant_name_hash {
                    return None;
                }
                let hash = unsafe { std::ptr::read_unaligned(p.add(pos+4) as *const u32) };
                match hash {
                    #(#v)*
                    _ => None
                }
            }
        }
    }
    fn generate_serde_from_buffer_unchecked(&self) -> TokenStream {
        let mut v = Vec::new();
        for variant in &self.variants {
            let name = variant.name_ident.clone();
            let serde_trait = variant.serde_trait.clone();
            let extra_size = variant.extra_size;
            let hash = variant.hash;
            if let Some(dt) = &variant.data_type {
                let ty = dt.ty.clone();
                v.push(quote! {
                    #hash => {
                        let obj: #ty = unsafe { ::flat_message::#serde_trait::from_buffer_unchecked(buf, pos+#extra_size) };
                        Self::#name(obj)
                    }
                });
            } else {
                v.push(quote! {
                    #hash=> Self::#name,
                });
            }
        }

        quote! {
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                let p = buf.as_ptr();
                let hash = unsafe { std::ptr::read_unaligned(p.add(pos+4) as *const u32) };
                match hash {
                    #(#v)*
                    _ => panic!("Invalid/Unknown variant !")
                }
            }
        }
    }
    pub fn generate_code(&self) -> TokenStream {
        let name = &self.name;
        let df = format_ident!("{}", self.data_format.to_string());
        let size_code = self.generate_serde_size();
        let from_buffer_code = self.generate_serde_from_buffer();
        let from_buffer_unchecked_code = self.generate_serde_from_buffer_unchecked();
        let write_code = self.generate_serde_write();

        quote! {
            unsafe impl<'a> SerDe<'a> for #name {
                const DATA_FORMAT: flat_message::DataFormat = flat_message::DataFormat::#df;

                #[inline(always)]
                #size_code
                #[inline(always)]
                #from_buffer_code
                #[inline(always)]
                #from_buffer_unchecked_code
                #[inline(always)]
                #write_code
            }
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
        let data_enum =
            match &input.data {
                Data::Enum(data_enum) => data_enum,
                _ => return Err(
                    "FlatMessageVariant can only be used on enums with variants of multiple types"
                        .to_string(),
                ),
            };

        let mut align = 1;
        for v in &data_enum.variants {
            let name = v.ident.clone();
            let name_str = name.to_string();
            let mut hash = common::hashes::crc32(name_str.as_bytes());
            match &v.fields {
                Fields::Unit => {
                    hash = (hash & 0xFFFFFF00) | 0xFF;
                    variants.push(VariantItem {
                        name: name.to_string(),
                        name_ident: name,
                        data_type: None,
                        serde_trait: syn::Ident::new("None", proc_macro2::Span::call_site()),
                        extra_size: 0,
                        hash,
                    });
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
                    let mut dt = DataType::new(ty, ty_str);
                    for attr in v.attrs.iter() {
                        dt.parse_attr(attr, &name_str)?;
                    }
                    align = align.max(dt.serialization_alignment());
                    let serde_trait = dt.serde_trait();
                    let extra_size = match dt.serialization_alignment() {
                        1 | 2 | 4 | 8 => 8,
                        16 => 16,
                        _ => panic!("Internal error: expected a Variant data format"),
                    };
                    // if the data format is unknown, we need to check if the field is a unique id or a timestamp
                    if dt.data_format == DataFormat::Unknwon {
                        return Err(format!("Please provide aditional specifications via #[flat_message_item(...)] for the field '{}' !", name));
                    }
                    if dt.unique_id {
                        return Err(format!("Unique IDs can not used inside a variant enum - for field {} in structure {} !", name, input.ident));
                    }
                    if dt.timestamp {
                        return Err(format!("Timestamp can not used inside a variant enum - for field {} in structure {} !", name, input.ident));
                    }
                    if dt.ignore_field {
                        return Err(format!("Ignore fields are not allowed in a variant enum - for field {} in structure {} !", name, input.ident));
                    }
                    hash = (hash & 0xFFFFFF00) | dt.type_hash();
                    variants.push(VariantItem {
                        name: name_str,
                        name_ident: name,
                        data_type: Some(dt),
                        serde_trait,
                        extra_size,
                        hash,
                    });
                }
                Fields::Named(_) => {
                    return Err(format!(
                        "Variant `{}` must be unit (e.g. Variant) or a single-field tuple variant (e.g. Variant(Type) )",
                        name
                    ));
                }
            }
        }
        let data_format = match align {
            1 => DataFormat::Variant8,
            2 => DataFormat::Variant16,
            4 => DataFormat::Variant32,
            8 => DataFormat::Variant64,
            16 => DataFormat::Variant128,
            _ => return Err(format!("Invalid alignment: {}", align)),
        };
        Ok(Self {
            name: input.ident,
            variants,
            sealed_enum,
            data_format,
        })
    }
}
