use criterion::{Criterion, black_box, criterion_group, criterion_main};
use queens::Board;

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

criterion_group!(benches, place_queen_benchmark);
criterion_main!(benches);
