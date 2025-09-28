// rustling-api/src/lib.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("backend error: {0}")]
    Backend(#[from] anyhow::Error),
}

pub trait Repository<T, ID> {
    fn find_all(&self) -> Result<Vec<T>, DataError>;

    fn find_by_id(&self, _id: &ID) -> Result<Option<T>, DataError> {
        unimplemented!("find_by_id not implemented")
    }

    fn save(&self, _entity: T) -> Result<T, DataError> {
        unimplemented!("save not implemented")
    }

    fn delete_by_id(&self, _id: &ID) -> Result<(), DataError> {
        unimplemented!("delete_by_id not implemented")
    }

    fn count(&self) -> Result<u64, DataError> {
        unimplemented!("count not implemented")
    }
}