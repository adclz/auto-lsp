use std::{path::PathBuf, process::Stdio, sync::Arc};

use assert_cmd::cargo::cargo_bin;
use escargot::CargoBuild;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{ChildStdin, ChildStdout, Command},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    jsonrpc: String,
    pub id: u64,
    pub result: T,
}

pub struct TestServer {
    stdin: ChildStdin,
    #[allow(dead_code)]
    read_handler: tokio::task::JoinHandle<()>,
    pub responses: Arc<tokio::sync::RwLock<Vec<String>>>,
}

impl TestServer {
    pub async fn stdio(testbed_dir: &PathBuf) -> Result<Self, std::io::Error> {
        let curr_dir = std::env::current_dir()?;
        assert!(curr_dir.exists());

        assert!(
            testbed_dir.exists(),
            "Workspace directory {:?} does not exist!",
            testbed_dir
        );

        let result = CargoBuild::new()
            .bin("stdio")
            .manifest_path("Cargo.toml")
            .run()
            .expect("Failed to build server");

        result.command().current_dir(&curr_dir).spawn()?;

        let bin = cargo_bin("stdio");

        let mut child = Command::new(bin)
            .current_dir(&curr_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let mut stdout = BufReader::new(stdout);

        let responses = Arc::new(tokio::sync::RwLock::new(vec![]));
        let responses_clone = responses.clone();
        let read_handler = tokio::task::spawn(async move {
            while let Ok(message) = TestServer::read_message(&mut stdout).await {
                eprintln!("Received: {}", message);
                responses_clone.write().await.push(message);
            }
        });

        Ok(Self {
            stdin,
            responses,
            read_handler,
        })
    }

    pub async fn read_message(
        stdout: &mut BufReader<ChildStdout>,
    ) -> Result<String, std::io::Error> {
        let mut headers = String::new();
        loop {
            let mut line = String::new();
            stdout.read_line(&mut line).await?;
            if line == "\r\n" {
                break; // End of headers
            }
            headers.push_str(&line);
        }

        // Extract Content-Length
        let content_length = headers
            .lines()
            .find_map(|line| {
                if line.to_lowercase().starts_with("content-length:") {
                    line["Content-Length:".len()..].trim().parse::<usize>().ok()
                } else {
                    None
                }
            })
            .unwrap_or(0);

        // Read full message body
        let mut body = vec![0; content_length];
        stdout.read_exact(&mut body).await?;

        Ok(String::from_utf8_lossy(&body).to_string())
    }

    pub async fn write_message(&mut self, message: &str) -> std::io::Result<()> {
        let msg = format!("Content-Length: {}\r\n\r\n{}", message.len(), message);
        self.stdin.write_all(msg.as_bytes()).await?;
        self.stdin.flush().await
    }
}
