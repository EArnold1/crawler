use crawler::{error::CrawlerError, manager::ManagerBuilder};

#[tokio::main]
async fn main() -> Result<(), CrawlerError> {
    let mut manager =
        ManagerBuilder::new(vec!["https://rust-unofficial.github.io/patterns/".into()])
            .set_max_depth(20)
            .build();

    manager.run().await?;

    Ok(())
}
