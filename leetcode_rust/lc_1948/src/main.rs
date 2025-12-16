use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Default)]
struct Trie {
    serial: String,
    children: HashMap<String, Trie>,
}

fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut root = Trie::default();
    for path in paths {
        let mut cur = &mut root;
        for folder in path {
            cur = cur.children.entry(folder.clone()).or_default();
        }
    }

    let mut counter = HashMap::<String, usize>::new();
    fn construct(node: &mut Trie, counter: &mut HashMap<String, usize>) {
        if node.children.is_empty() {
            return;
        }

        let mut serial = Vec::<String>::new();
        for (folder, child) in node.children.iter_mut() {
            construct(child, counter);
            serial.push(format!("{}({})", folder, child.serial));
        }

        serial.sort();
        node.serial = serial.join("");
        *counter.entry(node.serial.clone()).or_default() += 1;
    }

    construct(&mut root, &mut counter);
    let mut ans = Vec::new();
    let mut path = Vec::new();
    fn dfs(node: &Trie, counter: &HashMap<String, usize>, path: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
        if counter.get(&node.serial).unwrap_or(&0) > &1 {
            return;
        }
        if !path.is_empty() {
            ans.push(path.clone());
        }
        for (folder, child) in &node.children {
            path.push(folder.clone());
            dfs(child, counter, path, ans);
            path.pop();
        }
    }
    dfs(&root, &counter, &mut path, &mut ans);
    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicate_folder() {
        let paths = vec![
            vec!["a".to_string()],
            vec!["c".to_string()],
            vec!["d".to_string()],
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["d".to_string(), "a".to_string()],
        ];
        
        let result = delete_duplicate_folder(paths);
        
        // Expected: [["d"], ["d", "a"]] because folders "a" and "c" have identical structure
        // Both "a" and "c" contain only folder "b", so they are duplicates and should be removed
        let mut expected = vec![
            vec!["d".to_string()],
            vec!["d".to_string(), "a".to_string()],
        ];
        
        // Sort both vectors for comparison since order might vary
        let mut result_sorted = result;
        result_sorted.sort();
        expected.sort();
        
        assert_eq!(result_sorted, expected);
    }
}
