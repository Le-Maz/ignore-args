#![feature(const_trait_impl, unboxed_closures, tuple_trait, fn_traits)]

use std::marker::Tuple;

#[repr(transparent)]
pub struct IgnoreArgs<FnType>(FnType)
where
    FnType: FnOnce<()>;

impl<FnType, Args, Output> FnOnce<Args> for IgnoreArgs<FnType>
where
    FnType: FnOnce() -> Output,
    Args: Tuple,
{
    type Output = Output;

    #[inline(always)]
    extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
        self.0()
    }
}

impl<FnType, Args, Output> FnMut<Args> for IgnoreArgs<FnType>
where
    FnType: FnMut() -> Output,
    Args: Tuple,
{
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, _: Args) -> Self::Output {
        self.0()
    }
}

impl<FnType, Args, Output> Fn<Args> for IgnoreArgs<FnType>
where
    FnType: Fn() -> Output,
    Args: Tuple,
{
    #[inline(always)]
    extern "rust-call" fn call(&self, _: Args) -> Self::Output {
        self.0()
    }
}

#[inline(always)]
pub const fn ignore_args<FnType, Output>(function: FnType) -> IgnoreArgs<FnType>
where
    FnType: [const] FnOnce() -> Output,
{
    IgnoreArgs(function)
}
