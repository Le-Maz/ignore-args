use std::marker::Tuple;

/// A wrapper around a zero-argument [`Fn`] that discards any provided arguments.
///
/// This struct implements [`Fn<Args>`], [`FnMut<Args>`], and [`FnOnce<Args>`]
/// for **any argument tuple `Args`**, but calls the underlying function with `()`.
///
/// You can’t construct this type directly — instead use
/// [`ignore_args`](crate::ignore_args).
#[repr(transparent)]
pub struct IgnoreArgs<FnType>(FnType)
where
    FnType: Fn<()>;

impl<FnType, Args, Output> FnOnce<Args> for IgnoreArgs<FnType>
where
    FnType: Fn() -> Output,
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
    FnType: Fn() -> Output,
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

/// Wraps a closure or function that takes no arguments [`Fn`] into a callable
/// that ignores **any arguments** passed to it.
///
/// # Example
/// ```
/// use ignore_args::ignore_args;
///
/// fn say_hello() {
///     println!("Hello, world!");
/// }
///
/// let f = ignore_args(say_hello);
///
/// f(1, 2, 3); // prints "Hello, world!", arguments are discarded
/// f("foo", "bar"); // still prints "Hello, world!"
/// f(); // also works
/// ```
#[inline(always)]
pub const fn ignore_args<FnType, Output>(function: FnType) -> IgnoreArgs<FnType>
where
    FnType: Fn() -> Output,
{
    IgnoreArgs(function)
}
