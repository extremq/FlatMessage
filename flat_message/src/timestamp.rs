pub struct Timestamp {
    value: u64,
}
impl Timestamp {
    #[inline(always)]
    pub fn with_value(value: u64) -> Self {
        Self { value }
    }
    #[inline(always)]
    pub fn now() -> Self {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => Self {
                value: d.as_millis() as u64,
            },
            Err(_) => Self {
                value: 0
            }
        }
    }
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.value
    }
}
