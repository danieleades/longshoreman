use async_std::{fs::File, stream::StreamExt};
use longshoreman::Docker;
use std::path::Path;

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
