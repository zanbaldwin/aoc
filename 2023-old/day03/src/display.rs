use crate::{EngineMap, PartNumber, Symbol};
use std::fmt;

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}', ({},{})", self.symbol, self.coord.x, self.coord.y)
    }
}

impl fmt::Display for PartNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "#{} ({}-{},{})",
            self.id,
            self.coord.x,
            self.coord.x.saturating_add(self.length).saturating_sub(1),
            self.coord.y
        )
    }
}

impl fmt::Display for EngineMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut lines: Vec<String> = vec![];
        let max_width = ::std::cmp::max(
            self.parts.len().to_string().len(),
            self.symbols.len().to_string().len(),
        );

        lines.push("List of Machine Parts\n=====================".to_string());
        for (index, part) in self.parts.iter().enumerate() {
            lines.push(format!("{:width$}: {}", index + 1, part, width = max_width));
        }

        lines.push("\nList of Symbols\n===============".to_string());
        for (index, symbol) in self.symbols.iter().enumerate() {
            lines.push(format!(
                "{:width$}: {}",
                index + 1,
                symbol,
                width = max_width
            ));
        }

        write!(f, "{}", lines.join("\n"))
    }
}
