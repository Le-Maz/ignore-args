#![doc = include_str!("../README.md")]
#![feature(const_trait_impl, unboxed_closures, tuple_trait, fn_traits)]

mod ignore_args;
mod ignore_args_mut;
mod ignore_args_once;

pub use ignore_args::{IgnoreArgs, ignore_args};
pub use ignore_args_mut::{IgnoreArgsMut, ignore_args_mut};
pub use ignore_args_once::{IgnoreArgsOnce, ignore_args_once};
