use futures_util::TryStreamExt;
use longshoreman::{Docker, Result};
use tempfile::NamedTempFile;
use tokio::fs::File;

#[tokio::test]
async fn single_images() -> Result<()> {
    let docker = Docker::new();
    let images = docker.images();

    let image = "alpine";

    // Pull image
    images.pull(image).tag("3.9").send().await?;

    let tmp_file = NamedTempFile::new()?;
    let export_to = tmp_file.reopen()?;

    // Export image
    images
        .get(&vec!["alpine:3.9"])
        .write(File::from_std(export_to))
        .await?;

    // Import
    let x = images
        .load(File::from_std(tmp_file.into_file()))
        .send()
        .try_collect::<Vec<_>>()
        .await?;
    assert_eq!(x, vec![("alpine".into(), "3.9".into())]);

    Ok(())
}

#[tokio::test]
async fn multiple_images() -> Result<()> {
    let docker = Docker::new();
    let images = docker.images();

    let image = "alpine";

    // Pull images
    for tag in vec!["3.9", "3.12"] {
        images.pull(image).tag(tag).send().await?;
    }

    let tmp_file = NamedTempFile::new()?;
    let export_to = tmp_file.reopen()?;

    // Export image
    images
        .get(&vec!["alpine:3.9", "alpine:3.12"])
        .write(File::from_std(export_to))
        .await?;

    // Import
    let x = images
        .load(File::from_std(tmp_file.into_file()))
        .send()
        .try_collect::<Vec<_>>()
        .await?;
    assert_eq!(
        x,
        vec![
            ("alpine".into(), "3.9".into()),
            ("alpine".into(), "3.12".into())
        ]
    );

    Ok(())
}
