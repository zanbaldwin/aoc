use crate::error::Error;
use crate::models::City;
use pathfinding::prelude::astar;

pub fn process(input: &str) -> Result<String, Error> {
    let map = City::try_from(input)?.map_bottom_right();

    let Some((_edges_taken, total_temperature_loss)) = astar(
        &map.start()?,
        |edge| map.get_edges(edge).into_iter().map(|edge| {
            let cost = edge.get_cost();
            (edge.clone(), cost)
        }).collect::<Vec<_>>(),
        |edge| map.get_heuristic(edge),
        |edge| map.is_complete(edge),
    ) else {
        return Err(Error::ExhaustiveSearch);
    };

    Ok(total_temperature_loss.to_string())
}

#[cfg(test)]
mod tests {
    use crate::TEST_INPUT;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("102", process(TEST_INPUT).unwrap());
    }
}
