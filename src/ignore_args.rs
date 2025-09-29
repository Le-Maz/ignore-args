use std::marker::Tuple;

#[repr(transparent)]
pub struct IgnoreArgs<FnType>(pub(crate) FnType)
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
