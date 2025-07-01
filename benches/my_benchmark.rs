use criterion::{Criterion, black_box, criterion_group, criterion_main};
use queens::{Board, solve};

fn place_queen_benchmark(c: &mut Criterion) {
    c.bench_function("place_queen_empty", |b| {
        b.iter(|| {
            let board = Board::new();
            board.place_queen(black_box(0), black_box(0));
        })
    });

    c.bench_function("place_queen_with_color_region", |b| {
        b.iter(|| {
            let board = Board::new();
            let queen_idx = 1u64.rotate_left(2);
            let color_region_mask = queen_idx | 1u64.rotate_left(10) | 1u64.rotate_left(20);
            board.place_queen(black_box(queen_idx), black_box(color_region_mask));
        })
    });
}

fn solve_benchmark(c: &mut Criterion) {
    c.bench_function("solve_actual_board", |b| {
        let raw_color_regions =
            "11112333 11222344 11255346 77253344 73355334 77335344 87355333 77333333";
        b.iter(|| solve(black_box(raw_color_regions), black_box(false)))
    });

    c.bench_function("solve_regions_are_columns", |b| {
        let raw_color_regions =
            "12345678 12345678 12345678 12345678 12345678 12345678 12345678 12345678";
        b.iter(|| solve(black_box(raw_color_regions), black_box(false)))
    });

    c.bench_function("solve_regions_are_rows", |b| {
        let raw_color_regions =
            "11111111 22222222 33333333 44444444 55555555 66666666 77777777 88888888";
        b.iter(|| solve(black_box(raw_color_regions), black_box(false)))
    });
}

criterion_group!(benches, place_queen_benchmark, solve_benchmark);
criterion_main!(benches);
