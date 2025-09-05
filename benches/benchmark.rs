#![feature(test)]
extern crate test;

use std::collections::HashMap;
use test::Bencher;

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

#[bench]
fn trie_match(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();

    t.insert("test".bytes(), String::from("test"));

    b.iter(|| {
        assert!(t.contains_key("test".bytes()));
    })
}

#[bench]
fn trie_mismatch(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();

    t.insert("test".bytes(), String::from("test"));

    b.iter(|| {
        assert!(!t.contains_key("tst".bytes()));
    })
}

#[bench]
fn hash_map_match(b: &mut Bencher) {
    let mut h = HashMap::new();
    let key = String::from("test");

    h.insert(key.clone(), true);

    b.iter(|| {
        h.get(&key);
    })
}

#[bench]
fn hash_map_mismatch(b: &mut Bencher) {
    let mut h = HashMap::new();
    let key = String::from("test");
    let notkey = String::from("tst");

    h.insert(key, true);

    b.iter(|| {
        h.get(&notkey);
    })
}

#[bench]
fn trie_massive_match(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.bytes(), key.clone());
    }

    b.iter(|| {
        for key in &keys {
            assert!(t.contains_key(key.bytes()));
        }
    })
}

#[bench]
fn trie_massive_mismatch_on_0(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();
    let mismatching = String::from("0999");
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.bytes(), key.clone());
    }

    b.iter(|| {
        for _ in 0..keys.len() {
            assert!(!t.contains_key(mismatching.bytes()));
        }
    })
}

#[bench]
fn trie_massive_mismatch_on_1(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();
    let mismatching = String::from("9099");
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.bytes(), key.clone());
    }

    b.iter(|| {
        for _ in 0..keys.len() {
            assert!(!t.contains_key(mismatching.bytes()));
        }
    })
}

#[bench]
fn trie_massive_mismatch_on_2(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();
    let mismatching = String::from("9909");
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.bytes(), key.clone());
    }

    b.iter(|| {
        for _ in 0..keys.len() {
            assert!(!t.contains_key(mismatching.bytes()));
        }
    })
}

#[bench]
fn trie_massive_mismatch_on_3(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();
    let mismatching = String::from("9990");
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.bytes(), key.clone());
    }

    b.iter(|| {
        for _ in 0..keys.len() {
            assert!(!t.contains_key(mismatching.bytes()));
        }
    })
}

#[bench]
fn hash_map_massive_match(b: &mut Bencher) {
    let mut h = HashMap::new();
    let keys = generate_keys();

    for key in &keys {
        h.insert(key.clone(), key.clone());
    }

    b.iter(|| {
        for key in &keys {
            assert!(h.contains_key(key));
        }
    })
}

#[bench]
fn hash_map_massive_mismatch_on_0(b: &mut Bencher) {
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
    })
}

#[bench]
fn hash_map_massive_mismatch_on_0_one_symbol_key(b: &mut Bencher) {
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
    })
}

#[bench]
fn trie_prefixes_match(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();
    let keys = generate_keys();
    for key in &keys {
        t.insert(key.bytes(), key.clone());
    }
    b.iter(|| {
        for key in &keys {
            assert!(!t.find_prefixes(key.bytes()).is_empty());
        }
    })
}

#[bench]
fn trie_postfixes_match(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();
    let keys = generate_keys();
    for key in &keys {
        t.insert(key.bytes(), key.clone());
    }
    b.iter(|| {
        for key in &keys {
            assert!(!t.find_postfixes(key.bytes()).is_empty());
        }
    })
}

#[bench]
fn trie_prefix_longest_match(b: &mut Bencher) {
    let mut t = oramacore_ptrie::Trie::new();
    let keys = generate_keys();
    for key in &keys {
        t.insert(key.bytes(), key.clone());
    }
    b.iter(|| {
        for key in &keys {
            assert!(t.find_longest_prefix(key.bytes()).is_some());
        }
    })
}
