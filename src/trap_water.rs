use std::cmp::{max, min};

pub fn trap_stack(height: Vec<i32>) -> i32 {
  let mut st: Vec<usize> = vec![];
  let mut trapped_water = 0;
  for i in 0..height.len() {
    while st.len() > 0 && height[i] > height[st[st.len() - 1]] {
      let top_idx = st.pop().unwrap();
      if st.len() == 0 {
        break;
      }
      let top_height = height[top_idx];
      let left_idx = st[st.len() - 1];
      let left_height = height[left_idx];
      let height = min(height[i], left_height) - top_height;
      let width = i - left_idx - 1;
      trapped_water += height * (width as i32);
    }
    st.push(i);
  }
  trapped_water
}

pub fn trap(height: Vec<i32>) -> i32 {
  let (mut left, mut right, mut left_max_height, mut right_max_height) =
    (0, height.len() - 1, 0, 0);
  let mut total_water = 0;
  while left < right {
    if height[left] < height[right] {
      left_max_height = max(left_max_height, height[left]);
      total_water += left_max_height - height[left];
      left += 1;
    } else {
      right_max_height = max(right_max_height, height[right]);
      total_water += right_max_height - height[right] as i32;
      right -= 1;
    }
  }
  total_water
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn simple_water() {
    let heights = vec![1, 0, 1];
    assert_eq!(trap_stack(heights), 1);
  }

  #[test]
  fn simple_unbounded() {
    let heights = vec![0, 1, 0];
    assert_eq!(trap_stack(heights), 0);
  }

  #[test]
  fn simple_unbounded_left() {
    let heights = vec![0, 1, 2];
    assert_eq!(trap_stack(heights), 0);
  }

  #[test]
  fn simple_unbounded_right() {
    let heights = vec![2, 1, 0];
    assert_eq!(trap_stack(heights), 0);
  }

  #[test]
  fn simple_bounded() {
    let heights = vec![0, 1, 0, 2, 0];
    assert_eq!(trap_stack(heights), 1);
  }

  #[test]
  fn equal_heights() {
    let heights = vec![0, 2, 2, 2, 0];
    assert_eq!(trap_stack(heights), 0);
  }

  #[test]
  fn sample1() {
    let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(trap_stack(heights), 6);
  }
}
