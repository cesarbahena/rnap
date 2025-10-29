use clap::Parser;

#[derive(Parser)]
#[command(name = "rnap")]
#[command(about = "Requirements Normalization and Assessment Platform")]
pub enum Cli {
    Create {
        #[arg(help = "Gene name in TYPE-slug format (e.g., FEAT-user-auth)")]
        name: String,
        #[arg(
            short,
            long,
            help = "Trait definitions (e.g., title:dominant,description:recessive)"
        )]
        traits: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_create_command() {
        let cli = Cli::try_parse_from([
            "rnap",
            "create",
            "FEAT-user-auth",
            "--traits",
            "title:dominant",
        ]);

        assert!(cli.is_ok());
        if let Cli::Create { name, traits } = cli.unwrap() {
            assert_eq!(name, "FEAT-user-auth");
            assert_eq!(traits, "title:dominant");
        } else {
            panic!("Expected Create variant");
        }
    }
}
