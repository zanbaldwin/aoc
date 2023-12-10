use crate::error::Error;

pub fn process(_input: &str) -> Result<String, Error> {
    Err(Error::NotYetImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_input1() {
        assert_eq!(
            "4",
            process(
                ".....
.S-7.
.|.|.
.L-J.
....."
            )
            .unwrap()
        );
    }

    #[test]
    fn test_part1_input2() {
        assert_eq!(
            "8",
            process(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            )
            .unwrap()
        );
    }
}
