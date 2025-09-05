use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::collections::HashMap;

fn generate_keys() -> Vec<String> {
    let mut keys = Vec::new();
    for i in 1..9 {
        for j in 1..9 {
            for k in 1..9 {
                for l in 1..9 {
                    keys.push(format!("{}{}{}{}", i, j, k, l));
                }
            }
        }
    }
    keys
}

fn trie_benchmark(c: &mut Criterion) {
    c.bench_function("trie_match", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        t.insert(black_box("test".bytes()), black_box(String::from("test")));
        b.iter(|| {
            assert!(t.contains_key(black_box("test".bytes())));
        });
    });

    c.bench_function("trie_mismatch", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        t.insert(black_box("test".bytes()), black_box(String::from("test")));
        b.iter(|| {
            assert!(!t.contains_key(black_box("tst".bytes())));
        });
    });

    c.bench_function("trie_massive_match", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for key in &keys {
                assert!(t.contains_key(black_box(key.bytes())));
            }
        });
    });

    c.bench_function("trie_massive_mismatch_on_0", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let mismatching = String::from("0999");
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for _ in 0..keys.len() {
                assert!(!t.contains_key(black_box(mismatching.bytes())));
            }
        });
    });

    c.bench_function("trie_massive_mismatch_on_1", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let mismatching = String::from("9099");
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for _ in 0..keys.len() {
                assert!(!t.contains_key(mismatching.bytes()));
            }
        });
    });

    c.bench_function("trie_massive_mismatch_on_2", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let mismatching = String::from("9909");
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for _ in 0..keys.len() {
                assert!(!t.contains_key(black_box(mismatching.bytes())));
            }
        });
    });

    c.bench_function("trie_massive_mismatch_on_3", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let mismatching = String::from("9990");
        let keys = generate_keys();
        for key in &keys {
            t.insert(key.bytes(), key.clone());
        }
        b.iter(|| {
            for _ in 0..keys.len() {
                assert!(!t.contains_key(black_box(mismatching.bytes())));
            }
        });
    });

    c.bench_function("trie_prefixes_match", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for key in &keys {
                assert!(!t.find_prefixes(black_box(key.bytes())).is_empty());
            }
        });
    });

    c.bench_function("trie_postfixes_match", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for key in &keys {
                assert!(!t.find_postfixes(black_box(key.bytes())).is_empty());
            }
        });
    });

    c.bench_function("trie_prefix_longest_match", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for key in &keys {
                assert!(t.find_longest_prefix(black_box(key.bytes())).is_some());
            }
        });
    });

    c.bench_function("trie_massive_prefixes_match", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for key in &keys {
                assert!(!t.find_prefixes(black_box(key.bytes())).is_empty());
            }
        });
    });

    c.bench_function("trie_massive_longest_prefixes_match", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for key in &keys {
                assert!(t.find_longest_prefix(black_box(key.bytes())).is_some());
            }
        });
    });

    c.bench_function("trie_massive_postfixes_match", |b| {
        let mut t = oramacore_ptrie::Trie::new();
        let keys = generate_keys();
        for key in &keys {
            t.insert(black_box(key.bytes()), black_box(key.clone()));
        }
        b.iter(|| {
            for key in &keys {
                assert!(!t.find_postfixes(black_box(key.bytes())).is_empty());
            }
        });
    });
}

fn hashmap_benchmark(c: &mut Criterion) {
    c.bench_function("hashmap_match", |b| {
        let mut h = HashMap::new();
        let key = String::from("test");
        h.insert(key.clone(), true);
        b.iter(|| {
            h.get(&key);
        });
    });

    c.bench_function("hashmap_mismatch", |b| {
        let mut h = HashMap::new();
        let key = String::from("test");
        let notkey = String::from("tst");
        h.insert(key, true);
        b.iter(|| {
            h.get(&notkey);
        });
    });

    c.bench_function("hashmap_massive_match", |b| {
        let mut h = HashMap::new();
        let keys = generate_keys();
        for key in &keys {
            h.insert(key.clone(), key.clone());
        }
        b.iter(|| {
            for key in &keys {
                assert!(h.contains_key(key));
            }
        });
    });

    c.bench_function("hashmap_massive_mismatch_on_0", |b| {
        let mut h = HashMap::new();
        let mismatching = String::from("0999");
        let keys = generate_keys();
        for key in &keys {
            h.insert(key.clone(), key.clone());
        }
        b.iter(|| {
            for _ in 0..keys.len() {
                assert!(!h.contains_key(&mismatching));
            }
        });
    });

    c.bench_function("hashmap_massive_mismatch_on_0_one_symbol_key", |b| {
        let mut h = HashMap::new();
        let mismatching = String::from("0");
        let keys = generate_keys();
        for key in &keys {
            h.insert(key.clone(), key.clone());
        }
        b.iter(|| {
            for _ in 0..keys.len() {
                assert!(!h.contains_key(&mismatching));
            }
        });
    });
}

criterion_group!(benches, trie_benchmark, hashmap_benchmark,);
criterion_main!(benches);
