use futures_util::TryStreamExt;
use longshoreman::{Docker, Result};
use tempfile::NamedTempFile;
use tokio::fs::File;

#[tokio::test]
async fn image_get_load() -> Result<()> {
    let docker = Docker::new();
    let images = docker.images();

    let image = "alpine";

    // Pull image
    images.pull(image).tag("latest").send().await?;

    let tmp_file = NamedTempFile::new()?;
    let export_to = File::from_std(tmp_file.reopen()?);

    // Export image
    images.get("alpine:latest", export_to).send().await?;

    // Import
    let x = images
        .load(File::from_std(tmp_file.into_file()))
        .send()
        .try_collect::<Vec<_>>()
        .await?;
    assert_eq!(x, vec![("alpine".into(), "latest".into())]);

    Ok(())
}
