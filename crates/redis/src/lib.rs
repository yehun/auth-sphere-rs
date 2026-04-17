#![allow(dead_code)]
// #![allow(unused_variables)]
#![allow(unconditional_recursion)]

pub mod env;

mod core;

pub use core::{
    RedisService,
    RedisServiceOpt,
    RedisServiceError,
    RedisLock, DistributedLock,
    RedisMap,
    RedisSet,
};

