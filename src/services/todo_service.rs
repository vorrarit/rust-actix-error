use std::error::Error;
use std::fmt::Display;

pub struct TodoService;

impl TodoService {
    pub fn add(description: &str) -> Result<String, TodoServiceError> {
        match description {
            "give me error" => Err(TodoServiceError("you 've asked for it".to_string())),
            _ => Ok(description.to_uppercase())
        }
    }
}

#[derive(Debug)]
pub struct TodoServiceError(String);

impl Display for TodoServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "error in todo service with reason - {}", self.0)
    }
}

impl Error for TodoServiceError {}