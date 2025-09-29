use std::marker::Tuple;

#[repr(transparent)]
pub struct IgnoreArgsMut<FnType>(pub(crate) FnType)
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
