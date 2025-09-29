use anyhow::Error;

pub trait Repository<T, ID> {
    fn find_all(&self) -> Result<Vec<T>, Error>;
}

pub trait HelloWorld {
    fn hello();
}