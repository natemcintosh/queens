use criterion::{Criterion, black_box, criterion_group, criterion_main};
use queens::{QueenBoard, solve};

fn place_queen_benchmark(c: &mut Criterion) {
    c.bench_function("place_queen_empty", |b| {
        b.iter(|| {
            let board = QueenBoard::new(12, 12);
            let empty_board = QueenBoard::new(12, 12);
            board.place_queen(black_box(0), black_box(&empty_board));
        })
    });

    c.bench_function("place_queen_with_color_region", |b| {
        b.iter(|| {
            let board = QueenBoard::new(12, 12);
            let queen_idx: usize = 2;
            let queen_mask = 1usize.rotate_left(queen_idx as u32);
            let color_region_inds: Vec<usize> = vec![queen_idx, 10, 20];
            let mut cr_board = QueenBoard::new(12, 12);
            for idx in color_region_inds {
                cr_board.set_linear_index(idx, true);
            }
            board.place_queen(black_box(queen_mask), black_box(&cr_board));
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
