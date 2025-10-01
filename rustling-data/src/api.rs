#[async_trait::async_trait]
pub trait Repository<T, ID> {
    async fn find_all(&self) -> Result<Vec<T>, anyhow::Error>;
}
