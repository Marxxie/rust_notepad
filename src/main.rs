use std::{any::type_name, io, ops::Add, str::FromStr};

use iced::Element;

#[derive(Debug)]
enum RetrievalError {
    StdError,
    UnparseableError
}
struct Input;

impl Input {
    fn parse<T: FromStr>(parsee: String) -> Result<T, <T as FromStr>::Err> {
       parsee.parse::<T>()
    }

    fn retrieve<T: FromStr>() -> Result<T, RetrievalError> {
        let mut buffer = String::new();
        let result;

        if let Ok(_) = io::stdin().read_line(&mut buffer) {
           result = buffer.trim().to_owned()
        } else {
            return Err(RetrievalError::StdError)
        }

        match Self::parse::<T>(result) {
            Ok(parsed_result) => Ok(parsed_result),
            Err(_) => Err(RetrievalError::UnparseableError)
        }
    }

    fn request<T: FromStr>(message: Option<String>, retries: Option<usize>) -> Option<T> {
        let output;

        match message {
            Some(text) => output = text,
            None => output = "Please enter a {}.".to_owned().add(type_name::<T>())
        }

        eprintln!("{}", output);

        for _ in 0..retries.unwrap_or(5) {
          if let Ok(result) = Self::retrieve::<T>() {
            return Some(result)
          }
        };

        None
    }
}


fn main() {
    println!("Hello, world!");
}
