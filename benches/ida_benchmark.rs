
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rabin_ida::RabinIDA;

pub fn ida_benchmark(c: &mut Criterion) {
     let data = vec![3u8; 1024];
let n = 7;
 let k = 5;
 let sharer = RabinIDA::new(n, k);

 let shares = sharer.share(data.clone());

//  let rec = sharer.reconstruct(shares[1..=k as usize].to_vec()).unwrap();
    c.bench_function("ida 1mb 5of7 create", |b| b.iter(|| sharer.share(black_box(data.clone()))));
    c.bench_function("ida 1mb 5of7 reconstruct", |b| b.iter(|| sharer.reconstruct(black_box(shares[1..=k as usize].to_vec())).unwrap()));
}

criterion_group!(benches, ida_benchmark);
criterion_main!(benches);

