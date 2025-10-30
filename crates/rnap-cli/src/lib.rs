use clap::Parser;

fn main() {
    let _cli = Cli::parse();
}

#[derive(Parser)]
#[command(name = "rnap")]
#[command(about = "Requirements Normalization and Assessment Platform")]
pub enum Cli {
    Create {
        #[arg(help = "Genotype kind (e.g., FEAT, BUG)")]
        kind: String,
        #[arg(help = "Gene name (e.g., user authentication)")]
        name: String,
    },
    Mutate {
        #[arg(help = "Gene name (e.g., FEAT-0001-user-auth)")]
        gene: String,
        #[arg(help = "Trait assignments (e.g., title=Hello) and context")]
        args: Vec<String>,
        #[arg(short = 'a', long, help = "Append to array trait")]
        append: bool,
        #[arg(short = 'r', long, help = "Replace collection trait")]
        replace: bool,
        #[arg(long, default_value = "human", help = "Author of the mutation")]
        by: String,
    },
    Transcribe {
        #[arg(help = "Gene name (e.g., FEAT-0001-user-auth)")]
        gene: String,
    },
}
