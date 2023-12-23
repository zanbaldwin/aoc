use crate::{error::Error, parse, reindeer_hash};
use std::{collections::BTreeMap, fmt::Display};

pub type Label<'a> = &'a str;
pub type FocalLength = u8;

///
pub enum Instruction {
    Insert(FocalLength),
    Remove,
}

/// A single step in the initialization sequence. It consists of a label, and an
/// [`Instruction`] telling you what to do with it.
///
/// [`Instruction`]: crate::models::Instruction
pub struct Step<'a> {
    pub label: Label<'a>,
    pub instruction: Instruction,
}
impl<'a> Step<'a> {
    /// Get Lens to insert. If the step in the initialization sequence instructs
    /// you to insert a [`Lens`], then this function will return said lens.
    /// Otherwise it returns nothing and you should remove anything already
    /// existing in the lens slot specified by the step.
    ///
    /// [`Lens`]: crate::models::Lens
    pub fn get_lens(&self, sort_order: usize) -> Option<Lens<'a>> {
        match self.instruction {
            Instruction::Insert(focal_length) => Some(Lens {
                label: self.label,
                focal_length,
                sort_order,
            }),
            Instruction::Remove => None,
        }
    }
}

/// A representation of the lenses that the steps in the initialization sequence
/// reference (by label and focal point).
#[derive(Debug, PartialEq, Eq)]
pub struct Lens<'a> {
    pub label: Label<'a>,
    pub focal_length: FocalLength,
    pub sort_order: usize,
}
impl<'a> PartialOrd for Lens<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for Lens<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_order.cmp(&other.sort_order)
    }
}
impl<'a> Display for Lens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

/// A sorted map of lenses, by label.
pub type LensBox<'a> = BTreeMap<&'a str, Lens<'a>>;
/// A basic struct containing the map of lens boxes, just so I can implement
/// traits on it since I don't own [`BTreeMap`].
///
/// [`BTreeMap`]: std::collections::BTreeMap
pub struct Boxes<'a> {
    boxes: BTreeMap<u8, LensBox<'a>>,
}
impl<'a> TryFrom<&'a str> for Boxes<'a> {
    type Error = Error;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut boxes: BTreeMap<u8, LensBox<'a>> = BTreeMap::new();
        input.split(',').enumerate().try_for_each(|(i, step)| -> Result<(), Error> {
            let step = parse(step)?;
            let hash = reindeer_hash(step.label);
            let lens_box = boxes.entry(hash as u8).or_default();
            // We care about the order that they are being *inserted* but
            // order doesn't matter if they are being *replaced*.
            match step.get_lens(i) {
                Some(lens) => {
                    match lens_box.get_mut(step.label) {
                        Some(existing_lens) => {
                            existing_lens.focal_length = lens.focal_length;
                        },
                        None => {
                            lens_box.insert(step.label, lens);
                        },
                    };
                },
                None => {
                    lens_box.remove(&step.label);
                },
            }
            Ok(())
        })?;
        Ok(Boxes { boxes })
    }
}
impl<'a> Display for Boxes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for i in 0..=u8::MAX {
            if let Some(slots) = self.boxes.get(&i) {
                if slots.is_empty() {
                    continue;
                }
                result.push_str(format!("Box {i}:").as_str());
                let mut lenses: Vec<&Lens> = slots.values().collect();
                lenses.sort();
                for lens in lenses {
                    result.push_str(format!(" {lens}").as_str());
                }
                result.push('\n');
            }
        }
        write!(f, "{result}")
    }
}
impl<'a> Boxes<'a> {
    /// Get the focusing power of each lens (by label) determined by the lens'
    /// position within each box, and the position of its box.
    pub fn get_focusing_powers(&self) -> BTreeMap<&str, u32> {
        let mut focusing_powers: BTreeMap<&'a str, u32> = BTreeMap::new();

        (0..=u8::MAX)
            .filter_map(|box_number| Some((box_number, self.boxes.get(&box_number)?.values().collect::<Vec<_>>())))
            .for_each(|(box_number, mut lenses)| {
                lenses.sort();
                lenses.into_iter().enumerate().for_each(|(lens_position, lens)| {
                    focusing_powers.insert(
                        lens.label,
                        (box_number as u32 + 1) * (lens_position as u32 + 1) * lens.focal_length as u32,
                    );
                })
            });
        focusing_powers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    const EXPECTED_BOXES: &str = "
Box 0: [rn 1] [cm 2]
Box 3: [ot 7] [ab 5] [pc 6]";

    #[test]
    fn test_parse_boxes() {
        let boxes: Boxes = TEST_INPUT.try_into().unwrap();
        let ascii_graph = boxes.to_string();
        println!("{ascii_graph}");
        assert_eq!(EXPECTED_BOXES.trim(), ascii_graph.trim());
    }
}
