#[derive(Debug, Clone)]
enum State {
  Unvisited,
  InStack,
  Done,
}

fn get_cycle(node: usize, st: &mut Vec<usize>) -> Vec<usize> {
  let mut st2 = vec![];
  while let Some(top) = st.pop() {
    st2.push(top);
    if top == node {
      break;
    }
  }
  let mut cycle = st2.clone();
  cycle.reverse();
  while let Some(top) = st2.pop() {
    st.push(top);
  }
  cycle
}

fn dfs(
  graph: &Vec<Vec<usize>>,
  st: &mut Vec<usize>,
  visited: &mut Vec<State>,
  cycles: &mut Vec<Vec<usize>>,
) {
  for child in &graph[st[st.len() - 1]] {
    match &visited[*child] {
      State::Unvisited => {
        visited[*child] = State::InStack;
        st.push(*child);
        dfs(graph, st, visited, cycles);
      }
      State::InStack => {
        let cycle = get_cycle(*child, st);
        cycles.push(cycle);
      }
      State::Done => {}
    }
  }
  visited[st[st.len() - 1]] = State::Done;
  st.pop();
}

fn all_unique_cycles(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let mut visited = vec![State::Unvisited; graph.len()];
  let mut cycles = vec![];
  for i in 0..graph.len() {
    match visited[i] {
      State::Unvisited => {
        let mut st = vec![i];
        visited[i] = State::InStack;
        dfs(graph, &mut st, &mut visited, &mut cycles);
      }
      _ => {}
    }
  }
  cycles
}

fn johnsons() {}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn no_cycles() {
    let graph = vec![vec![1], vec![2], vec![]];
    let cycles = all_unique_cycles(&graph);
    let empty: Vec<Vec<usize>> = vec![];
    assert_eq!(empty, cycles);
  }

  #[test]
  fn simple_cycle() {
    let graph = vec![vec![1], vec![0]];
    let cycles = all_unique_cycles(&graph);
    let two_cycle = vec![vec![0, 1]];
    assert_eq!(two_cycle, cycles);
  }

  #[test]
  fn simple_cycle_with_more_nodes() {
    let graph = vec![vec![1], vec![2], vec![0, 3], vec![]];
    let cycles = all_unique_cycles(&graph);
    let three_cycle = vec![vec![0, 1, 2]];
    assert_eq!(three_cycle, cycles);
  }

  #[test]
  fn two_cycles() {
    let graph = vec![vec![1], vec![0], vec![3], vec![2]];
    let cycles = all_unique_cycles(&graph);
    let two_cycles = vec![vec![0, 1], vec![2, 3]];
    assert_eq!(two_cycles, cycles);
  }

  #[test]
  fn two_overlapping_cycles() {
    // 0 --> 1, 1 --> 0, 1 --> 2, 2 --> 1
    let graph = vec![vec![1], vec![0, 2], vec![1]];
    let cycles = all_unique_cycles(&graph);
    let two_cycles = vec![vec![0, 1], vec![1, 2]];
    assert_eq!(two_cycles, cycles);
  }

  #[test]
  fn three_overlapping_cycles() {
    // 0 --> 1, 1 --> 0, 1 --> 2, 2 --> 1, 0 --> 2
    let graph = vec![vec![1, 2], vec![0, 2], vec![1]];
    let cycles = all_unique_cycles(&graph);
    let three_cycles = vec![vec![0, 1], vec![1, 2], vec![0, 1, 2]];
    assert_eq!(three_cycles, cycles);
  }
}
