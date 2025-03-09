use std::error::Error;

use auto_lsp::lsp_server::Connection;
use auto_lsp::lsp_types::CompletionOptions;
use auto_lsp::python::PYTHON_PARSERS;
use auto_lsp::server::{InitOptions, LspOptions, Session};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();

    let mut session = Session::create(
        InitOptions {
            parsers: &PYTHON_PARSERS,
            lsp_options: LspOptions {
                workspace_symbols: true,
                document_symbols: true,
                diagnostics: true,
                inlay_hints: true,
                hover_info: true,
                code_lens: true,
                completions: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        },
        connection,
    )?;

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop()?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}
