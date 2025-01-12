use crate::builders::BuilderParams;

pub trait TryFromBuilder<T>: Sized
where
    T: Sized,
{
    type Error;
    fn try_from_builder(value: T, params: &mut BuilderParams) -> Result<Self, Self::Error>;
}
pub trait TryIntoBuilder<T>: Sized {
    type Error;
    fn try_into_builder(self, params: &mut BuilderParams) -> Result<T, Self::Error>;
}
impl<T, U> TryIntoBuilder<U> for T
where
    U: TryFromBuilder<T>,
{
    type Error = U::Error;
    fn try_into_builder(self, params: &mut BuilderParams) -> Result<U, Self::Error> {
        U::try_from_builder(self, params)
    }
}
