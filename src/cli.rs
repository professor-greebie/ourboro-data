use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
        /// Optional Location of PCCF file, otherwise will default to the current directory
        pub pccf: Option<String>,
        #[clap(long)]
        pub income: Option<bool>,
        #[clap(long)]
        pub sample: Option<bool>,
        #[clap(long)]
        pub population: Option<bool>,
        #[clap(long)]
        pub take: Option<usize>,
        #[clap(long)]
        pub skip: Option<usize>,
        /// Show verbose output
        #[clap(short, long)]
        pub verbose: Option<bool>,
        /// Optional location of the output file, otherwise will default to the current directory
        #[clap(short, long)]
        pub output: Option<String>,
        #[clap(long)]
        pub postal: Option<String>,
        #[clap(short, long)]
        pub province: Option<String>,
}
