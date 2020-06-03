use longshoreman::{Docker, Result};

#[tokio::test]
async fn create_inspect_remove() -> Result<()> {
    let docker = Docker::new();
    let images = docker.images();
    let containers = docker.containers();

    let image = "alpine";

    // Pull image
    images.pull(image).tag("latest").send().await?;

    // Create a simple container
    let id = containers
        .create(image)
        .name("my-cool-container")
        .send()
        .await?
        .id;

    // list containers and assert that it's there
    assert!(
        containers
            .list()
            .all(true)
            .limit(100)
            .send()
            .await?
            .into_iter()
            .any(|container| container.image == image)
    );

    // Inspect it
    let _response = containers.inspect(&id).size(true).send().await?;

    containers.remove(&id).send().await?;

    Ok(())
}
