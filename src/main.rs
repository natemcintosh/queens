use clap::Parser;
use queens::{build_bit_set_from_inds, disp_u64, solve};

#[derive(Parser)]
#[command(about = "Solve the queens problem")]
struct Args {
    /// Input string for the queens problem. Should be all in one string, representing
    /// the colors. You assign unique numbers to each color region (use the numbers 1 -
    /// 8 or it will error out). Separate each row with a space. An example input is
    /// "11233456 12234456 11233456 12273456 11233456 88885556 66888886 66666666"
    /// where number corresponds to a certain color, e.g. 1 => red, 2 => blue, etc.
    color_regions: String,

    /// Whether to print out the solution in a human-readable format
    #[arg(short, long, default_value_t = true)]
    verbose: bool,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Time how long it takes
    let start = std::time::Instant::now();

    // Solve the queens problem
    let res = solve(&args.color_regions, args.verbose).expect("Could not find a solution");

    // Print out the time it took
    println!("Time: {:?}\n\n", start.elapsed());

    // Print out the result, whatever it is
    disp_u64(build_bit_set_from_inds(&res));
}
