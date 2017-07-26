#[cfg(test)]

mod merklet {

use std::rc::{Weak, Rc};

extern crate openssl;
use self::openssl::hash::{DigestBytes, MessageDigest, hash2};

extern crate hex;
use self::hex::{FromHex, ToHex};

trait Hash2{
    fn hash2(&self) -> DigestBytes;
}

enum MerkleChild<T: Hash2>{
    Branch(Rc<MerkleBranch<T>>),
    Leaf(Rc<T>),
}

struct MerkleBranch<T: Hash2> {
    left: MerkleNode<T>,
    right: MerkleNode<T>,
}

struct MerkleNode<T: Hash2>{
    hash: DigestBytes,
    //parent: Weak<MerkleNode<T>>,
    next: MerkleChild<T>,
}

impl<T: Hash2> Hash2 for MerkleNode<T> {
    fn hash2(&self) -> DigestBytes {
        // Match merkle child left and right. If they are nodes concat the 
        // existing hash digests and hash that.If they are leaves hash the
        // data of the leaf.
        match self.next {
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

} //end mod merklet

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(false);
    }

}
