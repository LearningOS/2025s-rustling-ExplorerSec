/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM DONE
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut ref_TreeNode:&mut Option<_> =&mut self.root;
        while let Some(ref mut tree_node) = ref_TreeNode{
            if value == tree_node.value{
                return (); // 防止重复插入 BST
            }else{
                if value<tree_node.value{
                    ref_TreeNode =&mut tree_node.left;
                }else {
                    ref_TreeNode =&mut tree_node.right;
                }
            }
            
        }
        *ref_TreeNode =Some(Box::new(TreeNode::<T>::new(value)));
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut ref_TreeNode = &self.root;
        while let Some(tree_node) =ref_TreeNode{
            if tree_node.value == value {
                return true;
            }
            if value<tree_node.value{
                ref_TreeNode =&tree_node.left;
            }else{
                ref_TreeNode =&tree_node.right;
            }

        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if value<self.value{
            self.left = Some(Box::new(TreeNode::new(value)));
        }
        else if value>self.value{
            self.right = Some(Box::new(TreeNode::new(value)));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


