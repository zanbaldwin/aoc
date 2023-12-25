use error::Error;
use petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

pub mod error;
pub(crate) mod parser;
pub mod part1;

#[cfg(test)]
const TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

fn cut_wires(graph: &UnGraph<&str, ()>, num_wires: usize) -> Result<(usize, usize), Error> {
    let node_count = graph.node_count();
    // An error is never returned from the edge cost closure (all edges cost 1),
    // so we can safely unwrap the result.
    let result = stoer_wagner_min_cut(graph, |_| Ok::<usize, Error>(1))?;
    match result {
        Some((cut, partitioned_nodes)) => {
            if cut == 0 {
                return Err(Error::GraphIsDisconnected);
            }
            if cut != num_wires {
                return Err(Error::IncorrectNumberOfWiresCut(cut));
            }
            let a = partitioned_nodes.len();
            let b = node_count - a;
            Ok((a, b))
        }
        None => Err(Error::GraphNotBigEnough),
    }
}
