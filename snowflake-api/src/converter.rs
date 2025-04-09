//! Converter between rust types and snowflake types during query execution

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnowflakeType {
    #[serde(rename = "type")]
    pub kind: String,
    pub value: String,
}

/// a trait to represent a SnowflakeType conversion/// You could use it to convert your own custom types to snowflake values
pub trait ToSnowflakeType {
    fn to_snowflake(&self) -> SnowflakeType;
}


impl<T> From<T> for SnowflakeType
where
    T: ToSnowflakeType
{
    fn from(value: T) -> Self {
        value.to_snowflake()
    }
}

macro_rules! simple_impl {
    ($v:ty, $target:literal) => {
        impl ToSnowflakeType for $v {
            fn to_snowflake(&self) -> SnowflakeType {
                SnowflakeType {
                    kind: $target.to_string(),
                    value: self.to_string(),
                }
            }
        }
        
    };
}

simple_impl!(i16, "NUMBER");
simple_impl!(i32, "NUMBER");
simple_impl!(i64, "NUMBER");
simple_impl!(u16, "NUMBER");
simple_impl!(u32, "NUMBER");
simple_impl!(u64, "NUMBER");
simple_impl!(f32, "FLOAT");
simple_impl!(f64, "FLOAT");
simple_impl!(String, "TEXT");
simple_impl!(&str, "TEXT");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_impl() {
        let sf = SnowflakeType::from("test");
        assert_eq!(sf.kind, "TEXT".to_string());
        assert_eq!(sf.value, String::from("3"))
    }

}