#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config {
    max_size: u32,
}
impl Config {
    /// Returns the maximum serialized size allowed (in bytes).
    ///
    /// If the serialized size exceeds this limit, an error is returned.
    #[inline(always)]
    pub fn max_size(&self) -> u32 {
        self.max_size
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            max_size: 16 * 1024 * 1024,
        }
    }
}

/// Builder for creating `Config` instances.
///
/// This struct provides a fluent interface for configuring the `Config` instance.
/// It allows you to set the maximum serialized size and build the `Config` instance.
pub struct ConfigBuilder {
    config: Config,
}

/// Builder for creating `Config` instances.
///
/// This struct provides a fluent interface for configuring the `Config` instance.
/// It allows you to set the maximum serialized size and build the `Config` instance.
impl ConfigBuilder {
    /// Creates a new `ConfigBuilder` instance with default configuration.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Sets the maximum serialized size allowed (in bytes).
    ///
    /// If the serialized size exceeds this limit, an error is returned.
    #[inline(always)]
    pub fn max_size(mut self, max_size: u32) -> Self {
        self.config.max_size = max_size;
        self
    }

    /// Builds the `Config` instance with the configured options.
    ///
    /// This method returns the `Config` instance with the specified options.
    #[inline(always)]
    pub fn build(self) -> Config {
        self.config
    }
}
