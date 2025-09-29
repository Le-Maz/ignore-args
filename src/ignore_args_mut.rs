use std::marker::Tuple;

/// A wrapper around a zero-argument [`FnMut`] that discards any provided arguments.
///
/// This struct implements [`FnMut<Args>`], and [`FnOnce<Args>`]
/// for **any argument tuple `Args`**, but calls the underlying function with `()`.
///
/// You can’t construct this type directly — instead use
/// [`ignore_args_mut`](crate::ignore_args_mut).
#[repr(transparent)]
pub struct IgnoreArgsMut<FnType>(FnType)
where
    FnType: FnMut<()>;

impl<FnType, Args, Output> FnOnce<Args> for IgnoreArgsMut<FnType>
where
    FnType: FnMut() -> Output,
    Args: Tuple,
{
    type Output = Output;

    #[inline(always)]
    extern "rust-call" fn call_once(mut self, _: Args) -> Self::Output {
        self.0()
    }
}

impl<FnType, Args, Output> FnMut<Args> for IgnoreArgsMut<FnType>
where
    FnType: FnMut() -> Output,
    Args: Tuple,
{
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, _: Args) -> Self::Output {
        self.0()
    }
}

/// Wraps a closure or function that takes no arguments [`FnMut`] into a callable
/// that ignores **any arguments** passed to it.
///
/// # Example
/// ```
/// use ignore_args::ignore_args_mut;
///
/// let mut counter = 0;
/// let mut f = ignore_args_mut(|| {
///     counter += 1;
///     println!("called {counter} times");
/// });
///
/// f(123);
/// f("foo", "bar");
/// f();
/// ```
#[inline(always)]
pub const fn ignore_args_mut<FnType, Output>(function: FnType) -> IgnoreArgsMut<FnType>
where
    FnType: FnMut() -> Output,
{
    IgnoreArgsMut(function)
}
