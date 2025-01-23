use lsp_types::Diagnostic;

use crate::{builder_error, core_ast::core::AstSymbol, document::Document, workspace::Workspace};

use super::{
    buildable::Buildable,
    symbol::{MaybePendingSymbol, PendingSymbol},
};

pub trait TryFromBuilder<T>: Sized
where
    T: Sized,
{
    type Error;
    fn try_from_builder(
        value: T,
        workspace: &mut Workspace,
        document: &Document,
    ) -> Result<Self, Self::Error>;
}
pub trait TryIntoBuilder<T>: Sized {
    type Error;
    fn try_into_builder(
        self,
        workspace: &mut Workspace,
        document: &Document,
    ) -> Result<T, Self::Error>;
}
impl<T, U> TryIntoBuilder<U> for T
where
    U: TryFromBuilder<T>,
{
    type Error = U::Error;
    fn try_into_builder(
        self,
        workspace: &mut Workspace,
        document: &Document,
    ) -> Result<U, Self::Error> {
        U::try_from_builder(self, workspace, document)
    }
}

pub trait TryDownCast<
    T: Buildable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    type Output;

    fn try_downcast(
        &self,
        workspace: &mut Workspace,
        document: &Document,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic>;
}

impl<T, Y> TryDownCast<T, Y> for PendingSymbol
where
    T: Buildable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    type Output = Y;
    fn try_downcast(
        &self,
        workspace: &mut Workspace,
        document: &Document,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.get_rc()
            .borrow()
            .downcast_ref::<T>()
            .ok_or(builder_error!(
                field_range,
                format!(
                    "Invalid {:?} for {:?}: received: {:?}",
                    field_name,
                    input_name,
                    workspace.parsers.tree_sitter.queries.core.capture_names()
                        [self.get_query_index()]
                )
            ))?
            .try_into_builder(workspace, document)
    }
}

impl<T, Y> TryDownCast<T, Y> for MaybePendingSymbol
where
    T: Buildable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    type Output = Option<Y>;

    fn try_downcast(
        &self,
        workspace: &mut Workspace,
        document: &Document,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.as_ref().map_or(Ok(None), |pending| {
            pending
                .try_downcast(workspace, document, field_name, field_range, input_name)
                .map(Some)
        })
    }
}

impl<T, Y, V> TryDownCast<Y, V> for Vec<T>
where
    T: TryDownCast<Y, V, Output = V>,
    Y: Buildable,
    V: AstSymbol + for<'a> TryFromBuilder<&'a Y, Error = lsp_types::Diagnostic>,
{
    type Output = Vec<V>;

    fn try_downcast(
        &self,
        workspace: &mut Workspace,
        document: &Document,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.iter()
            .map(|item| item.try_downcast(workspace, document, field_name, field_range, input_name))
            .collect::<Result<Vec<_>, lsp_types::Diagnostic>>()
    }
}
