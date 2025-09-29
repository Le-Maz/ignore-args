#![feature(const_trait_impl, unboxed_closures, tuple_trait, fn_traits)]

pub mod ignore_args;
pub mod ignore_args_mut;
pub mod ignore_args_once;

#[inline(always)]
pub const fn ignore_args<FnType, Output>(function: FnType) -> ignore_args::IgnoreArgs<FnType>
where
    FnType: Fn() -> Output,
{
    ignore_args::IgnoreArgs(function)
}

#[inline(always)]
pub const fn ignore_args_mut<FnType, Output>(function: FnType) -> ignore_args_mut::IgnoreArgsMut<FnType>
where
    FnType: FnMut() -> Output,
{
    ignore_args_mut::IgnoreArgsMut(function)
}

#[inline(always)]
pub const fn ignore_args_once<FnType, Output>(
    function: FnType,
) -> ignore_args_once::IgnoreArgsOnce<FnType>
where
    FnType: FnOnce() -> Output,
{
    ignore_args_once::IgnoreArgsOnce(function)
}
