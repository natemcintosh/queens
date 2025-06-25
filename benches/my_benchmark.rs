use criterion::{Criterion, black_box, criterion_group, criterion_main};
use queens::Board;

fn place_queen_benchmark(c: &mut Criterion) {
    c.bench_function("place_queen_empty", |b| {
        b.iter(|| {
            let board = Board::new();
            board.place_queen(black_box(0), black_box(0));
        })
    });
}

criterion_group!(benches, place_queen_benchmark);
criterion_main!(benches);
