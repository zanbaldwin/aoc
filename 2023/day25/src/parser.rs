use crate::error::Error;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};
use std::collections::HashMap;

pub(crate) fn parse(input: &str) -> Result<UnGraph<&str, ()>, Error> {
    common::nom(parse_input, input)
}

fn parse_input(input: &str) -> IResult<&str, UnGraph<&str, ()>> {
    map(parse_list, |links| {
        let mut graph = UnGraph::<&str, ()>::new_undirected();

        let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
        links.iter().for_each(|(node_name, edges)| {
            let node_index = *nodes
                .entry(node_name)
                .or_insert_with(|| graph.add_node(node_name));

            edges.iter().for_each(|edge_name| {
                let edge_index = nodes
                    .entry(edge_name)
                    .or_insert_with(|| graph.add_node(edge_name));

                let edge_exists = graph
                    .edges(node_index)
                    .any(|edge_ref| edge_ref.target() == *edge_index);
                if !edge_exists {
                    graph.add_edge(node_index, *edge_index, ());
                }
            });
        });

        graph
    })(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(line_ending, parse_node)(input)
}

fn parse_node(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let separator = tuple((space0, tag(":"), space0));
    separated_pair(alpha1, separator, parse_edges)(input)
}

fn parse_edges(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, alpha1)(input)
}
