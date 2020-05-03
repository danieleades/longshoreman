use longshoreman::{Docker, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let containers = Docker::new().containers();

    // Create a simple container
    containers.create("alpine").send().await?;

    // Create a more complex example
    containers
        .create("alpine")
        .name("my-cool-container")
        .send()
        .await?;

    Ok(())
}
