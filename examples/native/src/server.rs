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

use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Stdio, sync::Arc};

use assert_cmd::cargo::cargo_bin;
use escargot::CargoBuild;
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    sync::mpsc::channel,
};
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    jsonrpc: String,
    pub id: u64,
    pub result: T,
}

pub struct TestServer {
    writer_tx: tokio::sync::mpsc::Sender<String>,
    notify_rx: tokio::sync::mpsc::Receiver<()>,
    pub responses: Arc<tokio::sync::RwLock<Vec<String>>>,
}

impl TestServer {
    fn build_binary(curr_dir: &PathBuf) -> Result<(), std::io::Error> {
        let result = CargoBuild::new()
            .bin("native-lsp")
            .run()
            .expect("Failed to build server");

        result.command().current_dir(curr_dir).spawn()?;
        Ok(())
    }

    fn spawn_binary(curr_dir: &PathBuf) -> Result<Child, std::io::Error> {
        let bin = cargo_bin("native-lsp");

        Command::new(bin)
            .current_dir(curr_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }

    pub async fn stdio() -> Result<Self, std::io::Error> {
        let curr_dir = std::env::current_dir()?;

        TestServer::build_binary(&curr_dir)?;
        let mut child = TestServer::spawn_binary(&curr_dir)?;

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let mut stdout = BufReader::new(stdout);

        let responses = Arc::new(tokio::sync::RwLock::new(vec![]));
        let responses_clone = responses.clone();

        let (notify_tx, notify_rx) = tokio::sync::mpsc::channel::<()>(100);
        let (writer_tx, mut rx) = channel::<String>(1);

        // Read messages from the server
        tokio::task::spawn(async move {
            while let Ok(message) = TestServer::read_message(&mut stdout).await {
                responses_clone.write().await.push(message);
                let _ = notify_tx.send(()).await;
            }
        });

        // Write messages to the server
        tokio::task::spawn(async move {
            while let Some(message) = rx.recv().await {
                let msg = format!("Content-Length: {}\r\n\r\n{}", message.len(), message);
                stdin.write_all(msg.as_bytes()).await?;
                stdin.flush().await?;
            }
            Ok::<(), std::io::Error>(())
        });

        Ok(Self {
            notify_rx,
            writer_tx,
            responses,
        })
    }

    /// Waits until `n` messages have been received.
    pub async fn wait_for_messages(&mut self, n: usize) {
        for _ in 0..n {
            self.notify_rx.recv().await;
        }
    }

    async fn read_message<T: AsyncBufRead + std::marker::Unpin>(
        stdout: &mut T,
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

    pub async fn write_message(
        &mut self,
        message: &str,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<std::string::String>> {
        self.writer_tx.send(message.to_string()).await
    }
}
