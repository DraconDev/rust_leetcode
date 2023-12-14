use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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

pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(n: &Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        match n {
            None => l.max(r),
            Some(n) => {
                let v = n.borrow();
                dfs(&v.left, 0, l + 1).max(dfs(&v.right, r + 1, 0))
            }
        }
    }
    dfs(&root, 0, 0) - 1
}

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

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let (Some(rn), Some(pn), Some(qn)) = (&root, &p, &q) {
        if rn.borrow().val == pn.borrow().val || rn.borrow().val == qn.borrow().val {
            return root;
        }

        let left = Self::lowest_common_ancestor(
            rn.borrow().left.as_ref().map(Rc::clone),
            Some(Rc::clone(pn)),
            Some(Rc::clone(qn)),
        );

        let right = Self::lowest_common_ancestor(
            rn.borrow().right.as_ref().map(Rc::clone),
            Some(Rc::clone(pn)),
            Some(Rc::clone(qn)),
        );

        if left.is_some() && right.is_some() {
            return root;
        }

        if left.is_some() {
            return left;
        }

        return right;
    }
    None
}
