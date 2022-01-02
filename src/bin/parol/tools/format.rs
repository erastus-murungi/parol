use miette::Result;
use parol::conversions::par::render_par_string;
use parol::obtain_grammar_config;

pub fn main(args: &[&str]) -> Result<()> {
    if args.len() > 1 {
        let file_name = args[1].to_owned();
        let grammar_config = obtain_grammar_config(&file_name, false)?;
        println!("{}", render_par_string(&grammar_config, true));
    } else {
        println!("Missing arguments <par-file>!");
        println!(
            "Example:\n\
            cargo run --bin parol format ./src/parser/parol-grammar-exp.par"
        );
    }
    Ok(())
}