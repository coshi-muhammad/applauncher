use criterion::{Bencher, Criterion, criterion_group, criterion_main};
//HACK: do this untill you seperate your app into diffrent files and modules

/*NOTE:
* this is an example benchmark for how to use Criterion for external benshmarking
* when you run `cargo bench` the benchamrks you have defined will be ran
* and an html containing graphs describing the preformance will be generated in
* /targets/report/index.html
* */

fn benchmark_slop(n: i32) {
    let mut sum = 0;
    for i in 1..n {
        sum += i;
    }
}
fn bench_slop(c: &mut Criterion) {
    c.bench_function("something to bench", |b: &mut Bencher| {
        b.iter(|| benchmark_slop(100));
    });
}

criterion_group!(benches, bench_slop);
criterion_main!(benches);
