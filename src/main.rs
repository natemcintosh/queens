use clap::Parser;
use queens::{build_bit_set_from_inds, disp_u64, format_thousands, solve};

#[derive(Parser)]
#[command(about = "Solve the queens problem")]
struct Args {
    /// Input string for the queens problem. Should be all in one string, representing
    /// the colors. You assign unique letters to each color region. Separate each row
    /// with a space. An example input is
    /// "aabccefg abbceefg aabccefg abbgcdef aabccedf hhhheeef ffhhhhhf ffffffff"
    /// where letter corresponds to a certain color, e.g. r => red, b => blue, etc.
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
    let (res, n_iters) = solve(&args.color_regions, args.verbose);
    let run_time = start.elapsed();
    let formatted_iters = format_thousands(n_iters);

    // Print out the time it took
    println!("Positions searched: {formatted_iters}");
    println!("Time: {:?}\n\n", run_time);

    // Print out the result, whatever it is
    disp_u64(build_bit_set_from_inds(&res.expect("No solution found")));
}
