use crate::error::Error;
use crate::{Config, Storage, VecLike};

pub trait FlatMessage<'a> {
    fn serialize_to<V: VecLike>(&self, output: &mut V, config: Config) -> Result<(), Error>;
    fn deserialize_from(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized;
    unsafe fn deserialize_from_unchecked(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized;
}
