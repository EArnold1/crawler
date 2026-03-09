use crawler::{error::CrawlerError, services::manager::ManagerBuilder};

#[tokio::main]
async fn main() -> Result<(), CrawlerError> {
    let manager = ManagerBuilder::new().max_depth(10).max_workers(10).build();

    manager
        .run(vec![
            "https://rust-unofficial.github.io/patterns/".into(),
            "https://www.rust-lang.org/".into(),
            "https://www.rust-lang.org/".into(),
            "https://github.com/EArnold1/crawler".into(),
            "https://medium.com/@datajournal/best-rust-html-parsers-c11cb68a503f".into(),
        ])
        .await;

    Ok(())
}
