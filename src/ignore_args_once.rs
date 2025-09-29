use std::marker::Tuple;

/// A wrapper around a zero-argument [`FnOnce`] that discards any provided arguments.
///
/// This struct implements [`FnOnce<Args>`]
/// for **any argument tuple `Args`**, but calls the underlying function with `()`.
///
/// You can’t construct this type directly — instead use
/// [`ignore_args_once`](crate::ignore_args_once).
#[repr(transparent)]
pub struct IgnoreArgsOnce<FnType>(FnType)
where
    FnType: FnOnce<()>;

impl<FnType, Args, Output> FnOnce<Args> for IgnoreArgsOnce<FnType>
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

/// Wraps a closure or function that takes no arguments [`FnOnce`] into a callable
/// that ignores **any arguments** passed to it.
///
/// # Example
/// ```
/// use ignore_args::ignore_args_once;
///
/// let f = ignore_args_once(|| {
///     println!("runs once, args ignored");
/// });
///
/// f("anything", "here");
/// ```
#[inline(always)]
pub const fn ignore_args_once<FnType, Output>(function: FnType) -> IgnoreArgsOnce<FnType>
where
    FnType: FnOnce() -> Output,
{
    IgnoreArgsOnce(function)
}
