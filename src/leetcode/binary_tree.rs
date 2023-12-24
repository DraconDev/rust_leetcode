use std::borrow::{Borrow, BorrowMut};

use std::collections::{HashMap, VecDeque};

use crate::Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// build example binary tree of 2 levels
fn build_tree_helper(
    val: i32,
    left_values: Vec<Option<i32>>,
    right_values: Vec<Option<i32>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = TreeNode::new(val);

    if !left_values.is_empty() {
        root.left = build_tree_helper(
            left_values[0].unwrap(),
            left_values[1..].to_vec(),
            Vec::new(),
        );
    }

    if !right_values.is_empty() {
        root.right = build_tree_helper(
            right_values[0].unwrap(),
            Vec::new(),
            right_values[1..].to_vec(),
        );
    }

    Some(Rc::new(RefCell::new(root)))
}

// Build example binary tree of 2 levels
pub fn build_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let left_values = vec![2, 4, 5, 8, 9].into_iter().map(Some).collect();
    let right_values = vec![3, 6, 7, 10].into_iter().map(Some).collect();

    build_tree_helper(1, left_values, right_values)
}

// pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     fn dfs(n: &Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
//         match n {
//             None => l.max(r),
//             Some(n) => {
//                 let v = n.borrow();
//                 dfs(&v.left, 0, l + 1).max(dfs(&v.right, r + 1, 0))
//             }
//         }
//     }
//     dfs(&root, 0, 0) - 1
// }

// pub fn lowest_common_ancestor(
//     root: Option<Rc<RefCell<TreeNode>>>,
//     p: Option<Rc<RefCell<TreeNode>>>,
//     q: Option<Rc<RefCell<TreeNode>>>,
// ) -> Option<Rc<RefCell<TreeNode>>> {
//     fn dfs(
//         n: &Option<Rc<RefCell<TreeNode>>>,
//         p: &Option<Rc<RefCell<TreeNode>>>,
//         q: &Option<Rc<RefCell<TreeNode>>>,
//     ) -> Option<Rc<RefCell<TreeNode>>> {
//         match n {
//             None => None,
//             Some(n) => {
//                 let v = n.borrow();
//                 if let (Some(p_node), Some(q_node)) = (p.as_ref(), q.as_ref()) {
//                     let p_val = p_node.borrow().val;
//                     let q_val = q_node.borrow().val;

//                     if v.val > p_val && v.val > q_val {
//                         dfs(&v.left, p, q)
//                     } else if v.val < p_val && v.val < q_val {
//                         dfs(&v.right, p, q)
//                     } else {
//                         Some(n.clone())
//                     }
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
//     dfs(&root, &p, &q)
// }

// pub fn lowest_common_ancestor(
//     root: Option<Rc<RefCell<TreeNode>>>,
//     p: Option<Rc<RefCell<TreeNode>>>,
//     q: Option<Rc<RefCell<TreeNode>>>,
// ) -> Option<Rc<RefCell<TreeNode>>> {
//     if let (Some(rn), Some(pn), Some(qn)) = (&root, &p, &q) {
//         if rn.borrow().val == pn.borrow().val || rn.borrow().val == qn.borrow().val {
//             return root;
//         }

//         let left = Self::lowest_common_ancestor(
//             rn.borrow().left.as_ref().map(Rc::clone),
//             Some(Rc::clone(pn)),
//             Some(Rc::clone(qn)),
//         );

//         let right = Self::lowest_common_ancestor(
//             rn.borrow().right.as_ref().map(Rc::clone),
//             Some(Rc::clone(pn)),
//             Some(Rc::clone(qn)),
//         );

//         if left.is_some() && right.is_some() {
//             return root;
//         }

//         if left.is_some() {
//             return left;
//         }

//         return right;
//     }
//     None
// }

// pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     let mut queue = VecDeque::<Option<(Rc<RefCell<TreeNode>>, i32)>>::new();

//     let mut sums = HashMap::new();

//     queue.push_back(root.map(|r| (Rc::clone(&r), 1)));

