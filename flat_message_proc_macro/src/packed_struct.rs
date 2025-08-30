use crate::data_type::FieldType;
use quote::quote;
use crate::field_info::FieldInfo;
use std::fmt::Write;

use syn::{DataStruct, DeriveInput};

pub(crate) struct PackedStruct<'a> {
    generics: &'a syn::Generics,
    name: &'a syn::Ident,
    fields: Vec<FieldInfo>,
    ignored_fields: Vec<FieldInfo>,
}

impl<'a> PackedStruct<'a> {
    pub(crate) fn generate_code(&self) -> proc_macro::TokenStream {
        quote!{}.into()
    }
    pub(crate) fn new(
        input: &'a DeriveInput,
        d: &'a DataStruct,
    ) -> Result<Self, String> {
        if let syn::Fields::Named(fields) = &d.fields {
            let mut data_members: Vec<FieldInfo> = Vec::with_capacity(32);
            let mut ignored_fields: Vec<FieldInfo> = Vec::new();
            let mut structure_hash = String::with_capacity(128);
            structure_hash.push_str(input.ident.to_string().as_str());
            structure_hash.push_str(",");

            for field in fields.named.iter() {
                let field = FieldInfo::new(field, None)?;
                if field.data_type.unique_id {
                    return Err(format!(
                        "Unique IDs are not supported for packed structures ! (for field {}) !",
                        field.name
                    ));
                } else if field.data_type.timestamp {
                    return Err(format!(
                        "Timestamp are not supported for packed structures ! (for field {}) !",
                        field.name
                    ));
                }
                if field.data_type.option {
                    return Err(format!("Option types (Option<T>)  are not supported for packed structures ! (for field {}) !", field.name));
                }
                if field.data_type.mandatory == false {
                    return Err(format!("In a packed structure, all fields must be mandatory ! (for field {}) ! Remove the `mandatory = false` attribute from #[flat_message_item(...)] description of this field !", field.name));
                }
                if field.data_type.ignore_field {
                    ignored_fields.push(field);
                } else
                {
                    structure_hash.push_str(field.name.as_str());
                    structure_hash.push_str("-");
                    let ty = &field.data_type.ty;
                    let ty_str = quote! {#ty}.to_string();
                    structure_hash.push_str(ty_str.as_str());
                    structure_hash.push_str("-");
                    write!(structure_hash, "{}:{}", field.data_type.serialization_alignment(),field.data_type.data_format as u8);
                    structure_hash.push_str(",");
                    data_members.push(field);
                }
            }
            if data_members.len() > 0xFFFF {
                return Err(format!("Structs with more than 65535 fields are not supported ! (Current structure has {} fields)", data_members.len()));
            }
            println!("Structure hash: {}", structure_hash);
            // now sort the key backwards based on their serialization alignment
            data_members.sort_unstable_by_key(|field_info| {
                usize::MAX - field_info.data_type.serialization_alignment()
            });
            Ok(PackedStruct {
                fields: data_members,
                generics: &input.generics,
                name: &input.ident,
                ignored_fields,
            })
        } else {
            Err("Can not read fields from the structure !".to_string())
        }
    }
}
