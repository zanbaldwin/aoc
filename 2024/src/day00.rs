use aoc_common::{ParseError, RunnerError, Solution};

use std::{collections::HashMap, fmt::Display};

/// This is a practice run for Advent starting tomorrow (originally: 2023, Day 20).
pub struct Day00 {}
impl Solution for Day00 {
    type Parsed = models::Graph;

    fn day(&self) -> u8 {
        0
    }

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        let mut modules = parser::get_modules(input.trim())?;

        let connections: Vec<_> = modules
            .iter()
            .map(|(name, module)| {
                (name.to_owned(), module.destinations().iter().map(String::to_owned).collect::<Vec<_>>())
            })
            .collect();

        let mut conjunctions: HashMap<String, &mut models::ConjunctionModule> = modules
            .iter_mut()
            .filter_map(|(name, module_enum)| match module_enum {
                models::Module::Conjunction(module_struct) => Some((name.to_owned(), module_struct)),
                _ => None,
            })
            .collect();
        connections.iter().for_each(|(name, destinations)| {
            destinations.iter().for_each(|destination| {
                if let Some(conjunction) = conjunctions.get_mut(destination.as_str()) {
                    conjunction.state.entry(name.clone()).or_insert(models::Signal::Low);
                }
            });
        });

        Ok(modules)
    }

    fn part1(&self, input: &Self::Parsed) -> Result<impl Display, RunnerError> {
        let mut exec = models::Execution::new(input.clone());
        for _i in 0..1000 {
            exec.push_the_button();
            while !exec.is_queue_clear() {
                exec.tick();
            }
        }

        Ok(exec)
    }

    fn part2(&self, input: &Self::Parsed) -> Result<impl Display, RunnerError> {
        let mut exec = models::Execution::new(input.clone());
        // Yeah so this is another one of Eric's "you can't bruteforce your
        // way through this one, sucker" puzzles.
        return Err(RunnerError::Unimplemented);
        while !exec.machine_on {
            exec.push_the_button();
            while !exec.is_queue_clear() {
                exec.tick();
            }
        }
        Ok(exec.presses)
    }
}

mod models {
    use std::collections::HashMap;

    pub type ModuleName = String;
    pub type Graph = HashMap<ModuleName, Module>;

    trait Receiver {
        fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;
    }

    #[derive(Debug, Clone)]
    pub struct BroadcastModule {
        pub(crate) destinations: Vec<ModuleName>,
    }
    #[derive(Debug, Clone)]
    pub struct FlipFlopModule {
        pub(crate) on: bool,
        pub(crate) destinations: Vec<ModuleName>,
    }
    #[derive(Debug, Clone)]
    pub struct ConjunctionModule {
        pub(crate) state: HashMap<ModuleName, Signal>,
        pub(crate) destinations: Vec<ModuleName>,
    }
    #[derive(Debug, Clone)]
    pub enum Module {
        Broadcast(BroadcastModule),
        FlipFlop(FlipFlopModule),
        Conjunction(ConjunctionModule),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Signal {
        High,
        Low,
    }
    #[derive(Debug, Clone)]
    pub struct Pulse {
        source: ModuleName,
        signal: Signal,
        destination: ModuleName,
    }
    #[derive(Default)]
    struct PulsesCount {
        high: u32,
        low: u32,
    }
    pub struct Execution {
        graph: Graph,
        queue: Vec<Pulse>,
        total: PulsesCount,
        pub(crate) machine_on: bool,
        pub(crate) presses: u32,
    }

