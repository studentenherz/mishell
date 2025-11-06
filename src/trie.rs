use std::collections::BTreeMap;

#[derive(Default)]
struct TrieNode {
    pub end_of_word: bool,
    children: BTreeMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(&self, c: char) -> Option<&TrieNode> {
        self.children.get(&c)
    }

    pub fn child_mut(&mut self, c: char) -> &mut TrieNode {
        self.children.entry(c).or_insert_with(TrieNode::new)
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    /// Insert a word into the trie
    pub fn insert(&mut self, word: &str) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            curr = curr.child_mut(c);
        }
        curr.end_of_word = true;
    }

    // Check if the trie contains a word
    #[allow(unused)]
    pub fn contains(&self, word: &str) -> bool {
        let mut curr = &self.root;
        for c in word.chars() {
            match curr.child(c) {
                Some(next) => curr = next,
                None => return false,
            }
        }

        curr.end_of_word
    }

    /// Get all words that start with prefix
    pub fn matches(&self, prefix: &str) -> Vec<String> {
        let mut matches = vec![];
        let mut curr = &self.root;
        for c in prefix.chars() {
            match curr.child(c) {
                Some(next) => curr = next,
                None => return matches,
            }
        }

        let mut prefix = String::from(prefix);
        self._get_contained_words(&mut matches, curr, &mut prefix);

        matches
    }

    fn _get_contained_words(
        &self,
        matches: &mut Vec<String>,
        curr: &TrieNode,
        prefix: &mut String,
    ) {
        if curr.end_of_word {
            matches.push(prefix.to_string());
        }

        for (c, next) in &curr.children {
            prefix.push(*c);
            self._get_contained_words(matches, next, prefix);
            prefix.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_basic() {
        let mut trie = Trie::new();
        trie.insert("myItem");
        trie.insert("myth");
        trie.insert("mine");

        assert!(trie.contains("myItem"));
        assert!(!trie.contains("myIte"));
    }

    #[test]
    fn test_if_finds_matches() {
        let mut trie = Trie::new();
        trie.insert("myItem");
        trie.insert("myItem2");
        trie.insert("myLittlePony");
        trie.insert("myInvention");

        assert!(trie
            .matches("myIt")
            .iter()
            .all(|item| ["myItem", "myItem2"].contains(&item.as_str())));

        assert!(trie
            .matches("myI")
            .iter()
            .all(|item| ["myItem", "myItem2", "myInvention"].contains(&item.as_str())));

        assert!(trie
            .matches("myi")
            .iter()
            .all(|item| [].contains(&item.as_str())));
    }
}
