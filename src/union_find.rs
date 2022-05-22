pub struct UF {
  parents: Vec<usize>,
  sizes: Vec<usize>,
}

impl UF {
  pub fn new(n: usize) -> Self {
    UF {
      parents: (0..n).collect::<Vec<usize>>(),
      sizes: vec![0; n],
    }
  }

  pub fn find(&mut self, i: usize) -> usize {
    if i != self.parents[i] {
      self.parents[i] = self.find(self.parents[i]);
    }
    self.parents[i]
  }

  pub fn union(&mut self, i: usize, j: usize) -> bool {
    let mut i_parent = self.find(i);
    let mut j_parent = self.find(j);
    if i_parent == j_parent {
      return false;
    }
    if self.sizes[i_parent] > self.sizes[j_parent] {
      std::mem::swap(&mut i_parent, &mut j_parent);
    }
    self.parents[i_parent] = j_parent;
    self.sizes[j_parent] += self.sizes[i_parent];
    true
  }

  pub fn size(&self, i: usize) -> usize {
    self.sizes[i]
  }
}
