use longshoreman::{Docker, Result};

#[tokio::test]
async fn test() -> Result<()> {
    let docker = Docker::new();
    let images = docker.images();
    let containers = docker.containers();

    let image = "alpine";

    // Pull image
    images.pull(image).tag("latest").send().await?;

    // Create a simple container
    let id = containers.create(image).send().await?.id().clone();

    containers.remove(&id).send().await?;

    Ok(())
}
