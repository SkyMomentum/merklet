
pub mod merklet {
    ///! A Toy Merkle Tree Type.
    ///!
    ///! Create Merkle trees from arbitrary types which impliment Hash2(Temporary).
    use std::rc::Rc;
    extern crate openssl;
    use self::openssl::hash::{DigestBytes, MessageDigest, hash2};

    /// Temporary trait to hash with sha256 from Openssl.
    pub trait Hash2 {
        fn hash2(&self) -> DigestBytes;
    }

    /// This is the `next` field in a MerkleNode. Either a branch with two children or a Rc
    /// reference to the leaf type T
    #[derive(Clone)]
    pub enum MerkleChild<T: Hash2> {
        Branch(MerkleBranch<T>),
        Leaf(Rc<T>),
    }

    /// Match merkle child left and right. If they are nodes concat the
    /// existing hashes digests and hash that.If they are leaves hashes the
    /// data of the leaf.
    impl<T: Hash2> Hash2 for MerkleChild<T> {
        fn hash2(&self) -> DigestBytes {
            match *self {
                MerkleChild::Branch(ref branch) => {
                    let mut concatenated_hash: Vec<u8> = Vec::new();
                    let mut left_digest: Vec<u8> = branch.left.hash.to_vec();
                    let mut right_digest: Vec<u8> = branch.right.hash.to_vec();
                    concatenated_hash.append(&mut left_digest);
                    concatenated_hash.append(&mut right_digest);
                    hash2(MessageDigest::sha256(), concatenated_hash.as_slice()).unwrap()
                }
                MerkleChild::Leaf(ref leaf) => leaf.hash2(),
            }
        }
    }

    /// Simple binary tree banch for this merkle tree.
    #[derive(Clone)]
    pub struct MerkleBranch<T: Hash2> {
        left: Rc<MerkleNode<T>>,
        right: Rc<MerkleNode<T>>,
    }

    impl<T: Hash2> Hash2 for MerkleBranch<T> {
        fn hash2(&self) -> DigestBytes {
            let mut concatenated_hash: Vec<u8> = Vec::new();
            let mut left_digest: Vec<u8> = self.left.hash.to_vec();
            let mut right_digest: Vec<u8> = self.right.hash.to_vec();
            concatenated_hash.append(&mut left_digest);
            concatenated_hash.append(&mut right_digest);
            hash2(MessageDigest::sha256(), concatenated_hash.as_slice()).unwrap()
        }
    }

    /// A node in a Merkle tree. The field 'next' holds a MerkleChild which represents
    /// either a branch or a leaf.
    #[derive(Clone)]
    pub struct MerkleNode<T: Hash2> {
        hash: DigestBytes,
        next: MerkleChild<T>,
    }

    impl<T: Hash2> Hash2 for MerkleNode<T> {
        fn hash2(&self) -> DigestBytes {
            self.next.hash2()
        }
    }

    impl<T: Hash2> MerkleNode<T> {
        fn is_leaf(&self) -> bool {
            match self.next {
                MerkleChild::Leaf(_) => true,
                MerkleChild::Branch(_) => false,
            }
        }

        fn is_branch(&self) -> bool {
            match self.next {
                MerkleChild::Leaf(_) => false,
                MerkleChild::Branch(_) => true,
            }
        }


        fn get_left_child(&self) -> Option<Rc<MerkleNode<T>>> {
            match self.next {
                MerkleChild::Branch(ref branch) => Some(branch.left.clone()),
                MerkleChild::Leaf(_) => None,
            }
        }

        fn get_right_child(&self) -> Option<Rc<MerkleNode<T>>> {
            match self.next {
                MerkleChild::Branch(ref branch) => Some(branch.right.clone()),
                MerkleChild::Leaf(_) => None,
            }
        }

        fn get_leaf_data(&self) -> Option<Rc<T>> {
            match self.next {
                MerkleChild::Branch(_) => None,
                MerkleChild::Leaf(ref leaf) => Some(leaf.clone()),
            }
        }
    }

