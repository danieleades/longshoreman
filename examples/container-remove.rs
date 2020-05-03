use longshoreman::{Docker, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let id = "CONTAINER_ID";

    Docker::new()
        .containers()
        .remove(id)
        .force(true)
        .send()
        .await?;

    Ok(())
}
