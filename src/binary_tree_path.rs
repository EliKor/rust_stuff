use std::cmp::max;

#[derive(Debug)]
struct TreeNode {
    val: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: u32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode { val, left, right }
    }
}

fn longest_path(root: Option<Box<TreeNode>>, max_len: &mut u32) -> u32 {
    match root {
        None => 0,
        Some(node) => {
            let left_len = longest_path(node.left, max_len);
            let right_len = longest_path(node.right, max_len);
            *max_len = max(*max_len, right_len + left_len);
            1 + max(right_len, left_len)
        }
    }
}

/*
 *        1
 *      2   3
 *    4   9   5
 *  6   7      8
 *
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn general_test() {
        let six = Box::new(TreeNode::new(5, None, None));
        let seven = Box::new(TreeNode::new(7, None, None));
        let eight = Box::new(TreeNode::new(8, None, None));
        let four = Box::new(TreeNode::new(4, Some(six), None));
        let nine = Box::new(TreeNode::new(9, Some(seven), None));
        let five = Box::new(TreeNode::new(5, None, Some(eight)));
        let two = Box::new(TreeNode::new(2, Some(four), None));
        let three = Box::new(TreeNode::new(3, Some(nine), Some(five)));
        let one = Box::new(TreeNode::new(1, Some(two), Some(three)));
        let mut max_len = 0;
        longest_path(Some(one), &mut max_len);
        assert_eq!(6, max_len);
    }
}

#[allow(dead_code)]
fn main() {}
