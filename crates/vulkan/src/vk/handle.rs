use super::ObjectType;

pub trait Handle: Copy {
    const TYPE: ObjectType;
    fn as_raw_u64(self) -> u64;
    fn from_raw_u64(value: u64) -> Self;
}