    /// Function to build a fresh Merkle tree.
    // TODO: Currently NOT COMPLETE, will return the root node.
    /// # Arguments
    /// *`leaves` - A slice of whatever will become the leaves.
    pub fn new_merkle_tree<T: Hash2 + Clone>(leaves: &[T]) -> Rc<MerkleNode<T>> {
        let leaf_iter = leaves.iter();
        let mut leaf_nodes: Vec<Rc<MerkleNode<T>>> = Vec::new();
        for leaf in leaf_iter {
            let rcleaf = Rc::new(make_leaf_node(leaf.clone()));
            leaf_nodes.push(rcleaf);
        }
        // Build tree and return root node.
        build_merkle_branches(&leaf_nodes)
    }

    /// Utility function to recursively build branches after the first round of node
    /// building for the leaves.
    /// # Arguments
    /// `nodes` - A vector of Rc<> to MerkleNodes.
    fn build_merkle_branches<T: Hash2 + Clone>(
        nodes: &Vec<Rc<MerkleNode<T>>>,
    ) -> Rc<MerkleNode<T>> {
        // For each pair of nodes make a new node for next level and hash the branch.
        let pair_iter = nodes.chunks(2);
        let mut branch_level: Vec<Rc<MerkleNode<T>>> = Vec::new();
        for pairs in pair_iter {
            if pairs.len() == 2 {
                let left_node: Rc<MerkleNode<T>> = pairs.first().unwrap().clone();
                //TODO: Handle the Option<> below.
                let right_node: Rc<MerkleNode<T>> = pairs
                    .last()
                    .expect("ERROR: There should be two nodes, but there are not.")
                    .clone();
                branch_level.push(make_branch_node(left_node, right_node));
            } else {
                let only_left: Rc<MerkleNode<T>> = pairs.first().unwrap().clone();
                let only_right: Rc<MerkleNode<T>> = pairs.first().unwrap().clone();
                branch_level.push(make_branch_node(only_left, only_right));
            }
        }
        let ret: Rc<MerkleNode<T>>;
        if branch_level.len() > 1 {
            ret = build_merkle_branches(&branch_level);
        } else {
            //TODO: Clone unneeded? Return the Rc<MerkleNode>.
            ret = branch_level[0].clone();
        }
        ret
    }

    /// Utility function that returns a branch node.
    /// # Arguments
    /// `left_node` - Rc<> of a MerkleNode.
    /// `right_node` - Rc<> of a MerkleNode.
    fn make_branch_node<T: Hash2>(
        left_node: Rc<MerkleNode<T>>,
        right_node: Rc<MerkleNode<T>>,
    ) -> Rc<MerkleNode<T>> {
        let branch = MerkleBranch {
            left: left_node,
            right: right_node,
        };
        let ret_node = MerkleNode {
            hash: branch.hash2(),
            next: MerkleChild::Branch(branch),
        };
        Rc::new(ret_node)
    }

