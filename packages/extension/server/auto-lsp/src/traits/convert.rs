use std::convert::Infallible;

use super::workspace::WorkspaceContext;

pub trait TryFromCtx<T>: Sized
where
    T: Sized,
{
    type Error;

    fn try_from_ctx(value: T, ctx: &dyn WorkspaceContext) -> Result<Self, Self::Error>;
}

pub trait TryIntoCtx<T>: Sized {
    type Error;

    fn try_into_ctx(self, ctx: &dyn WorkspaceContext) -> Result<T, Self::Error>;
}

impl<T, U> TryIntoCtx<U> for T
where
    U: TryFromCtx<T>,
{
    type Error = U::Error;

    fn try_into_ctx(self, ctx: &dyn WorkspaceContext) -> Result<U, Self::Error> {
        U::try_from_ctx(self, ctx)
    }
}
