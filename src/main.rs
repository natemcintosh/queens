use clap::Parser;
use queens::solve;

#[derive(Parser)]
#[command(about = "Solve the queens problem")]
struct Args {
    /// Input string for the queens problem. Should be all in one string, representing
    /// the colors. You assign unique numbers to each color region (use the numbers 1 -
    /// 8 or it will error out). Separate each row with a space. An example input is
    /// "11233456 12234456 11233456 12273456 11233456 88885556 66888886 66666666"
    /// where number corresponds to a certain color, e.g. 1 => red, 2 => blue, etc.
    color_regions: String,

    /// Maximum number of iterations to try before giving up
    #[clap(short, long)]
    #[arg(default_value_t = 100_000)]
    max_iters: usize,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Solve the queens problem
    solve(&args.color_regions, args.max_iters);
}
