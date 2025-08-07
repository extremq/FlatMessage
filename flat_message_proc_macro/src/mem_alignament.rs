use quote::quote;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MemAlignment {
    Align8Bits,
    Align16Bits,
    Align32Bits,
    Align64Bits,
    Align128Bits
}
impl MemAlignment {
    pub(crate) fn data_format(&self) -> proc_macro2::TokenStream {
        match self {
            MemAlignment::Align8Bits => quote! {DataFormat::POD8},
            MemAlignment::Align16Bits => quote! {DataFormat::POD16},
            MemAlignment::Align32Bits => quote! {DataFormat::POD32},
            MemAlignment::Align64Bits => quote! {DataFormat::POD64},
            MemAlignment::Align128Bits => quote! {DataFormat::POD128},
        }
    }
    pub(crate) const fn align_size(&self) -> usize {
        match self {
            MemAlignment::Align8Bits => 1,
            MemAlignment::Align16Bits => 2,
            MemAlignment::Align32Bits => 4,
            MemAlignment::Align64Bits => 8,
            MemAlignment::Align128Bits => 16,
        }
    }
}
impl TryFrom<&str> for MemAlignment {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1" => Ok(MemAlignment::Align8Bits),
            "2" => Ok(MemAlignment::Align16Bits),
            "4" => Ok(MemAlignment::Align32Bits),
            "8" => Ok(MemAlignment::Align64Bits),
            "16" => Ok(MemAlignment::Align128Bits),
            _ => Err(format!("Invalid alignment representation: '{}' (allowed memory alignments are 1, 2, 4, 8 or 16 bytes)", value)),
        }
    }
}
