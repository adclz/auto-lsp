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

use crate::requests::GetWorkspaceFiles;
use crate::server::{JsonRpcResponse, TestServer};

pub static CLIENT_CAPABILITIES: &str = include_str!("client.json");

#[fixture]
async fn stdio_server() -> TestServer {
    let testbed_dir = std::env::current_dir().unwrap().join("src/testbed");
    let mut server = TestServer::stdio().await.unwrap();

    // Send client capabilities
    // Workspace folder URI is prefixed with current directory
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

    // Initialized
    server
        .write_message(r#"{"jsonrpc": "2.0", "method": "initialized", "params": {}}"#)
        .await
        .unwrap();

    server.wait_for_messages(1).await;

    // Request with id 1 is the initialized response
    assert!(server.responses.read().await[0].contains(r#""id":1"#));
    server.responses.write().await.clear();

    server
}

#[rstest]
#[tokio::test]
async fn workspace_files(
    #[future] mut stdio_server: TestServer,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdio_server = stdio_server.await;

    // Workspace files are checked on initialization
    // We expect all files from the testbed directory to be present

    stdio_server
        .write_message(&GetWorkspaceFiles::request(2))
        .await?;

    stdio_server.wait_for_messages(1).await;

    let responses = stdio_server.responses.read().await;

    let workspace_response: JsonRpcResponse<Vec<String>> = serde_json::from_str(&responses[0])?;
    assert_eq!(workspace_response.id, 2);
    assert_eq!(workspace_response.result.len(), 3);

    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/file1.py"))
        .is_some());
    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/file2.py"))
        .is_some());
    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/nested/file3.py"))
        .is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn remove_file_notification(
    #[future] mut stdio_server: TestServer,
) -> Result<(), Box<dyn std::error::Error>> {
    let testbed_dir = std::env::current_dir().unwrap().join("src/testbed");
    let mut stdio_server = stdio_server.await;

    let file1_path_buf = testbed_dir.join("file1.py");
    let file1_path = file1_path_buf.to_str().unwrap();

    // We test if the server removes the file from the workspace
    // The file is still present in the testbed directory,
    // but after initializing the server it won't check the workspace anymore and will follow what the client sends

    let remove_file1 = format!(
        r#"{{
        "jsonrpc":"2.0",
        "method":"workspace/didChangeWatchedFiles",
        "params": {{
            "changes":[{{"uri":"file:{file1_path}","type":3}}]
        }}}}"#
    );

    stdio_server.write_message(remove_file1.as_str()).await?;

    stdio_server
        .write_message(&GetWorkspaceFiles::request(2))
        .await?;

    stdio_server.wait_for_messages(1).await;

    let responses = stdio_server.responses.read().await;

    let workspace_response: JsonRpcResponse<Vec<String>> = serde_json::from_str(&responses[0])?;
    assert_eq!(workspace_response.id, 2);
    assert_eq!(workspace_response.result.len(), 2);

    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/file2.py"))
        .is_some());
    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/nested/file3.py"))
        .is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn open_existing_document(
    #[future] mut stdio_server: TestServer,
) -> Result<(), Box<dyn std::error::Error>> {
    let testbed_dir = std::env::current_dir().unwrap().join("src/testbed");
    let mut stdio_server = stdio_server.await;

    let file1_path_buf = testbed_dir.join("file1.py");
    let file1_path = file1_path_buf.to_str().unwrap();

    // Sends a notification to open a text document
    // The server should not add the file to the workspace since it was already present at initialization

    let open_file_1 = format!(
        r#"{{
        "jsonrpc":"2.0",
        "method":"textDocument/didOpen",
        "params": {{
            "textDocument": {{
                "uri":"file:{file1_path}",
                "languageId":"python",
                "version":1,
                "text": ""
            }}
        }}}}"#
    );

    stdio_server.write_message(open_file_1.as_str()).await?;

    stdio_server
        .write_message(&GetWorkspaceFiles::request(2))
        .await?;

    stdio_server.wait_for_messages(1).await;

    let responses = stdio_server.responses.read().await;

    let workspace_response: JsonRpcResponse<Vec<String>> = serde_json::from_str(&responses[0])?;
    assert_eq!(workspace_response.id, 2);
    assert_eq!(workspace_response.result.len(), 3);

    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/file1.py"))
        .is_some());
    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/file2.py"))
        .is_some());
    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/nested/file3.py"))
        .is_some());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn open_new_document(
    #[future] mut stdio_server: TestServer,
) -> Result<(), Box<dyn std::error::Error>> {
    let testbed_dir = std::env::current_dir().unwrap().join("src/testbed");
    let mut stdio_server = stdio_server.await;

    let file4_path_buf = testbed_dir.join("file4.py");
    let file4_path = file4_path_buf.to_str().unwrap();

    // Sends a notification to open a newly created text document
    // The server should add the file to the workspace since it is not present at initialization

    let open_file_4 = format!(
        r#"{{
        "jsonrpc":"2.0",
        "method":"textDocument/didOpen",
        "params": {{
            "textDocument": {{
                "uri":"file:{file4_path}",
                "languageId":"py",
                "version":1,
                "text": "def o(): pass"
            }}
        }}}}"#
    );

    stdio_server.write_message(open_file_4.as_str()).await?;

    stdio_server
        .write_message(&GetWorkspaceFiles::request(2))
        .await?;

    stdio_server.wait_for_messages(1).await;

    let responses = stdio_server.responses.read().await;

    let workspace_response: JsonRpcResponse<Vec<String>> = serde_json::from_str(&responses[0])?;
    assert_eq!(workspace_response.id, 2);
    assert_eq!(workspace_response.result.len(), 4);

    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/file1.py"))
        .is_some());
    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/file2.py"))
        .is_some());
    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/nested/file3.py"))
        .is_some());
    assert!(workspace_response
        .result
        .iter()
        .find(|x| x.ends_with("testbed/file4.py"))
        .is_some());

    Ok(())
}
