use crate::Error;

pub fn process(_input: &str) -> miette::Result<String, Error> {
    Err(Error::NotYetImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;
}
