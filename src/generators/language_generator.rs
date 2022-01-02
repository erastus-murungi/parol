use crate::{Cfg, Symbol, Terminal};
use log::trace;
use miette::IntoDiagnostic;
use miette::{miette, Diagnostic, Result};
use rand::Rng;
use std::collections::HashMap;

const MAX_RESULT_SIZE: usize = 100000;
const MAX_REPEAT: u32 = 8;

#[derive(Error, Diagnostic, Debug)]
#[error("Stopping generation to prevent endless recursion at size {len}")]
#[diagnostic(
    help("Generation does not terminate in good time"),
    code("parol::generators::language_generator::source_size_exceeded")
)]
pub struct SourceSizeExceeded {
    len: usize,
}

#[derive(Debug)]
pub struct LanguageGenerator<'a> {
    generator_stack: Vec<Symbol>,
    cfg: &'a Cfg,
    cache: HashMap<String, rand_regex::Regex>,
}

impl<'a> LanguageGenerator<'a> {
    pub fn new(cfg: &'a Cfg) -> Self {
        Self {
            generator_stack: Vec::new(),
            cfg,
            cache: HashMap::new(),
        }
    }

    pub fn generate(&mut self, max_result_length: Option<usize>) -> Result<String> {
        let mut result = String::new();
        self.process_non_terminal(self.cfg.get_start_symbol())?;
        while let Some(symbol) = self.generator_stack.pop() {
            match symbol {
                Symbol::N(n) => self.process_non_terminal(&n),
                Symbol::T(Terminal::Trm(t, _)) => {
                    self.process_terminal(t.clone(), &mut result, max_result_length)
                }
                _ => Ok(()),
            }?
        }
        Ok(result)
    }

    fn process_non_terminal(&mut self, non_terminal: &str) -> Result<()> {
        let productions_of_nt = self.cfg.matching_productions(non_terminal);
        let chosen_index = rand::thread_rng().gen_range(0..productions_of_nt.len());
        trace!(
            "/* {} */ {} {}/{}",
            productions_of_nt[chosen_index].0,
            productions_of_nt[chosen_index].1,
            chosen_index + 1,
            productions_of_nt.len()
        );
        productions_of_nt[chosen_index]
            .1
            .get_r()
            .iter()
            .rev()
            .for_each(|s| self.generator_stack.push(s.clone()));
        Ok(())
    }

    fn process_terminal(
        &mut self,
        terminal: String,
        result: &mut String,
        max_result_length: Option<usize>,
    ) -> Result<()> {
        let mut rng = rand::thread_rng();
        let utf8_gen = self.get_regex(terminal)?;
        let generated = rng.sample::<String, _>(&utf8_gen);
        trace!("gen: {}", generated);
        result.push_str(&generated);
        result.push(' ');
        let len = result.len();
        if len > max_result_length.unwrap_or(MAX_RESULT_SIZE) {
            Err(miette!(SourceSizeExceeded { len }))
        } else {
            Ok(())
        }
    }

    fn get_regex<'b, 'c>(&'b mut self, terminal: String) -> Result<&'c rand_regex::Regex>
    where
        'b: 'c,
    {
        let exist = self.cache.get(&terminal).is_some();

        if exist {
            let regex = self.cache.get(&terminal).unwrap();
            trace!("Reusing cached regex for: {}", terminal);
            return Ok(regex);
        }

        match regex_syntax::ParserBuilder::new().build().parse(&terminal) {
            Ok(utf8_hir) => {
                let utf8_gen =
                    rand_regex::Regex::with_hir(utf8_hir, MAX_REPEAT).into_diagnostic()?;
                trace!("Caching regex for: {}", terminal);
                self.cache.insert(terminal.clone(), utf8_gen);
                self.get_regex(terminal)
            }
            Err(err) => Err(miette!(err)),
        }
    }
}
