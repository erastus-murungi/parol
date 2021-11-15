use crate::lexer::TerminalIndex;
use crate::parser::{NonTerminalIndex, ProductionIndex, ScannerIndex};
use std::fmt::{Display, Error, Formatter};

///
/// The type of the elements in the parser stack.
///
#[derive(Debug, Clone)]
pub enum ParseType {
    ///
    /// The index of a non-terminal in the generated NON_TERMINALS array
    ///
    N(NonTerminalIndex),

    ///
    /// The index of a terminal in the generated TERMINALS array
    ///
    T(TerminalIndex),

    ///
    /// The index of a scanner configuration
    ///
    S(ScannerIndex),

    ///
    /// End of production marker
    ///
    E(ProductionIndex),
}

impl Display for ParseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::N(n) => write!(f, "N({})", n),
            Self::T(t) => write!(f, "T({})", t),
            Self::S(s) => write!(f, "S({})", s),
            Self::E(e) => write!(f, "E({})", e),
        }
    }
}

///
/// The generated parsers are push down automata (PDA) which utilize a stack
/// during parsing. It helps to process the grammar's productions.
///
pub struct ParseStack {
    ///
    /// The actual stack.
    ///
    pub stack: Vec<ParseType>,
    terminal_names: &'static [&'static str],
    non_terminal_names: &'static [&'static str],
}

impl ParseStack {
    ///
    /// Creates a new instance with the given parameters.
    ///
    pub fn new(
        terminal_names: &'static [&'static str],
        non_terminal_names: &'static [&'static str],
    ) -> Self {
        Self {
            stack: Vec::new(),
            terminal_names,
            non_terminal_names,
        }
    }

    fn decode_terminal(&self, terminal_index: usize) -> &'static str {
        self.terminal_names[terminal_index]
    }

    fn decode_non_terminal(&self, non_terminal_index: usize) -> &'static str {
        self.non_terminal_names[non_terminal_index]
    }
}

impl Display for ParseStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.stack
            .iter()
            .rev()
            .enumerate()
            .fold(Ok(()), |acc, (i, e)| match acc {
                Ok(()) => match e {
                    ParseType::T(t) => writeln!(f, "{} - T({})", i, self.decode_terminal(*t)),
                    ParseType::N(n) => writeln!(f, "{} - N({})", i, self.decode_non_terminal(*n)),
                    _ => writeln!(f, "{} - {}", i, e),
                },
                Err(_) => acc,
            })
    }
}
