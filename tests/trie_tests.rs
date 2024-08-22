#[cfg(test)]
mod tests {
    use ptrie::Trie;

    #[test]
    fn new_trie_is_is_empty() {
        assert!(Trie::<char, String>::new().is_empty());
    }

    #[test]
    fn add_word_to_trie() {
        let mut t = Trie::new();
        t.insert("test".bytes(), String::from("test"));
        assert!(!t.is_empty());
    }

    #[test]
    fn contains_key_test() {
        let mut t = Trie::new();
        let test = "test".bytes();
        let tes = "tes".bytes();
        let notintest = "notintest".bytes();

        t.insert(test.clone(), String::from("test"));
        assert!(!t.is_empty());
        assert!(t.contains_key(test));
        assert!(!t.contains_key(tes));
        assert!(!t.contains_key(notintest));
    }

    #[test]
    fn contains_key_sub_path_test() {
        let mut t = Trie::new();
        let test = "test".bytes();
        let tes = "tes".bytes();
        let notintest = "notintest".bytes();

        t.insert(test.clone(), String::from("test"));
        t.insert(tes.clone(), String::from("tes"));
        assert!(!t.is_empty());
        assert!(t.contains_key(test));
        assert!(t.contains_key(tes));
        assert!(!t.contains_key(notintest));
    }

    #[test]
    fn clear_test() {
        let mut t = Trie::new();
        let data = "test".bytes();

        t.insert(data.clone(), String::from("test"));
        assert!(!t.is_empty());
        assert!(t.contains_key(data.clone()));

        t.clear();
        assert!(t.is_empty());
        assert!(!t.contains_key(data));
    }

    #[test]
    fn find_prefixes() {
        let mut trie = Trie::new();
        trie.insert("abc".bytes(), "ABC");
        trie.insert("abcd".bytes(), "ABCD");
        trie.insert("abcde".bytes(), "ABCDE");
        let prefixes = trie.find_prefixes("abcd".bytes());
        assert_eq!(prefixes, vec![&"ABC", &"ABCD"]);
        assert_eq!(trie.find_prefixes("efghij".bytes()), Vec::<&&str>::new());
        assert_eq!(trie.find_prefixes("abz".bytes()), Vec::<&&str>::new());
    }

    #[test]
    fn iterator() {
        let mut t = Trie::new();
        let test = "test".bytes();
        let tes = "tes".bytes();

        t.insert(test.clone(), String::from("test"));
        t.insert(tes.clone(), String::from("tes"));
        // TODO: for (k, v) in &t {
        for (k, v) in &t {
            assert!(std::str::from_utf8(&k).unwrap().starts_with("tes"));
            assert!(v.starts_with("tes"));
        }
    }

    #[test]
    fn test_remove() {
        let mut trie = Trie::new();
        trie.insert("hello".bytes(), 1);
        trie.insert("hell".bytes(), 2);
        trie.insert("h".bytes(), 3);

        assert_eq!(trie.remove("hello".bytes()), Some(1));
        assert_eq!(trie.get("hello".bytes()), None);
        assert_eq!(trie.get("hell".bytes()), Some(&2));
        assert_eq!(trie.get("h".bytes()), Some(&3));

        assert_eq!(trie.remove("h".bytes()), Some(3));
        assert_eq!(trie.get("h".bytes()), None);
        assert_eq!(trie.get("hell".bytes()), Some(&2));

        assert_eq!(trie.remove("nonexistent".bytes()), None);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serialize() {
        use serde_json;
        let mut trie = Trie::new();
        trie.insert("key".bytes(), 42);
        let serialized = serde_json::to_string(&trie).expect("Failed to serialize");
        // println!("serialized! {}", serialized);
        let deserialized: Trie<u8, i32> =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(deserialized.get("key".bytes()), Some(42).as_ref());
    }
}
