use longshoreman::{Docker, Result};

#[tokio::test]
async fn test() -> Result<()> {
    let containers = Docker::new().containers();

    // Create a simple container
    let id = containers.create("alpine").send().await?.id().clone();

    println!("got this far");

    containers.remove(&id).send().await?;

    Ok(())
}
