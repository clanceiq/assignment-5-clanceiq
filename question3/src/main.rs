use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    chs: HashMap<char, TrieNode>, // children nodes
    value: Option<i32>,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode {
            chs: HashMap::new(),
            value: None,
            },
        }   
    }

    fn add_string(&mut self, string: String, value: i32) {
        let mut current_node = &mut self.root;
        for c in string.chars() {
            current_node = current_node.chs
                .entry(c)
                .or_insert(TrieNode {
                    chs: HashMap::new(),
                    value: None,
                });
        }
        current_node.value = Some(value);
    }

    fn length(&self) -> usize {
        // recursive call for counting nodes
        fn count(node: &TrieNode) -> usize {
            let mut total = if node.value.is_some() { 1 } else { 0 };
            for child in node.chs.values() {
                total += count(child);
            }
            total
        }
        count(&self.root)
    }

    fn iter(&self) -> Vec<(char, Option<i32>)> {
        let mut result = Vec::new();

        // recursive call for traversing the trie
        fn traverse(node: &TrieNode, result: &mut Vec<(char, Option<i32>)>) {
            for (ch, child) in &node.chs {
                result.push((*ch, child.value));
                traverse(child, result);
            }
        }

        traverse(&self.root, &mut result);
        result
    }

    fn find(&self, key: &String) -> Option<&TrieNode> {
        let mut current_node = &self.root;

        for c in key.chars() {
            match current_node.chs.get(&c) {
                Some(node) => current_node = node,
                None => return None,
            }
        }
        Some(current_node)
    }

    fn delete(&mut self, key: &String) -> Option<i32> {
        let chars: Vec<char> = key.chars().collect(); // string --> char vector for indexing
       
        fn delete_helper(node: &mut TrieNode, chars: &[char]) -> (Option<i32>, bool) {
            // end of the word -- delete and check if node should be deleted
            if chars.is_empty() {
                let old = node.value.take();
                let should_delete = node.chs.is_empty();
                return (old, should_delete);
            }

            let c = chars[0]; // get first character
            if let Some(child) = node.chs.get_mut(&c) { // check if character exists in Trie
                let (res, delete_child) = delete_helper(child, &chars[1..]); // recursive call for next node

                // if safe to delete, remove the child node
                if delete_child {
                    node.chs.remove(&c);
                }

                // delete if current node has no children and has no value
                let should_delete = node.chs.is_empty() && node.value.is_none();
                return (res, should_delete);
            }

            // the key does not exist in the trie, do not delete
            (None, false)
        }

        let (res, _) = delete_helper(&mut self.root, &chars);
        res
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Bar".to_string(), 2);
    println!("{:#?}", trie);
    println!("-------------------");

    // Testing functions
    println!("Length: {}", trie.length());
    println!("-------------------");
    println!("Iter: {:?}", trie.iter());
    println!("-------------------");
    println!("Find 'Bar': {:?}", trie.find(&"Bar".to_string()));
    println!("-------------------");
    println!("Delete 'Bar': {:?}", trie.delete(&"Bar".to_string()));
    println!("After delete: {:#?}", trie);
}