use crawler::{error::CrawlerError, services::manager::Manager};

#[tokio::main]
async fn main() -> Result<(), CrawlerError> {
    let manager = Manager::new(20);

    manager
        .run(vec![
            "https://rust-unofficial.github.io/patterns/".into(),
            "https://www.rust-lang.org/".into(),
            "https://www.rust-lang.org/".into(),
            "https://github.com/EArnold1/crawler".into(),
        ])
        .await;

    Ok(())
}
