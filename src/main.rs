use clap::Parser;
use queens::solve;

#[derive(Parser)]
#[command(about = "Solve the queens problem")]
struct Args {
    /// Input string for the queens problem
    #[clap(short, long)]
    input_str: String,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Solve the queens problem
    solve(&args.input_str);
}
