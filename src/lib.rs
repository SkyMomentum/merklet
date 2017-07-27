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



mod tests {
use super::*;
use std::rc::{Weak, Rc};
extern crate openssl;
use self::openssl::hash::{DigestBytes, MessageDigest, hash2};

#[test]
fn it_works() {
    assert!(false);
}

#[test]
fn basics() {
    struct TestA {
        data: String,
    }

    impl TestA {
        fn new(sin: & str ) -> TestA {
            let out = TestA {
                data: sin.to_string(),
            };
            out
        }
    }

    impl Hash2 for TestA {
        fn hash2(&self) -> DigestBytes {
            hash2(MessageDigest::sha256(), self.data.as_bytes()).unwrap()
        }
    }

    let ta = TestA::new("A");
    let hasha = ta.hash2();
    let a = Rc::new(ta);

    let tb = TestA::new("B");
    let hashb = tb.hash2();
    let b = Rc::new(tb);

    let mc_leafa = MerkleChild::Leaf(a);
    let mc_leafb = MerkleChild::Leaf(b);

    let mn_a = MerkleNode {
        hash: hasha,
        next: mc_leafa,
    };
    let mn_b = MerkleNode {
        hash: hashb,
        next: mc_leafb,
    };

    let mb_branch = MerkleBranch {
        left: mn_a,
        right: mn_b,
    };
    let rc_branch = Rc::new(mb_branch);
    let mc_branch = MerkleChild::Branch(rc_branch);
    let mut mc_root: MerkleNode<TestA>;
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

} //end mod test
} //end mod merklet
