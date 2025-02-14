use std::error::Error;

use auto_lsp::server::{InitOptions, LspOptions, Session};

use auto_lsp::python::PYTHON_PARSERS;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut session = Session::create(InitOptions {
        parsers: &PYTHON_PARSERS,
        lsp_options: LspOptions {
            workspace_symbols: true,
            document_symbols: true,
            diagnostics: true,
            inlay_hints: true,
            hover_info: true,
            code_lens: true,
            completions: true,
            ..Default::default()
        },
    })?;

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop()?;
    session.io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}
