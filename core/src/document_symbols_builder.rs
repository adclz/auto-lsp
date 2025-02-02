use lsp_types::DocumentSymbol;

/// A builder for managing and assembling a collection of [`DocumentSymbol`]s.
#[derive(Default)]
pub struct DocumentSymbolsBuilder {
    document_symbols: Vec<DocumentSymbol>,
}

impl DocumentSymbolsBuilder {
    /// Adds a [`DocumentSymbol`] to the builder.
    pub fn push_symbol(&mut self, document_symbol: DocumentSymbol) {
        self.document_symbols.push(document_symbol);
    }

    /// Consumes the builder and returns the list of [`DocumentSymbol`]s.
    pub fn finalize(self) -> Vec<DocumentSymbol> {
        self.document_symbols
    }
}
