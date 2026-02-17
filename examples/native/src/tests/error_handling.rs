/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use rstest::{fixture, rstest};

use crate::requests::TriggerError;
use crate::server::TestServer;

pub static CLIENT_CAPABILITIES: &str = include_str!("client.json");

#[fixture]
async fn stdio_server() -> TestServer {
    let testbed_dir = std::env::current_dir().unwrap().join("src/testbed");
    let mut server = TestServer::stdio().await.unwrap();

    server
        .write_message(
            CLIENT_CAPABILITIES
                .replace(
                    "file:///testbed",
                    format!("file:///{}", testbed_dir.to_str().unwrap()).as_str(),
                )
                .as_str(),
        )
        .await
        .unwrap();

    server
        .write_message(r#"{"jsonrpc": "2.0", "method": "initialized", "params": {}}"#)
        .await
        .unwrap();

    server.wait_for_messages(1).await;

    assert!(server.responses.read().await[0].contains(r#""id":1"#));
    server.responses.write().await.clear();

    server
}

#[rstest]
#[tokio::test]
async fn trigger_error_request(
    #[future] mut stdio_server: TestServer,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdio_server = stdio_server.await;

    stdio_server
        .write_message(&TriggerError::request(2))
        .await?;

    stdio_server.wait_for_messages(1).await;

    let responses = stdio_server.responses.read().await;

    // The response should contain an error with code -32803 (RequestFailed)
    assert!(responses[0].contains(r#""error"#));
    assert!(responses[0].contains("test error"));
    assert!(responses[0].contains("-32803"));

    Ok(())
}
