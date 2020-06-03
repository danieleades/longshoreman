use longshoreman::{Docker, Result};

#[tokio::test]
async fn volume() -> Result<()> {
    let volumes = Docker::new().volumes();

    let volume1 = volumes
        .create()
        .name("my-volume")
        .driver("local")
        .driver_opt("type", "tmpfs")
        .driver_opt("device", "tmpfs")
        .driver_opt("o", "size=100m,uid=1000")
        .label("key", "value")
        .send()
        .await?;

    let volume2 = volumes.inspect("my-volume").await?;

    assert_eq!(volume1, volume2);

    Ok(())
}
