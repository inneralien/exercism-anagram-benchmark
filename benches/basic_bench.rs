use anagram::anagrams_for;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let word = "allergy";
    let inputs = &[
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];
    c.bench_function("Anagram 6 words", |b| {
        b.iter(|| anagrams_for(black_box(word), black_box(inputs)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
