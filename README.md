<h1 align="center">
  🎄 Prefix Trie
</h1>

<p align="center">
    <a href="https://crates.io/crates/ptrie">
        <img alt="Crates.io" src="https://img.shields.io/crates/v/ptrie" />
    </a>
    <a href="https://github.com/vemonet/ptrie/actions/workflows/test.yml">
        <img alt="Test" src="https://github.com/vemonet/ptrie/actions/workflows/test.yml/badge.svg" />
    </a>
    <a href="https://github.com/vemonet/ptrie/actions/workflows/release.yml">
        <img alt="Release" src="https://github.com/vemonet/ptrie/actions/workflows/release.yml/badge.svg" />
    </a>
    <a href="https://docs.rs/ptrie">
        <img alt="Documentation" src="https://docs.rs/ptrie/badge.svg" />
    </a>
    <a href="https://codecov.io/gh/vemonet/ptrie/branch/main">
        <img src="https://codecov.io/gh/vemonet/ptrie/branch/main/graph/badge.svg" alt="Codecov status" />
    </a>
    <a href="https://github.com/vemonet/ptrie/blob/main/LICENSE">
        <img alt="MIT license" src="https://img.shields.io/badge/License-MIT-brightgreen.svg" />
    </a>
</p>

`PTrie` is a generic implementation of the [trie data structure](https://en.wikipedia.org/wiki/Trie) with no dependencies, tailored for easy and efficient prefix and postfix search within a collection of objects, such as strings.

The structure is defined as `Trie<K, V>`, where `K` represents the type of keys in each node (an iterator of the chain to index), and `V` is the type of the associated values (any object to which the key points to).

## 💭 Motivation

The trie is particularly effective for operations involving common  prefix identification and retrieval, making it a good choice for  applications that require fast and efficient prefix-based search  functionalities.

## 🚀 Usage

Results are sorted in ascending order of their length.

### ✨ Find prefixes

You can return all prefixes in the trie that matches a given string, or directly retrieve the longest prefix.

```rust
use ptrie::Trie;

let mut trie = Trie::new();

trie.insert("a".bytes(), "A");
trie.insert("ab".bytes(), "AB");
trie.insert("abc".bytes(), "ABC");
trie.insert("abcde".bytes(), "ABCDE");

// Find all potential prefixes
let prefixes = trie.find_prefixes("abcd".bytes());
assert_eq!(prefixes, vec![&"A", &"AB", &"ABC"]);

// Find the longest prefix
let longest = trie.find_longest_prefix("abcd".bytes());
assert_eq!(longest, Some("ABC").as_ref());

// Find longest with length
if let Some((length, prefix)) = trie.find_longest_prefix_len("abcd".bytes()) {
    println!("Longest prefix: {} {}", prefix, length);
}
```

### 🔍 Find postfixes

You can also find all postfixes in the trie, e.g. all strings which have the given string as a prefix, and extends it.

```rust
use ptrie::Trie;

let mut trie = Trie::new();

trie.insert("app".bytes(), "App");
trie.insert("apple".bytes(), "Apple");
trie.insert("applet".bytes(), "Applet");
trie.insert("apricot".bytes(), "Apricot");

let strings = trie.find_postfixes("app".bytes());
assert_eq!(strings, vec![&"App", &"Apple", &"Applet"]);
```

### 🔑 Key-based retrieval functions

The crate provides functions to check for the existence of a key, to retrieve the associated value, or iterate the trie nodes.

```rust
use ptrie::Trie;

let mut trie = Trie::new();
trie.insert("app".bytes(), "App");
trie.insert("applet".bytes(), "Applet");

// Get a key
assert!(trie.contains_key("app".bytes()));
assert!(!trie.contains_key("not_existing_key".bytes()));
assert_eq!(trie.get("app".bytes()), Some("App").as_ref());
assert_eq!(trie.get("none".bytes()), None.as_ref());

// Iterate the trie
for (k, v) in &trie {
    println!("kv: {:?} {}", k, v);
}

// Remove a key
trie.remove("app".bytes());
assert!(!trie.contains_key("app".bytes()));
```

## 🏷️ Features

The `serde` feature adds Serde `Serialize` and `Deserialize` traits to the `Trie` and `TrieNode` struct.

```toml
ptrie = { version = "0.6", features = ["serde"] }
```

## 🛠️ Contributing

Contributions are welcome, checkout the [`CONTRIBUTING.md`](https://github.com/vemonet/ptrie/blob/main/CONTRIBUTING.md) for instructions to run the project in development.

## 📜 Changelog

Changelog available in the [`CHANGELOG.md`](https://github.com/vemonet/ptrie/blob/main/CHANGELOG.md).

## ⚖️ License

[MIT License](https://opensource.org/licenses/MIT)
