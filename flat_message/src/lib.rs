mod buffer;
mod config;
mod error;
mod flat_message;
mod flat_message_buffer;
pub mod headers;
mod metadata;
mod name;
mod serde;
mod unique_id;
pub mod size;
mod storage;
mod structure_information;
mod builder;

pub use self::config::Config;
pub use self::config::ConfigBuilder;
pub use self::error::Error;
pub use self::flat_message::FlatMessage;
pub use self::flat_message_buffer::FlatMessageBuffer;
pub use self::metadata::MetaData;
pub use self::metadata::MetaDataBuilder;
pub use self::name::Name;
pub use self::serde::SerDe;
pub use self::serde::SerDeSlice;
pub use self::serde::SerDeVec;
pub use self::storage::Storage;
pub use self::storage::VecLike;
pub use self::structure_information::StructureInformation;

pub use flat_message_proc_macro::*;

pub use common::data_format::DataFormat;
pub use common::hashes::crc32;
pub use unique_id::UniqueID;

pub trait FlatMessageOwned: for<'de> FlatMessage<'de> {}
impl<T> FlatMessageOwned for T where T: for<'de> FlatMessage<'de> {}