    impl Receiver for BroadcastModule {
        fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
            self.destinations
                .iter()
                .map(|d| Pulse {
                    source: pulse.destination.clone(),
                    signal: pulse.signal.clone(),
                    destination: d.to_owned(),
                })
                .collect()
        }
    }
    impl Receiver for FlipFlopModule {
        fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
            if let Signal::Low = pulse.signal {
                self.on = !self.on;
                let new_signal = match self.on {
                    true => Signal::High,
                    false => Signal::Low,
                };
                self.destinations
                    .iter()
                    .map(|d| Pulse {
                        source: pulse.destination.clone(),
                        signal: new_signal.clone(),
                        destination: d.to_owned(),
                    })
                    .collect()
            } else {
                vec![]
            }
        }
    }
    impl Receiver for ConjunctionModule {
        fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
            *self.state.entry(pulse.source).or_insert(Signal::Low) = pulse.signal;
            let new_signal = match self.state.values().all(|s| s == &Signal::High) {
                true => Signal::Low,
                false => Signal::High,
            };
            self.destinations
                .iter()
                .map(|d| Pulse {
                    source: pulse.destination.clone(),
                    signal: new_signal.clone(),
                    destination: d.to_owned(),
                })
                .collect()
        }
    }
    impl Module {
        pub fn destinations(&self) -> &Vec<ModuleName> {
            match self {
                Self::Broadcast(module) => &module.destinations,
                Self::FlipFlop(module) => &module.destinations,
                Self::Conjunction(module) => &module.destinations,
            }
        }
    }

    impl Execution {
        pub fn new(graph: Graph) -> Self {
            Self {
                graph,
                queue: Vec::new(),
                total: PulsesCount::default(),
                machine_on: false,
                presses: 0,
            }
        }

        pub(crate) fn push_the_button(&mut self) {
            self.presses += 1;
            self.fire(Pulse {
                source: "button".to_string(),
                signal: Signal::Low,
                destination: "broadcaster".to_string(),
            });
        }

        pub(crate) fn is_queue_clear(&self) -> bool {
            self.queue.is_empty()
        }

        fn fire(&mut self, pulse: Pulse) {
            match &pulse.signal {
                Signal::High => self.total.high += 1,
                Signal::Low => self.total.low += 1,
            };
            self.queue.push(pulse);
        }

        pub(crate) fn tick(&mut self) {
            self.queue.rotate_left(1);
            if let Some(pulse) = self.queue.pop() {
                if &pulse.destination == "rx" && pulse.signal == Signal::Low {
                    self.machine_on = true;
                }
                if let Some(module) = self.graph.get_mut(&pulse.destination) {
                    match module {
                        Module::FlipFlop(module) => module.process_pulse(pulse).into_iter().for_each(|p| self.fire(p)),
                        Module::Conjunction(module) => {
                            module.process_pulse(pulse).into_iter().for_each(|p| self.fire(p))
                        },
                        Module::Broadcast(module) => module.process_pulse(pulse).into_iter().for_each(|p| self.fire(p)),
                    };
                }
            }
        }
    }
    impl ::std::fmt::Display for Execution {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "{}", (self.total.high * self.total.low))
        }
    }
}

mod parser {
    use std::collections::HashMap;

    use super::models::*;
    use aoc_common::ParseError;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, newline, space0, space1},
        combinator::map,
        multi::{separated_list0, separated_list1},
        sequence::tuple,
        IResult,
    };

    fn parse_destinations(input: &str) -> IResult<&str, Vec<ModuleName>> {
        map(
            tuple((
                space1::<&str, ::nom::error::Error<&str>>,
                tag("->"),
                space1,
                separated_list1(tuple((tag(","), space0)), alphanumeric1),
            )),
            |(_, _, _, names)| names.iter().map(|s| s.to_string()).collect(),
        )(input)
    }

    fn parse_broadcast(input: &str) -> IResult<&str, (ModuleName, Module)> {
        map(tuple((tag("broadcaster"), parse_destinations)), |(_, destinations)| {
            ("broadcaster".to_string(), Module::Broadcast(BroadcastModule { destinations }))
        })(input)
    }

    fn parse_flipflop(input: &str) -> IResult<&str, (ModuleName, Module)> {
        map(tuple((tag("%"), alphanumeric1, parse_destinations)), |(_, name, destinations)| {
            (name.to_string(), Module::FlipFlop(FlipFlopModule { on: false, destinations }))
        })(input)
    }

    fn parse_conjunction(input: &str) -> IResult<&str, (ModuleName, Module)> {
        map(tuple((tag("&"), alphanumeric1, parse_destinations)), |(_, name, destinations)| {
            (name.to_string(), Module::Conjunction(ConjunctionModule { state: HashMap::default(), destinations }))
        })(input)
    }

    fn parse_module(input: &str) -> IResult<&str, (ModuleName, Module)> {
        alt((parse_broadcast, parse_flipflop, parse_conjunction))(input)
    }

    fn parse_lines(input: &str) -> IResult<&str, Vec<(ModuleName, Module)>> {
        separated_list0(newline, parse_module)(input)
    }

    pub(crate) fn get_modules(input: &str) -> Result<Graph, ParseError> {
        let (_, modules) = parse_lines(input.trim()).map_err(|err| ParseError::Nom(err.to_string()))?;
        let modules: Graph = modules.into_iter().collect();
        Ok(modules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_example() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let solution = Day00 {};
        let graph = solution.parse(input).unwrap();
        let exec = solution.part1(&graph).unwrap();
        assert_eq!("32000000".to_string(), exec.to_string());
    }

    #[test]
    fn test_second_example() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let solution = Day00 {};
        let graph = solution.parse(input).unwrap();
        let exec = solution.part1(&graph).unwrap();
        assert_eq!("11687500".to_string(), exec.to_string())
    }
}
