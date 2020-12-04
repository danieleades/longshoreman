use longshoreman::{Docker, Result};
use tokio::{fs::File};
use futures_util::{StreamExt,  stream::{Stream}};

#[tokio::test]
async fn test_image_build() -> Result<()> {
    let docker = Docker::new();
    let images = docker.images();
    let containers = docker.containers();

    let image = "longshoreman/test";
    //replace filepath with absolute one
    let archive = File::open("sample_image.tar.gz").await?;

    // Pull image
    let mut response_stream = images
        .build(archive).label("key", "value").buildarg("key", "value").tag("longshoreman/test:1").dockerfile("Dockerfile")
        .with_progress();
    while let Some(response) = response_stream.next().await {
        println!("{:?}", response?)
    }
    // Create a simple container
    let id = containers
        .create(image)
        .name("my-cool-container")
        .send()
        .await?
        .id;

    // list containers and assert that it's there
    assert!(containers
        .list()
        .all(true)
        .limit(100)
        .send()
        .await?
        .into_iter()
        .any(|container| container.image == image));

    // Inspect it
    let _response = containers.inspect(&id).size(true).send().await?;

    containers.remove(&id).send().await?;

    Ok(())
}
