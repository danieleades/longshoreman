use async_std::fs::File;
use longshoreman::Docker;
use async_std::stream::StreamExt;

#[tokio::test]
async fn load() {
    let images_client = Docker::new().images();

    let archive = File::open("tests/artefacts/ubuntu.tar.gz").await.expect("unable to read test archive from file system!");

    let mut images_stream = Box::pin(images_client.load(archive).send());

    let image_tuple = images_stream.next().await.unwrap().unwrap();

    assert_eq!(image_tuple, ("ubuntu".to_string(), "longshoremantest".to_string()));
}