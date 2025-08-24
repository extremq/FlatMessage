use std::collections::HashMap;

use crate::attribute_parser;

use super::utils;
use common::data_format::DataFormat;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_str;
use syn::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FieldType {
    Object,
    Slice,
    Vector,
}

impl FieldType {
    pub(crate) fn serde_trait(&self) -> &'static str {
        match self {
            FieldType::Object => "SerDe",
            FieldType::Slice => "SerDeSlice",
            FieldType::Vector => "SerDeVec",
        }
    }
}

pub(crate) struct DataType {
    pub(crate) field_type: FieldType,
    pub(crate) data_format: DataFormat,
    pub(crate) name: String,
    pub(crate) ty: syn::Type,
    pub(crate) unique_id: bool,
    pub(crate) timestamp: bool,
    pub(crate) ignore_field: bool,
    pub(crate) option: bool,
    pub(crate) mandatory: bool,
    pub(crate) use_default_if_deserialize_fails: bool,
    pub(crate) default_value: Option<String>,
}

impl DataType {
    pub(crate) fn type_hash(&self) -> u32 {
        match self.field_type {
            FieldType::Object => self.data_format as u32,
            FieldType::Slice | FieldType::Vector => (self.data_format as u32) | 0x80,
        }
    }
    #[inline(always)]
    pub(crate) fn serde_trait(&self) -> syn::Ident {
        syn::Ident::new(
            self.field_type.serde_trait(),
            proc_macro2::Span::call_site(),
        )
    }
    pub(crate) fn new(ty: syn::Type, mut def: String) -> Self {
        utils::type_name_formatter(&mut def);
        let mut option = false;
        if def.starts_with("Option<") && def.ends_with(">") {
            def = def["Option<".len()..def.len() - 1].to_string();
            option = true;
        }
        let field_type = if def.starts_with("Vec<") && def.ends_with(">") {
            def = def["Vec<".len()..def.len() - 1].to_string();
            FieldType::Vector
        } else if def.starts_with("&[") && def.ends_with("]") {
            if DataFormat::from(&def[1..def.len()]) == DataFormat::FixArray {
                // this wil be treated as an object (&[u8; N])
                def = def[1..def.len()].to_string();
                FieldType::Object
            } else {
                def = def[2..def.len() - 1].to_string();
                FieldType::Slice
            }
        } else {
            FieldType::Object
        };
        //println!(" -------- DataType: {def} 3");
        let unique_id = matches!(def.as_str(), "UniqueID" | "flat_message :: UniqueID");
        let timestamp = matches!(def.as_str(), "Timestamp" | "flat_message :: Timestamp");
        let zst = def.starts_with("PhantomData")
            || def.starts_with("std :: marker :: PhantomData")
            || def.starts_with("marker :: PhantomData");
        DataType {
            field_type,
            data_format: DataFormat::from(def.as_str()),
            name: def,
            ty,
            unique_id,
            timestamp,
            ignore_field: zst,
            option,
            mandatory: !option,
            default_value: None,
            use_default_if_deserialize_fails: false,
        }
    }

