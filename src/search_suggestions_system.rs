use crate::trie::TrieNode;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

type TrieState = BinaryHeap<String>;

fn update_state(node: &mut TrieNode<TrieState>, word: &String) {
  node.state.push(word.clone());
  if node.state.len() > 3 {
    node.state.pop();
  }
}

impl Solution {
  pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let mut res = vec![];
    let root = TrieNode::<TrieState>::trie_from_words(&products, Some(update_state));
    let mut curr = Some(&root);
    for c in search_word.as_bytes() {
      curr = curr.and_then(|val| val.children.get(c));
      match curr {
        Some(trie_node) => {
          let mut state = trie_node.state.clone();
          let mut top_words = std::iter::repeat_with(|| state.pop())
            .take(3)
            .filter_map(|x| x)
            .collect::<Vec<String>>();
          top_words.reverse();
          res.push(top_words);
        }
        None => {
          res.push(vec![]);
        }
      }
    }
    res
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let products = vec![
      "mobile".to_string(),
      "mouse".to_string(),
      "moneypot".to_string(),
      "monitor".to_string(),
      "mousepad".to_string(),
    ];
    let search_word = "mouse".to_string();
    let sol = vec![
      vec![
        "mobile".to_string(),
        "moneypot".to_string(),
        "monitor".to_string(),
      ],
      vec![
        "mobile".to_string(),
        "moneypot".to_string(),
        "monitor".to_string(),
      ],
      vec!["mouse".to_string(), "mousepad".to_string()],
      vec!["mouse".to_string(), "mousepad".to_string()],
      vec!["mouse".to_string(), "mousepad".to_string()],
    ];
    assert_eq!(Solution::suggested_products(products, search_word), sol);
  }
}
