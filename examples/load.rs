use async_std::io::prelude::*;
use async_std::fs::File;
use std::path::Path;
use longshoreman::Docker;
use async_std::stream::StreamExt;

#[tokio::main]
async fn main() {
    let archive_path = Path::new("../tests/artefacts/ubuntu.tar.gz");
    let archive = File::open(archive_path).await.unwrap();

    let images_client = Docker::new().images();

    let mut images_stream = Box::pin(images_client.load(archive).with_progress());

    while let Some(response) = images_stream.next().await {
        println!("{:?}", response.unwrap())
    }
}