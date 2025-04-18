//! Converter between rust types and snowflake types during query execution

use serde::{Serialize, Deserialize};
use crate::responses::SnowflakeType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnowflakeBindingType {
    #[serde(rename = "type")]
    pub kind: SnowflakeType,
    pub value: String,
}

/// a trait to represent a SnowflakeType conversion/// You could use it to convert your own custom types to snowflake values
pub trait ToSnowflakeType {
    fn to_snowflake(&self) -> SnowflakeBindingType;
}


impl<T> From<T> for SnowflakeBindingType
where
    T: ToSnowflakeType
{
    fn from(value: T) -> Self {
        value.to_snowflake()
    }
}

macro_rules! simple_impl {
    ($v:ty, $target:expr) => {
        impl ToSnowflakeType for $v {
            fn to_snowflake(&self) -> SnowflakeBindingType {
                SnowflakeBindingType {
                    kind: $target,
                    value: self.to_string(),
                }
            }
        }
        
    };
}

simple_impl!(i16, SnowflakeType::Fixed);
simple_impl!(i32, SnowflakeType::Fixed);
simple_impl!(i64, SnowflakeType::Fixed);
simple_impl!(u16, SnowflakeType::Fixed);
simple_impl!(u32, SnowflakeType::Fixed);
simple_impl!(u64, SnowflakeType::Fixed);
simple_impl!(f32, SnowflakeType::Real);
simple_impl!(f64, SnowflakeType::Real);
simple_impl!(String, SnowflakeType::Text);
simple_impl!(&str, SnowflakeType::Text);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_impl() {
        let sf = SnowflakeBindingType::from("test");
        assert_eq!(sf.kind, SnowflakeType::Text);
        assert_eq!(sf.value, String::from("3"))
    }

}