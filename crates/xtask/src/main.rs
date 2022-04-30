use anyhow::Context;
use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Sets the verbosity level
    #[clap(short, parse(from_occurrences))]
    verbosity: usize,
    /// The task to run
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Builds the book
    Build,
    /// Releases the book to zyghost
    Release,
}

fn main() -> anyhow::Result<()> {
    // let's measure how long it takes to do anything
    let start = std::time::Instant::now();

    let cli = Cli::parse();

    let level = match cli.verbosity {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };
    // use the verbosity level later when we build TVM
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match cli.command {
        Command::Build => {
            duct::cmd!("mdbook", "build")
                .run()
                .context("could not build book")?;
        }
        Command::Release => {
            duct::cmd!(
                "aws",
                "s3",
                "sync",
                "book",
                "s3://zyghost.com/guides/beginners-guide-to-message-passing",
                "--acl",
                "public-read"
            )
            .run()
            .context("could not release")?;
        }
    }

    let elapsed = start.elapsed().as_secs_f32();
    tracing::info!("finished in {:.2}s", elapsed);

    Ok(())
}
