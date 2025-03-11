use std::error::Error;

use auto_lsp::lsp_server::Connection;
use auto_lsp::python::PYTHON_PARSERS;
use auto_lsp::server::{InitOptions, LspOptions, Session};
use native_lsp::GetWorkspaceFiles;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();

    let mut session = Session::create(
        InitOptions {
            parsers: &PYTHON_PARSERS,
            lsp_options: LspOptions {
                ..Default::default()
            },
        },
        connection,
    )?;

    session.register_request::<GetWorkspaceFiles, _>(|session, _| {
        let guard = session.get_workspace();
        Ok(guard.roots.iter().map(|r| r.0.to_string()).collect())
    });

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop()?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}
