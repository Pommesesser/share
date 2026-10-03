use futures_util::StreamExt;
use share_core::API_KEY;
use share_core::FileEntry;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;

pub const SERVER: &str = "https://share-server.de";

pub async fn upload(path: &str) -> String {
    let path = Path::new(path);

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("failed to read filename");

    let file = tokio::fs::File::open(path)
        .await
        .expect("failed to open file");
    let stream = ReaderStream::new(file);

    let response = reqwest::Client::new()
        .post(format!("{SERVER}/files"))
        .bearer_auth(API_KEY)
        .header("x-file-name", filename)
        .body(reqwest::Body::wrap_stream(stream))
        .send()
        .await
        .expect("failed to send request");

    if !response.status().is_success() {
        panic!("server returned {}", response.status());
    }

    response.text().await.expect("failed to read response")
}

pub async fn get(id: &str) -> String {
    let response = reqwest::Client::new()
        .get(format!("{SERVER}/files/{id}"))
        .bearer_auth(API_KEY)
        .send()
        .await
        .expect("failed to get a response");

    if !response.status().is_success() {
        panic!("server returned {}", response.status());
    }

    let filename = response
        .headers()
        .get("x-file-name")
        .expect("missing filename")
        .to_str()
        .expect("invalid filename header")
        .to_owned();

    let mut file = tokio::fs::File::create(&filename)
        .await
        .expect("failed to create file");

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.expect("failed to read response chunk");

        file.write_all(&chunk).await.expect("failed to write file");
    }

    filename
}

pub async fn list() -> Vec<FileEntry> {
    let response = reqwest::Client::new()
        .get(format!("{SERVER}/files"))
        .bearer_auth(API_KEY)
        .send()
        .await
        .expect("failed to get a response");

    if !response.status().is_success() {
        panic!("server returned {}", response.status());
    }

    response
        .json::<Vec<FileEntry>>()
        .await
        .expect("failed to read response")
}

pub async fn remove(id: &str) -> String {
    let response = reqwest::Client::new()
        .delete(format!("{SERVER}/files/{id}"))
        .bearer_auth(API_KEY)
        .send()
        .await
        .expect("failed to send request");

    if !response.status().is_success() {
        panic!("server returned {}", response.status());
    }

    response.text().await.expect("failed to read response body")
}
