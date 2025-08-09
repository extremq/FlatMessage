pub trait FlagsSupport<T>
where
    T: Copy + Sized,
    Self: Sized,
{
    fn from_value(value: T) -> Option<Self>;
    fn to_value(&self) -> T;
    fn any_set(&self, flag: Self) -> bool;
    fn all_set(&self, flag: Self) -> bool;
    fn is_empty(&self) -> bool;
    fn set(&mut self, flag: Self);
    fn unset(&mut self, flag: Self);
    fn toggle(&mut self, flag: Self);
    fn clear(&mut self);
}
