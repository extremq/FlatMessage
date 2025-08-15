use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataFormat {
    Unknwon = 0,
    U8 = 1,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Bool,
    String,
    // enums
    EnumI8,
    EnumI16,
    EnumI32,
    EnumI64,
    EnumU8,
    EnumU16,
    EnumU32,
    EnumU64,
    // IPs
    IPv4,
    IPv6,
    IP,
    FixArray,
    Flags8,
    Flags16,
    Flags32,
    Flags64,
    Flags128,
    Struct4,
    Struct8,
    Struct16,
    POD8,
    POD16,
    POD32,
    POD64,
    POD128,
    // Rezerved
    // Hash8,
    // Hash16,
    // Hash32,
    // Hash64,
    // Path,
    // DateTime -> maybe from chronno
}
impl DataFormat {
    pub fn is_object_container(&self) -> bool {
        matches!(self, DataFormat::Struct4 | DataFormat::Struct8 | DataFormat::Struct16)
    }
    pub fn is_enum(&self) -> bool {
        match self {
            DataFormat::EnumI8
            | DataFormat::EnumI16
            | DataFormat::EnumI32
            | DataFormat::EnumI64
            | DataFormat::EnumU8
            | DataFormat::EnumU16
            | DataFormat::EnumU32
            | DataFormat::EnumU64 => true,
            _ => false,
        }
    }
    pub fn is_pod(&self) -> bool {
        match self {
            DataFormat::POD8
            | DataFormat::POD16
            | DataFormat::POD32
            | DataFormat::POD64
            | DataFormat::POD128 => true,
            _ => false,
        }
    }
    pub fn is_flags(&self) -> bool {
        match self {
            DataFormat::Flags8
            | DataFormat::Flags16
            | DataFormat::Flags32
            | DataFormat::Flags64
            | DataFormat::Flags128 => true,
            _ => false,
        }
    }    
    pub const fn alignament(&self) -> u8 {
        match self {
            DataFormat::U8 => 1,
            DataFormat::U16 => 2,
            DataFormat::U32 => 4,
            DataFormat::U64 => 8,
            DataFormat::U128 => 16,
            DataFormat::I8 => 1,
            DataFormat::I16 => 2,
            DataFormat::I32 => 4,
            DataFormat::I64 => 8,
            DataFormat::I128 => 16,
            DataFormat::F32 => 4,
            DataFormat::F64 => 8,
            DataFormat::Bool => 1,
            DataFormat::String => 1,
            DataFormat::EnumI8 => 1,
            DataFormat::EnumI16 => 2,
            DataFormat::EnumI32 => 4,
            DataFormat::EnumI64 => 8,
            DataFormat::EnumU8 => 1,
            DataFormat::EnumU16 => 2,
            DataFormat::EnumU32 => 4,
            DataFormat::EnumU64 => 8,
            DataFormat::IPv4 => 1,
            DataFormat::IPv6 => 1,
            DataFormat::IP => 1,
            DataFormat::FixArray => 1,
            DataFormat::Flags8 => 1,
            DataFormat::Flags16 => 2,
            DataFormat::Flags32 => 4,
            DataFormat::Flags64 => 8,
            DataFormat::Flags128 => 16,
            DataFormat::POD8 => 1,
            DataFormat::POD16 => 2,
            DataFormat::POD32 => 4,
            DataFormat::POD64 => 8,
            DataFormat::POD128 => 16,
            DataFormat::Struct4 => 4,
            DataFormat::Struct8 => 8,
            DataFormat::Struct16 => 16,
            DataFormat::Unknwon => 1,
        }
    }
}
impl Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataFormat::U8 => write!(f, "U8"),
            DataFormat::U16 => write!(f, "U16"),
            DataFormat::U32 => write!(f, "U32"),
            DataFormat::U64 => write!(f, "U64"),
            DataFormat::U128 => write!(f, "U128"),
            DataFormat::I8 => write!(f, "I8"),
            DataFormat::I16 => write!(f, "I16"),
            DataFormat::I32 => write!(f, "I32"),
            DataFormat::I64 => write!(f, "I64"),
            DataFormat::I128 => write!(f, "I128"),
            DataFormat::F32 => write!(f, "F32"),
            DataFormat::F64 => write!(f, "F64"),
            DataFormat::Bool => write!(f, "Bool"),
            DataFormat::String => write!(f, "String"),
            DataFormat::EnumI8 => write!(f, "EnumI8"),
            DataFormat::EnumI16 => write!(f, "EnumI16"),
            DataFormat::EnumI32 => write!(f, "EnumI32"),
            DataFormat::EnumI64 => write!(f, "EnumI64"),
            DataFormat::EnumU8 => write!(f, "EnumU8"),
            DataFormat::EnumU16 => write!(f, "EnumU16"),
            DataFormat::EnumU32 => write!(f, "EnumU32"),
            DataFormat::EnumU64 => write!(f, "EnumU64"),
            DataFormat::IPv4 => write!(f, "IPv4"),
            DataFormat::IPv6 => write!(f, "IPv6"),
            DataFormat::IP => write!(f, "IP"),
            DataFormat::FixArray => write!(f, "FixArray"),
            DataFormat::Flags8 => write!(f, "Flags8"),
            DataFormat::Flags16 => write!(f, "Flags16"),
            DataFormat::Flags32 => write!(f, "Flags32"),
            DataFormat::Flags64 => write!(f, "Flags64"),
            DataFormat::Flags128 => write!(f, "Flags128"),
            DataFormat::POD8 => write!(f, "POD8"),
            DataFormat::POD16 => write!(f, "POD16"),
            DataFormat::POD32 => write!(f, "POD32"),
            DataFormat::POD64 => write!(f, "POD64"),
            DataFormat::POD128 => write!(f, "POD128"),
            DataFormat::Struct4 => write!(f, "Struct4"),
            DataFormat::Struct8 => write!(f, "Struct8"),
            DataFormat::Struct16 => write!(f, "Struct16"),
            DataFormat::Unknwon => write!(f, "Unknwon"),
        }
    }
}

