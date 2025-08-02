#[cfg(test)]
mod enums;
#[cfg(test)]
mod generic;
#[cfg(test)]
mod ip;
#[cfg(test)]
mod version;

pub(crate) use flat_message::{Config, FlatMessage, Storage};
pub(crate) use std::fmt::Debug;

pub(crate) fn validate_correct_serde<T>(obj: T)
where
    T: Eq + PartialEq + Debug + for<'a> crate::FlatMessage<'a>,
{
    let mut output = Vec::new();
    obj.serialize_to(&mut output, Config::default()).unwrap();
    let storage = Storage::from_buffer(&output);
    let deserialized = T::deserialize_from(&storage).unwrap();
    assert_eq!(obj, deserialized);
}

fn main() {
    println!("This is a test module for the flat_message crate.");
}
