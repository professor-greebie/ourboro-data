use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
        /// Optional Location of PCCF file, otherwise will default to the current directory
        pub pccf: Option<String>,
        #[clap(long)]
        pub ourboro: Option<bool>,
        #[clap(long)]
        pub income: Option<bool>,
        #[clap(long)]
        pub sample: Option<bool>,
        /// Collect population data from census
        #[clap(long)]
        pub population: Option<bool>,
        /// Collect land area data from census
        #[clap(long)]
        pub land_area: Option<bool>,
        /// Collect total occupied dwellings data from census
        #[clap(long)]
        pub total_occupied_dwellings: Option<bool>,
        /// Collect total single detached houses data from census
        #[clap(long)]
        pub total_single_detached_houses: Option<bool>,
        #[clap(long)]
        pub take: Option<usize>,
        #[clap(long)]
        pub skip: Option<usize>,
        /// Show verbose output
        #[clap(short, long)]
        pub verbose: Option<bool>,
        
        #[clap(short, long)]
        /// Optional location of the output file, otherwise will default to the current directory
        pub output: Option<String>,
        
        #[clap(short, long)]
        /// Convert the xlsx file in the located path to a csv file
        pub xlsx: Option<String>,
        #[clap(long)]
        pub postal: Option<String>,
        #[clap(short, long)]
        pub province: Option<String>,
}