impl From<&str> for DataFormat {
    fn from(value: &str) -> Self {
        //println!("Value: {}", value);
        // check to see if value is in th form of [u8;<number>]
        if let Some(buf) = value.strip_prefix("[u8;") {
            if let Some(value) = buf.strip_suffix("]") {
                if let Ok(_) = value.trim().parse::<usize>() {
                    return DataFormat::FixArray;
                }
            }
        };
        match value {
            "u8" => DataFormat::U8,
            "u16" => DataFormat::U16,
            "u32" => DataFormat::U32,
            "u64" => DataFormat::U64,
            "u128" => DataFormat::U128,
            "i8" => DataFormat::I8,
            "i16" => DataFormat::I16,
            "i32" => DataFormat::I32,
            "i64" => DataFormat::I64,
            "i128" => DataFormat::I128,
            "f32" => DataFormat::F32,
            "f64" => DataFormat::F64,
            "bool" => DataFormat::Bool,
            "&str" => DataFormat::String,
            "String" => DataFormat::String,
            "enum_i8" => DataFormat::EnumI8,
            "enum_i16" => DataFormat::EnumI16,
            "enum_i32" => DataFormat::EnumI32,
            "enum_i64" => DataFormat::EnumI64,
            "enum_u8" => DataFormat::EnumU8,
            "enum_u16" => DataFormat::EnumU16,
            "enum_u32" => DataFormat::EnumU32,
            "enum_u64" => DataFormat::EnumU64,
            // ip
            "std :: net :: Ipv4Addr" | "net :: Ipv4Addr" | "Ipv4Addr" => DataFormat::IPv4,
            "std :: net :: Ipv6Addr" | "net :: Ipv6Addr" | "Ipv6Addr" => DataFormat::IPv6,
            "std :: net :: IpAddr" | "net :: IpAddr" | "IpAddr" => DataFormat::IP,
            // flags
            "flags_u8" => DataFormat::Flags8,
            "flags_u16" => DataFormat::Flags16,
            "flags_u32" => DataFormat::Flags32,
            "flags_u64" => DataFormat::Flags64,
            "flags_u128" => DataFormat::Flags128,
            // struct
            "struct_4" => DataFormat::Struct4,
            "struct_8" => DataFormat::Struct8,
            "struct_16" => DataFormat::Struct16,
            // copy struct
            "pod_1" => DataFormat::POD8,
            "pod_2" => DataFormat::POD16,
            "pod_4" => DataFormat::POD32,
            "pod_8" => DataFormat::POD64,
            "pod_16" => DataFormat::POD128,
            // the rest are considered generic objects
            _ => DataFormat::Unknwon,
        }
        // ;
        // println!("Name: {} -> {}", value, r);
        // r
    }
}
