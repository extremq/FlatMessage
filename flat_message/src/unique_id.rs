use std::sync::atomic::AtomicU64;

static GLOBAL_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniqueID {
    value: u64,
}
impl UniqueID {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            value: GLOBAL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        }
    }

    #[inline(always)]
    pub fn with_value(value: u64) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.value
    }
}
