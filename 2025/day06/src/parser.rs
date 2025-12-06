use crate::{Calculation, Error, Homework, Operation};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u32},
    multi::separated_list0,
    sequence::delimited,
};

fn construct(numbers: Vec<Vec<u32>>, operations: Vec<Operation>) -> Result<Homework, Error> {
    let rows = numbers.len();
    let columns = numbers.iter().map(|c| c.len()).max().unwrap_or(0);
    let mut calculations = vec![Vec::with_capacity(rows); columns];
    calculations.fill(Vec::with_capacity(columns));

    // Swap rows and columns. There must be an easier way of doing this, hopefully
    // something in the standard library, but I am unaware of such helpers. I want
    // clean, beautiful code and this is ugly :(
    'row: for row in numbers.iter() {
        for j in 0..columns {
            if let Some(number) = row.get(j) {
                let calculation = calculations.get_mut(j).expect("calculation should be pre-existing");
                calculation.push(*number);
            } else {
                continue 'row;
            }
        }
    }

    let problems: Vec<Calculation> = calculations
        .into_iter()
        .zip(operations.into_iter())
        .map(|(numbers, operation)| Calculation { numbers, operation })
        .collect();
    Ok(Homework { problems })
}

pub(crate) fn parse(input: &str) -> IResult<&str, Homework> {
    // What I'm trying to do:
    // separated_pair(parse_number_lines, newline, parse_operations)
    //     .map_res(|(numbers, operations)| construct(numbers, operations))
    //     .parse(input.trim())

    // What I have to do because the parse_number_lines() seems to be greedy and
    // consuming the newline before the line of operations making separated_pair() fail.
    // Which... ??? You would think that separated pair would only consume
    // separators that... separate?! Is there no non-consuming peeking in nom?!
    let (remaining, numbers) = parse_number_lines(input.trim())?;
    let (remaining, operations) = parse_operations(remaining.trim())?;
    let homework = construct(numbers, operations)
        .map_err(|_| nom::Err::Error(nom::error::Error::new(remaining, nom::error::ErrorKind::Verify)))?;
    Ok((remaining, homework))
}

fn parse_number_lines(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list0(newline, parse_numbers).parse(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(space0, separated_list0(space1, u32), space0).parse(input)
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    delimited(space0, separated_list0(space1, parse_operation), space0).parse(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((tag("+"), tag("*"))).map_res(|op: &str| op.try_into()).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("+", Operation::Addition)]
    #[case("*", Operation::Multiplication)]
    fn test_parse_operation(#[case] input: &str, #[case] expected: Operation) {
        let (_remaining, operation) = parse_operation(input).unwrap();
        assert_eq!(expected, operation);
    }

    #[test]
    fn test_parse_operation_invalid() {
        assert!(parse_operation("-").is_err());
    }

    #[rstest]
    #[case(" * +", &[Operation::Multiplication, Operation::Addition])]
    #[case("+  *", &[Operation::Addition, Operation::Multiplication])]
    #[case("+ +  +   + ", &[Operation::Addition, Operation::Addition, Operation::Addition, Operation::Addition])]
    fn test_parse_operations(#[case] input: &str, #[case] expected: &[Operation]) {
        let (_remaining, operations) = parse_operations(input).unwrap();
        assert_eq!(expected, &operations);
    }

    #[test]
    fn test_parse_operations_invalid() {
        assert!(parse_operations("* ++ *").is_err());
    }

    #[rstest]
    #[case("  1 2 3", &[1, 2, 3])]
    #[case("1", &[1])]
    #[case("", &[])]
    #[case("12      432 ", &[12, 432])]
    fn test_parse_numbers(#[case] input: &str, #[case] expected: &[u32]) {
        let (_remaining, numbers) = parse_numbers(input).unwrap();
        assert_eq!(expected, &numbers);
    }

    #[rstest]
    #[case("1\n2\n3", vec![vec![1], vec![2], vec![3]])]
    #[case("  1", vec![vec![1]])]
    #[case("", vec![vec![]])]
    #[case("12      432 ", vec![vec![12, 432]])]
    fn test_parse_number_lines(#[case] input: &str, #[case] expected: Vec<Vec<u32>>) {
        let (_remaining, numbers) = parse_number_lines(input).unwrap();
        assert_eq!(expected, numbers);
    }

    fn valid() -> Homework {
        Homework {
            problems: vec![
                Calculation {
                    numbers: vec![123, 45, 6],
                    operation: Operation::Multiplication,
                },
                Calculation {
                    numbers: vec![328, 64, 98],
                    operation: Operation::Addition,
                },
                Calculation {
                    numbers: vec![51, 387, 215],
                    operation: Operation::Multiplication,
                },
                Calculation {
                    numbers: vec![64, 23, 314],
                    operation: Operation::Addition,
                },
            ],
        }
    }

    #[test]
    fn test_construct() {
        let numbers = vec![vec![123, 328, 51, 64], vec![45, 64, 387, 23], vec![6, 98, 215, 314]];
        let operations = vec![
            Operation::Multiplication,
            Operation::Addition,
            Operation::Multiplication,
            Operation::Addition,
        ];
        let homework = construct(numbers, operations).unwrap();
        assert_eq!(homework, valid());
    }

    #[test]
    fn test_parse() {
        let input: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"#;
        let (_remaining, homework) = parse(input).unwrap();
        assert_eq!(homework, valid());
    }
}