//     fn bfs(
//         queue: &mut VecDeque<Option<(Rc<RefCell<TreeNode>>, i32)>>,
//         sums: &mut HashMap<i32, i32>,
//     ) {
//         while let Some(Some((n, l))) = queue.pop_front() {
//             let v = n.borrow();

//             sums.entry(l).and_modify(|e| *e += v.val).or_insert(v.val);

//             if let Some(left) = v.left.clone() {
//                 queue.push_back(Some((left, l + 1)));
//             }

//             if let Some(right) = v.right.clone() {
//                 queue.push_back(Some((right, l + 1)));
//             }
//         }
//     }

//     bfs(&mut queue, &mut sums);

//     let max_sum = sums.values().max().unwrap_or(&0).clone();
//     max_sum
// }

// pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
//     fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
//         match root {
//             None => None,
//             Some(n) => {
//                 let v = n.borrow();
//                 if v.val == val {
//                     Some(Rc::clone(n))
//                 } else if v.val > val {
//                     dfs(&v.left, val)
//                 } else {
//                     dfs(&v.right, val)
//                 }
//             }
//         }
//     }
//     dfs(&root, val)
// }

// pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//     fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
//         match root {
//             None => (true, 0), // An empty tree is balanced with a height of 0.
//             Some(n) => {
//                 let v = n.borrow();
//                 let (left_balanced, left_height) = dfs(&v.left);
//                 let (right_balanced, right_height) = dfs(&v.right);

//                 let balanced =
//                     left_balanced && right_balanced && (left_height - right_height).abs() <= 1;
//                 let height = 1 + std::cmp::max(left_height, right_height);

//                 (balanced, height)
//             }
//         }
//     }

//     dfs(&root).0
// }

// type Node = Option<Rc<RefCell<TreeNode>>>;

// pub fn delete_node(mut root: Node, key: i32) -> Node {
//     fn rec(node1: Node, node2: Node, key: i32, fnd: bool) -> Node {
//         match (node1, node2) {
//             (None, None) => None,
//             (None, Some(nd)) | (Some(nd), None) => {
//                 if fnd {
//                     Some(nd)
//                 } else {
//                     let v = nd.borrow().val;
//                     if v == key {
//                         return rec(
//                             nd.borrow().left.clone(),
//                             nd.borrow().right.clone(),
//                             key,
//                             true,
//                         );
//                     }
//                     {
//                         let mut nd = nd.borrow_mut();
//                         if v < key {
//                             nd.right = rec(None, nd.right.clone(), key, false);
//                         }
//                         if v > key {
//                             nd.left = rec(None, nd.left.clone(), key, false);
//                         }
//                     }
//                     Some(nd)
//                 }
//             }
//             (ndl, Some(ndr)) => {
//                 {
//                     let mut ndr = ndr.borrow_mut();
//                     ndr.left = rec(ndl, ndr.left.clone(), key, fnd);
//                 }
//                 Some(ndr)
//             }
//         }
//     }
//     rec(None, root, key, false)
// }
use std::cell::RefCell;

use std::rc::Rc;

// impl Solution {
//     pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         match root {
//             Some(node) => {
//                 let left = Self::min_depth.as_ref().borrow().left.clone());
//                 let right = Self::min_depth.as_ref().borrow().right.clone());

//                 if left == 0 || right == 0 {
//                     return left.max(right) + 1
//                 }

//                 left.min(right) + 1
//             },
//             None => 0,
//         }
//     }
// }

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            mut current_sum: i32,
            target_sum: i32,
        ) -> bool {
            match root {
                None => false,
                Some(n) => {
                    let v = n.as_ref().borrow();
                    current_sum += v.val;
                    if v.left.is_none() && v.right.is_none() {
                        if current_sum == target_sum {
                            return true;
                        }
                    }
                    dfs(&v.left, current_sum, target_sum) || dfs(&v.right, current_sum, target_sum)
                }
            }
        }
        dfs(&root, 0, target_sum)
    }
}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            match root {
                None => (),
                Some(n) => {
                    let v = n.as_ref().borrow();
                    result.push(v.val);
                    dfs(&v.left, result);
                    dfs(&v.right, result);
                }
            }
        }
        dfs(&root, &mut result);
        result
    }
}
