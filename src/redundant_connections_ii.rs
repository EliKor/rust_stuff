/*
Starts out as a rooted tree with a root with in-degree of 0 but we added another edge
This means that either one node has two parents or the root now has a parent

If the root now has a parent then we have a cycle, we should return the last edge in the list in this cycle

If one node now has two parents, we know that one of the two parent-child edges involving this node can be removed
To decide which one to remove we can try excluding either one and seeing if we can still reach all nodes from the root
This would require 2 more traversals of the rooted tree though which is more than necessary
Instead, we can just not include the second parent-child edge that we find.
If we don't end up detecting a cycle, we can just remove this edge because either not including it stopped a cycle or there was no cycle to begin
with and it is the later of the two edges
If we do end up detecting a cycle then we remove the first parent-child edge because removing the second didn't stop a cycle from forming

Open questions:
- Why can we use Union-Find in this case of a directed graph?
  - Because of the problem setup, a cycle found via union-find will be a legitimate
    cycle because any "pseudo-cycles" (falsely detected cycles) will be first captured by
    the two parent case. This is because these "pseudo-cycles" must consist of a node with in-degree 2
- What does finding a cycle mean when the root is not included?
  - Cycles would never have been found in the original rooted tree, so one of the edges
    in the cycle must be an invalid edge.
  - If we find a cycle first and then the second parent edge, we know the second parent edge isn't part of the cycle
    so it shouldn't be removed.
  - If we find the second parent edge first and then we find a cycle, we know that we didn't consider this second edge
    so it can't be a part of the cycle and so shouldn't be removed
  - In both cases, we should remove the first of the two parent edges we find assuming we also found a cycle

Input: edges = [[1,2],[1,3],[2,3]]
Output: [2,3]

Input: edges = [[1,2],[2,3],[3,4],[4,1],[1,5]]
Output: [4,1]

Input: edges = [[1,2],[2,3][3,1]]
*/

use crate::union_find::UF;

// Given edges, return an edge that can be removed without making
// it impossible to connect all of the nodes
fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
  vec![0, 0]
}

fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
  let n = edges.len();
  let mut uf = UF::new(n);
  let mut parents = vec![0; n];
  let (mut two_parent_edge1, mut two_parent_edge2, mut final_cycle_edge) = (None, None, None);
  for edge in edges {
    let parent = edge[0] as usize;
    let child = edge[1] as usize;
    if parents[child - 1] != 0 {
      // parent has already been assigned
      two_parent_edge1 = Some(vec![parents[child - 1] as i32, child as i32]);
      two_parent_edge2 = Some(edge.clone());
    } else {
      parents[child - 1] = parent;
      if !uf.union(parent - 1, child - 1) {
        // there was a cycle found
        final_cycle_edge = Some(edge.clone());
      }
    }
  }
  match (two_parent_edge1, two_parent_edge2, final_cycle_edge) {
    (Some(tp_edge1), Some(_), Some(_)) => tp_edge1, // There was a cycle and a two parent node
    (Some(_), Some(tp_edge2), None) => tp_edge2,    // There was a two parent node
    (None, _, Some(cycle_edge)) => cycle_edge,      // There was just a cycle
    _ => panic!("Invalid Input"),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn finds_redundant_connection1() {
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    let res = find_redundant_directed_connection(edges);
    let sol = vec![2, 3];
    assert_eq!(sol, res);
  }

  #[test]
  fn finds_redundant_connection2() {
    let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]];
    let res = find_redundant_directed_connection(edges);
    let sol = vec![4, 1];
    assert_eq!(sol, res);
  }

  #[test]
  fn finds_redundant_connection3() {
    let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![5, 2]];
    let res = find_redundant_directed_connection(edges);
    let sol = vec![1, 2];
    assert_eq!(sol, res);
  }

  #[test]
  fn finds_redundant_connection4() {
    let edges = vec![vec![4, 2], vec![1, 5], vec![5, 2], vec![5, 3], vec![2, 4]];
    let res = find_redundant_directed_connection(edges);
    let sol = vec![4, 2];
    assert_eq!(sol, res);
  }
}
