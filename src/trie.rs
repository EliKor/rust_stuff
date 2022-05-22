use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode<T: Default> {
  pub children: HashMap<u8, Box<TrieNode<T>>>,
  pub state: T,
}

pub type UpdateStateFn<T> = fn(&mut TrieNode<T>, &String) -> ();

impl<T: Default> TrieNode<T> {
  pub fn add_word(&mut self, word: &String, update_state: Option<UpdateStateFn<T>>) {
    let mut curr = self;
    if let Some(update_fn) = update_state {
      update_fn(curr, word);
    }
    for c in word.as_bytes() {
      curr = curr
        .children
        .entry(*c)
        .or_insert(Box::new(TrieNode::default()));
      if let Some(update_fn) = update_state {
        update_fn(curr, word);
      }
    }
  }

  pub fn trie_from_words(words: &Vec<String>, update_state: Option<UpdateStateFn<T>>) -> Box<Self> {
    let mut root = Box::new(TrieNode::default());
    for word in words {
      root.add_word(word, update_state);
    }
    root
  }
}
