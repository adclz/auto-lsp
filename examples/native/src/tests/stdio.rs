use rstest::{fixture, rstest};

use super::server::TestServer;
use crate::{tests::server::JsonRpcResponse, GetWorkspaceFiles};

pub static CLIENT_CAPABILITIES: &str = include_str!("client-capabilities.json");

#[fixture]
async fn stdio_server() -> TestServer {
    let testbed_dir = std::env::current_dir().unwrap().join("src/testbed");
    let mut server = TestServer::stdio(&testbed_dir).await.unwrap();

    // Send client capabilities
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
    server
}

#[rstest]
#[tokio::test]
async fn workspace_folders(
    #[future] mut stdio_server: TestServer,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdio_server = stdio_server.await;

    stdio_server
        .write_message(GetWorkspaceFiles::REQUEST)
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let responses = stdio_server.responses.read().await;
    assert_eq!(responses[0], "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"capabilities\":{\"positionEncoding\":\"utf-16\",\"textDocumentSync\":2,\"workspace\":{\"workspaceFolders\":{\"changeNotifications\":true,\"supported\":true}}}}}");

    let workspace_response: JsonRpcResponse<Vec<String>> = serde_json::from_str(&responses[1])?;
    assert_eq!(workspace_response.id, 2);
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
