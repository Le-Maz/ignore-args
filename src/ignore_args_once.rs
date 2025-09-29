use std::marker::Tuple;

#[repr(transparent)]
pub struct IgnoreArgsOnce<FnType>(pub(crate) FnType)
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
