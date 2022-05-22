use std::cmp::max;
/*
  Brute force: Take all substrings and evaluate them (n + n-1 + n-2 + n-3 ... 2 ==> n^2 different substrings each of which takes O(n) so O(n^3))
  Idea: Treat it like the palindrome problem where you can either start as () or )( O(n^2)
  Idea: Use DP for an O(n^2) algorithm on the start and end of the parens
  Seems like we should be able to do it in O(n) but the single pass approach seems really tricky

  How to ignore previously seen parens so you don't miss a valid substring but also include them later in case they become useful

  Is there a way for a substring to never be able to be valid again? The first possible valid substring must have seen a ( first
  and then following that cannot see more ) than ( seen previously.
  The moment we cross this threshold is the moment we can restart trying to find the newest substring

  ((())(( --> 4
  ((())(() --> Seems like 5 naively but is still 4, if we see opens after closing some we need to get rid of all of these opens before considering the previous opens
  So in the above case we close off two opens and have the furthest left open, but then we have two more opens that need to be closed before we are valid again


  Optimal/Easier solution:
  Keep track of index of last seen open paren and whenever you see a closed paren pop from stack and the top of the stack as index so long as st is not empty
  If the stack is empty then you've seen too many closed parens for any of the previous to be valid, so insert the closed paren's index
  This is a lot like validating parens but with the clever initialization and the use of indexes
*/
pub fn longest_valid_parens(s: String) -> i32 {
  let mut st = vec![];
  let mut open_parens = 0;
  let mut total_valid = 0;
  let mut curr_valid = 0;
  let mut closing = false;
  let bytes = s.as_bytes();
  for c in bytes {
    match *c {
      b'(' if closing => {
        closing = false;
        st.push((curr_valid, open_parens));
        curr_valid = 0;
        open_parens = 1;
      }
      b'(' => {
        open_parens += 1;
      }
      b')' => {
        closing = true;
        open_parens -= 1;
        if open_parens < 0 {
          curr_valid = 0;
          continue;
        }
        curr_valid += 1;
        total_valid = max(total_valid, curr_valid);
        if open_parens == 0 {
          match st.pop() {
            Some((prev_valid, prev_open_parens)) => {
              open_parens = prev_open_parens;
              curr_valid += prev_valid;
              total_valid = max(total_valid, curr_valid);
            }
            None => st.push((curr_valid, open_parens)),
          }
        }
      }
      _ => {}
    }
  }
  total_valid * 2
}

fn longest_valid_parens_2(s: String) -> i32 {
  let bytes = s.as_bytes();
  let mut total_valid = 0;
  let mut st = vec![-1];
  for i in 0..bytes.len() {
    if bytes[i] == b'(' {
      st.push(i as i32);
    } else {
      st.pop();
      if st.len() == 0 {
        st.push(i as i32);
      } else {
        total_valid = max(total_valid, i as i32 - st[st.len() - 1]);
      }
    }
  }
  total_valid
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn empty_string() {
    assert_eq!(longest_valid_parens("".to_string()), 0);
    assert_eq!(longest_valid_parens_2("".to_string()), 0);
  }

  #[test]
  fn single_paren() {
    assert_eq!(longest_valid_parens("(".to_string()), 0);
    assert_eq!(longest_valid_parens_2("(".to_string()), 0);
  }

  #[test]
  fn simple_valid() {
    assert_eq!(longest_valid_parens("()".to_string()), 2);
    assert_eq!(longest_valid_parens_2("()".to_string()), 2);
  }

  #[test]
  fn simple_invalid() {
    assert_eq!(longest_valid_parens(")(".to_string()), 0);
    assert_eq!(longest_valid_parens_2(")(".to_string()), 0);
  }

  #[test]
  fn middle_valid() {
    assert_eq!(longest_valid_parens("((()()((".to_string()), 4);
    assert_eq!(longest_valid_parens_2("((()()((".to_string()), 4);
  }

  #[test]
  fn end_valid() {
    assert_eq!(longest_valid_parens("((()()(())".to_string()), 8);
    assert_eq!(longest_valid_parens_2("((()()(())".to_string()), 8);
  }

  #[test]
  fn long_valid() {
    assert_eq!(
      longest_valid_parens("((((((((((((((((()))))))))))())))))))()".to_string()),
      36
    );
    assert_eq!(
      longest_valid_parens_2("((((((((((((((((()))))))))))())))))))()".to_string()),
      36
    );
  }

  #[test]
  fn longer_valid() {
    let input = "(()()()()()((((((())))))".to_string();
    assert_eq!(longest_valid_parens(input.clone()), 12);
    assert_eq!(longest_valid_parens_2(input), 12);
  }

  #[test]
  fn lots_of_singles() {
    let input = "()()()(((((()())()(())()())".to_string();
    assert_eq!(longest_valid_parens(input.clone()), 18);
    assert_eq!(longest_valid_parens_2(input), 18);
  }

  #[test]
  fn some_singles() {
    let input = "()()()((()))()".to_string();
    assert_eq!(longest_valid_parens(input), 14);
  }

  #[test]
  fn just_singles() {
    let input = "()()()()".to_string();
    assert_eq!(longest_valid_parens(input), 8);
  }

  #[test]
  fn open_with_singles() {
    let input = "()(()()()".to_string();
    assert_eq!(longest_valid_parens(input), 6);
  }

  #[test]
  fn closed_with_singles() {
    let input = "()()()())".to_string();
    assert_eq!(longest_valid_parens(input), 8);
  }
}