    /// Utility function that returns a leaf node.
    /// # Arguments
    /// `leaf` - The item that is to be the leaf of the tree.
    fn make_leaf_node<T: Hash2>(leaf: T) -> MerkleNode<T> {
        let leaf_node = MerkleNode {
            hash: leaf.hash2(),
            next: MerkleChild::Leaf(Rc::new(leaf)),
        };
        leaf_node
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::rc::Rc;
        use std::borrow::Borrow;
        use std::ops::Deref;
        extern crate openssl;
        use self::openssl::hash::{DigestBytes, MessageDigest, hash2};
        //extern crate hex;
        //use self::hex::{FromHex, ToHex};

        // --Test Utilities--
        // Dummy data struct for leaf
        #[derive(Clone)]
        struct TestData {
            data: String,
        }

        impl TestData {
            fn new(sin: &str) -> TestData {
                let out = TestData {
                    data: sin.to_string(),
                };
                out
            }
        }

        impl Hash2 for TestData {
            fn hash2(&self) -> DigestBytes {
                hash2(MessageDigest::sha256(), self.data.as_bytes()).unwrap()
            }
        }

        //Function for building dummy leaf
        fn make_test_leaf_node(sdata_in: &str) -> Rc<MerkleNode<TestData>> {
            let data = TestData::new(sdata_in);
            let leaf_node = MerkleNode {
                hash: data.hash2(),
                next: MerkleChild::Leaf(Rc::new(data)),
            };
            Rc::new(leaf_node)
        }

        #[test]
        fn making_leaf() {
            let t_data = TestData::new("A");
            let t_leaf = make_leaf_node(t_data);
            assert_eq!(
                *hash2(MessageDigest::sha256(), "A".as_bytes()).unwrap(),
                *t_leaf.hash2()
            );
            assert!(t_leaf.is_leaf());
        }

        #[test]
        fn making_a_branch() {
            let leafa = make_test_leaf_node("A");
            let leafb = make_test_leaf_node("B");

            let mut tmp: Vec<u8> = Vec::new();
            let mut la_digest: Vec<u8> = leafa.hash.to_vec();
            let mut lb_digest: Vec<u8> = leafb.hash.to_vec();
            tmp.append(&mut la_digest);
            tmp.append(&mut lb_digest);
            let expected = hash2(MessageDigest::sha256(), tmp.as_slice()).unwrap();

            let br = make_branch_node(leafa, leafb);
            assert_eq!(*expected, *br.hash2());
            assert!(br.is_branch());
        }

        #[test]
        fn test_build_merkle_branches() {
            let branch_ab = make_branch_node(make_test_leaf_node("A"), make_test_leaf_node("B"));
            let branch_ba = make_branch_node(make_test_leaf_node("B"), make_test_leaf_node("A"));
            let test_vec = vec![branch_ab, branch_ba];
            let test_branch = build_merkle_branches(&test_vec);

            // Traverse the tree, fail if we get the wrong child type. Match the left most leaf
            // node it should have data "A"
            match test_branch.next {
                MerkleChild::Branch(ref branch) => {
                    match branch.deref().borrow().deref().left.next {
                        MerkleChild::Branch(ref b2) => {
                            match b2.deref().borrow().deref().left.next {
                                MerkleChild::Branch(_) => {
                                    assert!(false);
                                }
                                MerkleChild::Leaf(ref l3) => {
                                    assert_eq!(l3.deref().borrow().data, "A");
                                }
                            }
                        }
                        MerkleChild::Leaf(_) => {
                            assert!(false);
                        }
                    }
                }
                MerkleChild::Leaf(_) => {
                    assert!(false);
                }
            }
        }

        #[test]
        fn lr_rl_not_equal() {
            let branch_ab = make_branch_node(make_test_leaf_node("A"), make_test_leaf_node("B"));
            let branch_ba = make_branch_node(make_test_leaf_node("B"), make_test_leaf_node("A"));
            assert_ne!(*branch_ab.hash, *branch_ba.hash);
        }

        #[test]
        fn two_simple_trees_cmp_roots() {}

        #[test]
        fn merklenode_child_util_funcs() {
            //Test is_leaf()
            let test_a = make_test_leaf_node("A");
            assert!(test_a.is_leaf());
            //Test is_branch()
            let test_branch = make_branch_node(make_test_leaf_node("C"), make_test_leaf_node("D"));
            assert!(test_branch.is_branch());
            //Test get_left_child()
            match test_branch.get_left_child().unwrap().next {
                MerkleChild::Leaf(ref l) => {
                    assert_eq!(l.data, "C");
                }
                MerkleChild::Branch(_) => {
                    assert!(false);
                }
            }
            //Test get_right_child()
            match test_branch.get_right_child().unwrap().next {
                MerkleChild::Leaf(ref l) => {
                    assert_eq!(l.data, "D");
                }
                MerkleChild::Branch(_) => {
                    assert!(false);
                }
            }
            //Test get_leaf_data()
            assert_eq!(test_a.get_leaf_data().unwrap().data, "A");
        }

        #[test]
        fn library_ready_for_any_use() {
            println!("------------------  HALT  ------------------");
            println!("        Proceed no further. Dangerous mutants ahead!");
            println!("        --------------------------------------------");
            assert!(false);
        }
    } //end mod test
} //end mod merklet
