use parol::{build::Builder, ParolErrorReporter};
use parol_runtime::Report;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

fn main() {
    // Skip in GitHub Actions
    if let Ok(x) = env::var("GITHUB_ACTIONS") {
        if x == "true" {
            return;
        }
    }

    let par_file = PathBuf::from("veryla.par");
    let exp_file = PathBuf::from("src/generated/veryla-exp.par");

    let par_modified = fs::metadata(par_file).unwrap().modified().unwrap();
    let exp_modified = fs::metadata(exp_file).unwrap().modified().unwrap();

    if par_modified > exp_modified {
        println!("cargo:warning=veryla.par was changed");

        let now = Instant::now();

        // CLI equivalent is:
        // parol -f ./veryla.par -e ./veryla-exp.par -p ./src/veryla_parser -a ./src/veryla_grammar_trait -t VerylGrammar -m veryla_grammar -g
        if let Err(err) = Builder::with_explicit_output_dir("src/generated")
            .grammar_file("veryla.par")
            .expanded_grammar_output_file("veryla-exp.par")
            .parser_output_file("veryla_parser.rs")
            .actions_output_file("veryla_grammar_trait.rs")
            .enable_auto_generation()
            .user_type_name("VerylaGrammar")
            .user_trait_module_name("veryla_grammar")
            .trim_parse_tree()
            .generate_parser()
        {
            {
                ParolErrorReporter::report_error(&err, "veryla.par").unwrap_or_default();
                process::exit(1);
            }
        }

        let elapsed_time = now.elapsed();
        println!(
            "cargo:warning=parol build time: {} milliseconds",
            elapsed_time.as_millis()
        );
    }
}
