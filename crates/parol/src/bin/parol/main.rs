extern crate clap;

mod arguments;
mod tools;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::{anyhow, Context, Result};
use arguments::CliArgs;
use clap::Parser;
use miette::IntoDiagnostic;
use parol_runtime::log::trace;

use id_tree::Tree;
use parol::{
    build::{BuildListener, IntermediateGrammar},
    render_par_string, to_report, GrammarConfig, ParolGrammar,
};
use parol_runtime::parser::ParseTreeType;

// To rebuild the parser sources from scratch use the command build_parsers.ps1

fn main() -> miette::Result<()> {
    env_logger::try_init().into_diagnostic()?;
    trace!("env logger started");

    let args = CliArgs::parse();

    if let Some(subcommand) = args.subcommand {
        return subcommand.invoke_main().map_err(to_report);
    }

    // If relative paths are specified, they should be resoled relative to the current directory
    let mut builder =
        parol::build::Builder::with_explicit_output_dir(env::current_dir().into_diagnostic()?);

    // It's okay if the output doesn't exist;
    builder.disable_output_sanity_checks();
    // Don't care about cargo.
    builder.set_cargo_integration(false);

    // NOTE: Grammar file is required option
    let grammar_file = args
        .grammar
        .as_ref()
        .ok_or_else(|| anyhow!("Missing input grammar file (Specify with `-f`)"))
        .map_err(to_report)?;
    builder.grammar_file(grammar_file);

    builder.max_lookahead(args.lookahead).into_diagnostic()?;
    if let Some(module) = &args.module {
        builder.user_trait_module_name(module);
    }
    if let Some(user_type) = &args.user_type {
        builder.user_type_name(user_type);
    }
    if let Some(actions_file) = &args.actions {
        builder.actions_output_file(actions_file);
    }
    if let Some(parser_file) = &args.parser {
        builder.parser_output_file(parser_file);
    }
    if args.auto_generate {
        builder.enable_auto_generation();
    }
    if args.range {
        builder.range();
    }
    builder.inner_attributes(args.inner_attributes.clone());
    if let Some(expanded_grammar_file) = &args.expanded {
        if expanded_grammar_file == OsStr::new("--") {
            // We special case this in our listener (see below)
        } else {
            builder.expanded_grammar_output_file(expanded_grammar_file);
        }
    }

    let mut listener = CLIListener {
        grammar_file,
        config: &args,
    };
    let mut generator = builder
        .begin_generation_with(Some(&mut listener))
        .into_diagnostic()?;

    generator.parse().map_err(to_report)?;
    generator.expand().map_err(to_report)?;
    generator.post_process().map_err(to_report)?;
    generator.write_output().map_err(to_report)?;

    Ok(())
}

pub struct CLIListener<'a> {
    config: &'a CliArgs,
    grammar_file: &'a Path,
}
impl CLIListener<'_> {
    fn verbose(&self) -> bool {
        self.config.verbose
    }
}
impl BuildListener for CLIListener<'_> {
    fn on_initial_grammar_parse(
        &mut self,
        syntax_tree: &Tree<ParseTreeType>,
        parol_grammar: &ParolGrammar,
    ) -> Result<()> {
        if self.verbose() {
            println!("{}", parol_grammar);
        }

        if let Some(file_name) = self.config.write_internal.as_ref() {
            let serialized = format!("{}", parol_grammar);
            fs::write(file_name, serialized).context("Error writing left-factored grammar!")?;
        }

        if self.config.generate_tree_graph {
            parol::generate_tree_layout(syntax_tree, self.grammar_file)
                .context("Error generating tree layout")?;
        }

        Ok(())
    }

    fn on_intermediate_grammar(
        &mut self,
        stage: IntermediateGrammar,
        grammar_config: &GrammarConfig,
    ) -> Result<()> {
        match stage {
            // no passes yet
            IntermediateGrammar::Untransformed => {
                if let Some(file_name) = self.config.write_untransformed.as_ref() {
                    let serialized = render_par_string(grammar_config, false)?;
                    fs::write(file_name, serialized)
                        .context("Error writing untransformed grammar!")?;
                }
            }
            // final pass
            IntermediateGrammar::LAST => {
                if let Some(file_name) = self.config.expanded.as_ref() {
                    // NOTE: We still need special handling for writing to stdout
                    let lf_source = render_par_string(grammar_config, true)?;
                    if *file_name == OsStr::new("--") {
                        print!("{}", lf_source);
                    } else {
                        // Should be handled by the builder
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
