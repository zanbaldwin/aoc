use crate::aoc_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Err(AocError::IoError(::std::io::Error::new(
        ::std::io::ErrorKind::Other, 
        "Not yet implemented.",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
}