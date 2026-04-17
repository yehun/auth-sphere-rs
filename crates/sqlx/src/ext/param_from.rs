use crate::ext::Param;
use chrono::{NaiveDate, NaiveDateTime};

macro_rules! impl_from_num_for_param {
    ($t:ty, $v:ident) => {
        impl From<$t> for Param {
            fn from(value: $t) -> Self {
                Param::$v(value)
            }
        }

        impl From<&$t> for Param {
            fn from(value: &$t) -> Self {
                Param::$v(*value)
            }
        }
    };
}

macro_rules! impl_from_unsigned_num_for_param {
    ($t:ty, $v:ident, $target:ty) => {
        impl From<$t> for Param {
            fn from(value: $t) -> Self {
                Param::$v(value as $target)
            }
        }

        impl From<&$t> for Param {
            fn from(value: &$t) -> Self {
                Param::$v(*value as $target)
            }
        }
    };
}

macro_rules! impl_from_clone_for_param {
    ($t:ty, $v:ident) => {
        impl From<$t> for Param {
            fn from(value: $t) -> Self {
                Param::$v(value.to_owned())
            }
        }

        impl From<&$t> for Param {
            fn from(value: &$t) -> Self {
                Param::$v(value.clone())
            }
        }
    };
}

impl_from_num_for_param!(bool, Bool);
impl_from_num_for_param!(i16, Short);
impl_from_num_for_param!(i32, Int);
impl_from_num_for_param!(i64, Long);
impl_from_num_for_param!(f32, Float);
impl_from_num_for_param!(f64, Double);

impl_from_unsigned_num_for_param!(u8, Short, i16);
impl_from_unsigned_num_for_param!(u16, Short, i16);
impl_from_unsigned_num_for_param!(u32, Int, i32);
impl_from_unsigned_num_for_param!(u64, Long, i64);

impl_from_clone_for_param!(NaiveDate, Date);
impl_from_clone_for_param!(NaiveDateTime, DateTime);
impl_from_clone_for_param!(String, String);
impl From<&str> for Param {
    fn from(value: &str) -> Self {
        Param::String(value.to_owned())
    }
}

macro_rules! impl_from_array_for_param {
    ( $( $t:ty) * ) => {
        $(
            impl From<Vec<$t>> for Param {
                fn from(value: Vec<$t>) -> Self {
                    Self::Array(value.iter().map(|v| Param::from(v)).collect())
                }
            }
        )*
    };
}
impl_from_array_for_param!(bool i16 i32 i64 u16 u32 u64 f32 f64 String NaiveDate NaiveDateTime);
