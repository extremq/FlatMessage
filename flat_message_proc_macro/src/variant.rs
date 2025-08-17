use crate::data_type::DataType;
use super::utils;
use common::data_format::DataFormat;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput, Fields};

pub struct Variant {
    name: syn::Ident,
    variants: Vec<(String, Option<DataType>)>,
    sealed_enum: bool,
    data_format: DataFormat,
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


    fn generate_serde_size(&self) -> TokenStream {
        let extra_size = match self.data_format {
            DataFormat::Variant8 |
            DataFormat::Variant16 |
            DataFormat::Variant32 |
            DataFormat::Variant64 => quote! {8},
            DataFormat::Variant128 => quote! {16},
            _ => panic!("Internal error: expected a Variant data format"),
        };
        let struct_name = self.name.clone();
        let mut v = Vec::new();
        for (name, opt_dt) in &self.variants {
            let name = syn::Ident::new(name, proc_macro2::Span::call_site());
            if let Some(dt) = opt_dt {
                let serde_trait = dt.serde_trait();
                v.push(quote! {
                    #struct_name::#name(obj) => ::flat_message::#serde_trait::size(obj) + #extra_size,
                });
            } else {
                v.push(quote! {
                    #struct_name::#name => #extra_size,
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
        quote! {
            unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                todo!()
            }
        }
    }
    fn generate_serde_from_buffer(&self) -> TokenStream {
        quote! {
            fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
                todo!()
            }
        }
    }      
    fn generate_serde_from_buffer_unchecked(&self) -> TokenStream {
        quote! {
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                todo!()
            }
        }
    }        
    pub fn generate_code(&self) -> TokenStream {
        let name = &self.name;
        let df = format_ident!("{}",self.data_format.to_string());
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
        let data_enum = match &input.data {
            Data::Enum(data_enum) => data_enum,
            _ => return Err("FlatMessageVariant can only be used on enums with variants of multiple types".to_string()),
        };

        let mut align = 1;
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
                    align = align.max(dt.serialization_alignment());
                    println!("Variant {} -> type: {} -> {:?}, align = {}",name, ty_str_clone, dt.data_format, dt.serialization_alignment());
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
            data_format
        })
    }
}
