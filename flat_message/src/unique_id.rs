#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UniqueID {
    value: u64,
}
impl UniqueID {
    #[inline(always)]
    pub fn with_value(value: u64) -> Self {
        UniqueID { value }
    }

    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.value
    }
}
