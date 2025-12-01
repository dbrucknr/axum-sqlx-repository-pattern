use std::error::Error;

pub type Returns<T> = Result<T, Box<dyn Error>>;
