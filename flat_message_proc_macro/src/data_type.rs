use std::collections::HashMap;

use super::utils;
use common::data_format::DataFormat;

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
    pub(crate) zst: bool,
    pub(crate) option: bool
}

impl DataType {
    pub(crate) fn type_hash(&self) -> u32 {
        match self.field_type {
            FieldType::Object => self.data_format as u32,
            FieldType::Slice | FieldType::Vector => (self.data_format as u32) | 0x80,
        }
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
            def = def[2..def.len() - 1].to_string();
            FieldType::Slice
        } else {
            FieldType::Object
        };
        //println!(" -------- DataType: {def}");
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
            zst,
            option
        }
    }

    pub(crate) fn update(
        &mut self,
        attr: &HashMap<String, String>,
        field_nane: &str,
    ) -> Result<(), String> {
        let has_repr = attr.contains_key("repr");
        let has_kind = attr.contains_key("kind");
        if (!has_repr) && (!has_kind) {
            return Ok(());
        }
        if has_repr && !has_kind {
            return Err(format!("If we provided the 'repr' attribute you need to also provide the attribute 'kind' (for field: '{}')",field_nane));
        }
        if !has_repr && has_kind {
            return Err(format!("If we provided the 'kind' attribute you need to also provide the attribute 'repr' (for field: '{}')",field_nane));
        }
        // kind and repr are present
        let kind = attr.get("kind").unwrap();
        let repr = attr.get("repr").unwrap();
        if kind == "enum" {
            let new_name = format!("enum_{}", repr);
            let new_data_format = DataFormat::from(new_name.as_str());
            if new_data_format.is_enum() == false {
                return Err(format!("Invalid representation for an enum: '{}' in field: '{}'. The possible representations for an enum are: u8, u16, u32, u64, i8, i16, i32 and i64.",repr, field_nane));
            }
            self.data_format = new_data_format;
            return Ok(());
        }
        Err(format!(
            "Invalid kind: '{}' in field: '{}'. The possible kinds are: 'enum'.",
            kind, field_nane
        ))
    }

    pub(crate) fn serialization_alignment(&self) -> usize {
        match self.field_type {
            FieldType::Object => 1,
            FieldType::Slice | FieldType::Vector => self.data_format.alignament() as usize,
        }
    }
}
