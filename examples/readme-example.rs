use longshoreman::{Docker, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let docker = Docker::new();

    // Pull an image
    docker.images().pull("ubuntu").tag("latest").send().await?;

    // Create a container
    let response = docker
        .containers()
        .create("ubuntu")
        .name("my-container")
        .send()
        .await?;
    let id = response.id();

    // Remove the container
    docker.containers().remove(id).force(true).send().await?;

    Docker::new()
        .containers()
        .remove(id)
        .force(true)
        .send()
        .await?;

    Ok(())
}
