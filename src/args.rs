use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct AsmArgs {
    /// please provide the path to the graph file
    pub graph: String,
}