    pub(crate) fn parse_attr(&mut self, attr: &Attribute, field_name: &str) -> Result<(), String> {
        if attr.path().is_ident("flat_message_item") {
            let all_tokens = attr.meta.clone().into_token_stream();
            let mut tokens = TokenStream::default();
            let mut iter = all_tokens.into_iter();
            while let Some(token) = iter.next() {
                if let proc_macro2::TokenTree::Group(group) = token {
                    if group.delimiter() == proc_macro2::Delimiter::Parenthesis {
                        tokens = group.stream().into();
                        break;
                    }
                }
            }
            let mut attr = attribute_parser::parse(tokens);
            // println!("Field name: {}", field_name);
            // println!("Attr: {:?}", attr);
            self.update_attributes(&mut attr, field_name)?;
            self.update_default_value(&mut attr, field_name)?;
        }
        Ok(())
    }
    fn update_default_value(
        &mut self,
        attr: &mut HashMap<String, String>,
        field_name: &str,
    ) -> Result<(), String> {
        if let Some(value) = attr.remove("default") {
            self.default_value = Some(value);
        }
        // some basic checks
        // 1. we should provide a default value for &str
        if self.name == "&str" {
            if self.default_value.is_none() {
                // empty string
                self.default_value = Some("\"\"".to_string());
            } else {
                let mut value = self.default_value.take().unwrap();
                value.insert_str(0, "r#\"");
                value.push_str("\"#");
                self.default_value = Some(value);
            }
        }

        Ok(())
    }
    fn update_attributes(
        &mut self,
        attr: &mut HashMap<String, String>,
        field_nane: &str,
    ) -> Result<(), String> {
        if attr.len() == 0 {
            return Err(format!("No attributes provided for field: '{}'. You can only provide one of the following attributes: 'kind', 'repr' or 'align'.",field_nane));
        }

        let has_repr = attr.contains_key("repr");
        let has_kind = attr.contains_key("kind");
        let has_align = attr.contains_key("align");
        let has_mandatory = attr.contains_key("mandatory");
        let has_validate = attr.contains_key("validate");
        let ignore_field = if attr.contains_key("ignore") {
            utils::to_bool(attr.get("ignore").unwrap()).unwrap_or(false)
        } else if attr.contains_key("skip") {
            utils::to_bool(attr.get("skip").unwrap()).unwrap_or(false)
        } else {
            false
        };
        if has_mandatory {
            self.mandatory = utils::to_bool(attr.get("mandatory").unwrap()).unwrap_or(true);
        }
        if has_validate {
            match attr.get("validate").unwrap().as_str() {
                "strict" => self.use_default_if_deserialize_fails = false,
                "fallback" => self.use_default_if_deserialize_fails = true,
                _ => return Err(format!("Invalid value for the 'validate' attribute: '{}' in field: '{}'. The possible values are: 'strict' or 'fallback'.",attr.get("validate").unwrap(), field_nane)),
            }
        }
        if ignore_field {
            self.ignore_field = true;
            return Ok(());
        } else {
            if has_kind {
                let kind = attr.get("kind").unwrap();
                if kind == "enum" {
                    if !has_repr {
                        return Err(format!("If we provided the 'kind' attribute with the value 'enum' you need to also provide the attribute 'repr' (for field: '{}')",field_nane));
                    }
                    let repr = attr.get("repr").unwrap();
                    let new_name = format!("enum_{}", repr);
                    let new_data_format = DataFormat::from(new_name.as_str());
                    if new_data_format.is_enum() == false {
                        return Err(format!("Invalid representation for an enum: '{}' in field: '{}'. The possible representations for an enum are: u8, u16, u32, u64, i8, i16, i32 and i64.",repr, field_nane));
                    }
                    self.data_format = new_data_format;
                    return Ok(());
                }
                if kind == "flags" {
                    if !has_repr {
                        return Err(format!("If we provided the 'kind' attribute with the value 'flags' you need to also provide the attribute 'repr' (for field: '{}')",field_nane));
                    }
                    let repr = attr.get("repr").unwrap();
                    let new_name = format!("flags_{}", repr);
                    let new_data_format = DataFormat::from(new_name.as_str());
                    if new_data_format.is_flags() == false {
                        return Err(format!("Invalid representation for flags: '{}' in field: '{}'. The possible representations for flags are: u8, u16, u32, u64 and u128",repr, field_nane));
                    }
                    self.data_format = new_data_format;
                    return Ok(());
                }
                if kind == "struct" {
                    if !has_align {
                        return Err(format!("If we provided the 'kind' attribute with the value 'struct' you need to also provide the attribute 'align' (for field: '{}')",field_nane));
                    }
                    let align = attr.get("align").unwrap();
                    match align.as_str() {
                        "4" => self.data_format = DataFormat::Struct4,
                        "8" => self.data_format = DataFormat::Struct8,
                        "16" => self.data_format = DataFormat::Struct16,
                        _ => return Err(format!("Invalid alignment for a struct: '{}' in field: '{}'. The possible alignments for a struct are: 4, 8 and 16.",align, field_nane)),
                    };
                    return Ok(());
                }
                if kind == "variant" {
                    if !has_align {
                        return Err(format!("If we provided the 'kind' attribute with the value 'variant' you need to also provide the attribute 'align' (for field: '{}')",field_nane));
                    }
                    let align = attr.get("align").unwrap();
                    match align.as_str() {
                        "1" => self.data_format = DataFormat::Variant8,
                        "2" => self.data_format = DataFormat::Variant16,
                        "4" => self.data_format = DataFormat::Variant32,
                        "8" => self.data_format = DataFormat::Variant64,
                        "16" => self.data_format = DataFormat::Variant128,
                        _ => return Err(format!("Invalid alignment for a variant: '{}' in field: '{}'. The possible alignments for a variant are: 1,2,4,8 and 16.",align, field_nane)),
                    };
                    return Ok(());
                }
                return Err(format!(
                    "Invalid kind: '{}' in field: '{}'. The possible kinds are: 'enum', 'flags', 'struct' or 'variant'.",
                    kind, field_nane
                ));
            }
            // kind not present
            if has_repr {
                return Err(format!("If we provided the 'repr' attribute you need to also provide the attribute 'kind' (for field: '{}')",field_nane));
            }
            if has_align {
                return Err(format!("If we provided the 'align' attribute you need to also provide the attribute 'kind' (for field: '{}')",field_nane));
            }
            if has_mandatory || has_validate {
                return Ok(());
            }
            // check for other errors
            // possible parameters
            static KEYS: &[&'static str] = &[
                "kind",
                "repr",
                "align",
                "ignore",
                "skip",
                "mandatory",
                "default",
                "validate",
            ];
            for key in KEYS {
                if attr.contains_key(*key) {
                    continue;
                }
                return Err(format!(
                    "Unknown attribute '{}' in field: '{}'",
                    *key, field_nane
                ));
            }
            return Err(format!(
                "Invalid combination of attributes in field: '{}'. ",
                field_nane
            ));
        }
    }

    pub(crate) fn serialization_alignment(&self) -> usize {
        match self.field_type {
            FieldType::Object => {
                if self.data_format.is_object_container() {
                    self.data_format.alignament() as usize
                } else {
                    1
                }
            }
            FieldType::Slice | FieldType::Vector => self.data_format.alignament() as usize,
        }
    }

    pub(crate) fn default_value(&self) -> proc_macro2::TokenStream {
        let default_tokens = if let Some(default_value) = &self.default_value {
            let default_value_parsed: proc_macro2::TokenStream = parse_str(&default_value).unwrap();
            if self.option {
                quote! { Some(#default_value_parsed) }
            } else {
                quote! { #default_value_parsed }
            }
        } else {
            if self.option {
                quote! { None }
            } else {
                let ty = self.ty.clone();
                quote! { #ty::default() }
            }
        };
        default_tokens
    }
}
