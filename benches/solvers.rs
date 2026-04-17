use criterion::{Criterion, black_box, criterion_group, criterion_main};
use queens::solve_brute_force;

const ACTUAL_BOARD: &str =
    "11112333 11222344 11255346 77253344 73355334 77335344 87355333 77333333";
const COLUMNS_8X8: &str = "12345678 12345678 12345678 12345678 12345678 12345678 12345678 12345678";
const ROWS_8X8: &str = "11111111 22222222 33333333 44444444 55555555 66666666 77777777 88888888";

fn bench_solvers(c: &mut Criterion) {
    for (name, input) in [
        ("actual_board", ACTUAL_BOARD),
        ("columns_8x8", COLUMNS_8X8),
        ("rows_8x8", ROWS_8X8),
    ] {
        let mut group = c.benchmark_group(format!("solve/{name}"));
        group.bench_function("brute_force", |b| {
            b.iter(|| solve_brute_force(black_box(input), black_box(false)));
        });
        // When solve_backtracking is implemented, add:
        // group.bench_function("backtracking", |b| {
        //     b.iter(|| solve_backtracking(black_box(input), black_box(false)));
        // });
        group.finish();
    }
}

criterion_group!(benches, bench_solvers);
criterion_main!(benches);
