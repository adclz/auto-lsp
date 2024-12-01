use super::ast_item::DynSymbol;
pub trait TryFromBuilder<T>: Sized
where
    T: Sized,
{
    type Error;
    fn try_from_builder(value: T, check: &mut Vec<DynSymbol>) -> Result<Self, Self::Error>;
}
pub trait TryIntoBuilder<T>: Sized {
    type Error;
    fn try_into_builder(self, check: &mut Vec<DynSymbol>) -> Result<T, Self::Error>;
}
impl<T, U> TryIntoBuilder<U> for T
where
    U: TryFromBuilder<T>,
{
    type Error = U::Error;
    fn try_into_builder(self, check: &mut Vec<DynSymbol>) -> Result<U, Self::Error> {
        U::try_from_builder(self, check)
    }
}
