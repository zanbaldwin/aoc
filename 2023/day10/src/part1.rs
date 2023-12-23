use crate::{error::Error, models::*};

pub fn process(input: &str) -> Result<String, Error> {
    let map: Map = input.try_into()?;
    let circuit = map.circuit()?;
    let max_distance = ((circuit.len() as f64) / 2f64).floor() as u64;
    Ok(max_distance.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::cell;

    #[test]
    fn test_cell_parsing() {
        assert_eq!(cell("S(34,12)"), Cell { position: (34, 12), pipe: Pipe::Start });
    }

    #[test]
    fn test_branches() {
        let input: &str = "
F7L
JS7
F-J
";
        let map: Map = input.try_into().unwrap();
        assert_eq!(cell("S(2,2)"), map.start);
        let branches = map.branches(map.start);

        assert!(branches.contains(&cell("7(2,1)")));
        assert!(branches.contains(&cell("7(3,2)")));
        // Shouldn't contain more than those two valid branches.
        assert_eq!(2, branches.len());
        // Should never contain the start position.
        assert!(!branches.contains(&cell("S(2,2)")));
        // According to the input, should not contain the cell directly south of
        // the start position.
        assert!(!branches.contains(&cell("-(2,3)")));
    }

    #[test]
    fn test_circuit() {
        let input: &str = "
.....
.S-7.
.|.|.
.L-J.
.....
";
        let map: Map = input.try_into().unwrap();
        let circuit = map.circuit().unwrap();
        assert_eq!(
            circuit,
            vec![
                cell("S(2,2)"),
                cell("-(3,2)"),
                cell("7(4,2)"),
                cell("|(4,3)"),
                cell("J(4,4)"),
                cell("-(3,4)"),
                cell("L(2,4)"),
                cell("|(2,3)"),
                cell("S(2,2)"),
            ]
        );
    }

    #[test]
    fn test_circuit_no_diagonals() {
        let input: &str = "
.....
.S-7.
.|.|.
.L-J.
.....
";
        let map: Map = input.try_into().unwrap();
        let circuit = map.circuit().unwrap();
        circuit.windows(2).for_each(|chunk| {
            let a_x = chunk[0].position.0 as i64;
            let a_y = chunk[0].position.1 as i64;
            let b_x = chunk[1].position.0 as i64;
            let b_y = chunk[1].position.1 as i64;
            let x = (a_x - b_x).abs();
            let y = (a_y - b_y).abs();
            let distance = x + y;
            assert_eq!(1, distance);
        });
    }

    #[test]
    fn test_part1_input1() {
        assert_eq!(
            "4",
            process(
                "
.....
.S-7.
.|.|.
.L-J.
.....
"
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

    #[test]
    fn test_part1_multi_branch() {
        let input: &str = "
.|...
-S-7.
.|.|.
.L-J.
.....
";
        let map: Map = input.try_into().unwrap();
        let circuit = map.circuit().unwrap();
        assert_eq!(
            circuit,
            vec![
                cell("S(2,2)"),
                cell("-(3,2)"),
                cell("7(4,2)"),
                cell("|(4,3)"),
                cell("J(4,4)"),
                cell("-(3,4)"),
                cell("L(2,4)"),
                cell("|(2,3)"),
                cell("S(2,2)"),
            ]
        );
    }

    #[test]
    fn test_shortest_possible_path() {
        let input: &str = "
.|..
.S7.
.LJ.
";
        let map: Map = input.try_into().unwrap();
        let circuit = map.circuit().unwrap();
        assert_eq!(
            circuit,
            vec![
                cell("S(2,2)"),
                cell("7(3,2)"),
                cell("J(3,3)"),
                cell("L(2,3)"),
                cell("S(2,2)"),
            ]
        );
    }

    #[test]
    fn test_does_not_go_back_to_start_unless_pointing() {
        let input: &str = "
LJF7FJF
7FJSJFJ
JL-J-JF
";
        let map: Map = input.try_into().unwrap();
        let circuit = map.circuit().unwrap();
        assert_eq!(
            circuit,
            vec![
                cell("S(4,2)"),
                cell("7(4,1)"),
                cell("F(3,1)"),
                cell("J(3,2)"),
                cell("F(2,2)"),
                cell("L(2,3)"),
                cell("-(3,3)"),
                cell("J(4,3)"),
                cell("S(4,2)"),
            ]
        );
    }
}
