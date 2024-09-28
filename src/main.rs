mod args;
mod graph;
mod metadata;
mod mermaid_graph;

use anyhow::Error;
use structopt::StructOpt;
use crate::args::Opts;
use crate::mermaid_graph::generate_mermaid_graph;

fn main() -> Result<(), Error> {

    let Opts::Tree(args) = Opts::from_args();
    let metadata = metadata::get(&args)?;
    let graph = graph::build(&args, metadata)?;
    let mermaid = generate_mermaid_graph(&args, &graph)?;
    Ok(())

}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
    use predicates::prelude::predicate;

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("cargo-tree-mermaid")?;

        cmd.arg("--version");

        cmd.assert().success()
            .stdout(predicate::str::contains("--->"));

        Ok(())
    }
}