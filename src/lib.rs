#[cfg(test)]

mod merklet {

use std::rc::{Weak, Rc};

extern crate openssl;
use self::openssl::hash::{DigestBytes, MessageDigest, hash2};

extern crate hex;
use self::hex::{FromHex, ToHex};

pub trait Hash2{
    fn hash2(&self) -> DigestBytes;
}

pub enum MerkleChild<T: Hash2>{
    Branch(Rc<MerkleBranch<T>>),
    Leaf(Rc<T>),
}

impl<T: Hash2> Hash2 for MerkleChild<T> {
    fn hash2(&self) -> DigestBytes {
        // Match merkle child left and right. If they are nodes concat the 
        // existing hash digests and hash that.If they are leaves hash the
        // data of the leaf.
        match *self {
            MerkleChild::Branch(ref branch) => {
                let mut concatenated_hash: Vec<u8> = Vec::new();
                let mut left_digest: Vec<u8> = branch.left.hash.to_vec();
                let mut right_digest: Vec<u8> = branch.right.hash.to_vec();

                concatenated_hash.append(&mut left_digest);
                concatenated_hash.append(&mut right_digest);

                hash2(MessageDigest::sha256(), concatenated_hash.as_slice()).unwrap()
            }
            MerkleChild::Leaf(ref leaf) => {
                //Placeholder
                //hash2(MessageDigest::sha256(), &Vec::from_hex("616263").unwrap()).unwrap()
                leaf.hash2()
            }
        }
    }
}

pub struct MerkleBranch<T: Hash2> {
    left: MerkleNode<T>,
    right: MerkleNode<T>,
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

pub struct MerkleNode<T: Hash2>{
    hash: DigestBytes,
    //parent: Weak<MerkleNode<T>>,
    next: MerkleChild<T>,
}

impl<T: Hash2> Hash2 for MerkleNode<T> {
    fn hash2(&self) -> DigestBytes {
        self.next.hash2()
    }
}

/*
fn new_merkle_tree<T: Hash2>(leaves: &[T]) -> MerkleNode<T> {
    // Take all leaves generate hash digest, for each pair generate has
    // for the merklenode.

    let leaf_iter = leaves.iter();
    
    for leaf in leaf_iter {
        // Make a new node, add the leaf to .next member as a MerkleChild::Leaf
        // and the hash digest to .hash then add to hash for next step
    }

    //call build_merkle_branches to form tree for the newly created nodes
}

fn build_merkle_branches<T: Hash2>(nodes: &[MerkleNode<T>]) -> MerkleNode<T>{
    // For each pair of nodes make a new node for next level and hash the branch.
    let pair_iter = leaves.chunks(2);
    for pairs in pair_iter {
    }
}*/

fn make_branch_node<T: Hash2>(left_node: MerkleNode<T>,
                                right_node: MerkleNode<T>) -> MerkleNode<T> {
    let branch = MerkleBranch {
        left: left_node,
        right: right_node,
    };
    let ret_node = MerkleNode {
        hash: branch.hash2(),
        next: MerkleChild::Branch(Rc::new(branch)),
    };
    ret_node
}

fn make_leaf_node<T: Hash2>(leaf: T) -> MerkleNode<T> {
    let leaf_node = MerkleNode {
        hash: leaf.hash2(),
        next: MerkleChild::Leaf(Rc::new(leaf)),
    };
    leaf_node
}

mod tests {
use super::*;
use std::rc::{Weak, Rc};
extern crate openssl;
use self::openssl::hash::{DigestBytes, MessageDigest, hash2};

// --Test Utilities--
// Dummy data struct for leaf
struct TestData {
    data: String,
}

impl TestData {
    fn new(sin: & str ) -> TestData {
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
fn make_test_leaf_node(sdata_in: &str) -> MerkleNode<TestData> {
    let data = TestData::new(sdata_in);
    let leaf_node = MerkleNode{
        hash: data.hash2(),
        next: MerkleChild::Leaf(Rc::new(data)),
    };
    leaf_node
}

#[test]
fn making_leaf() {
    let t_data = TestData::new("A");
    let t_leaf =  make_leaf_node(t_data);
    assert_eq!(*hash2(MessageDigest::sha256(), "A".as_bytes()).unwrap(), *t_leaf.hash2());
}

#[test]
fn making_branch() {
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
}

#[test]
fn basics() {

    let mn_a = make_test_leaf_node("A");
    let mn_b = make_test_leaf_node("B");

    let mb_branch = MerkleBranch {
        left: mn_a,
        right: mn_b,
    };
    let rc_branch = Rc::new(mb_branch);
    let mc_branch = MerkleChild::Branch(rc_branch);
    let mut mc_root: MerkleNode<TestData>;
    mc_root.hash = mc_branch.hash2();
    mc_root.next = mc_branch;
    
}

#[test]
fn left_right_not_equal() {
}

#[test]
fn simple_tree() {
}

#[test]
fn two_simple_trees_cmp_roots() {
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
