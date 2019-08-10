use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub struct MerkleTree {}

type Error = ();

#[derive(Debug, Clone)]
pub struct Node {
    hash_value: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Default for Node {
    fn default() -> Self {
        let mut hasher = DefaultHasher::new();
        0.hash(&mut hasher);

        Node {
            hash_value: hasher.finish(),
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct RootNode {
    hash_value: u64,
    top_node: Box<Node>,
    depth: usize,
}

// hash two values
fn combine_hash(a: u64, b: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(a);
    hasher.write_u64(b);
    hasher.finish()
}

// construct node from value
fn construct_leaf_node<T: Hash>(value: T) -> Node {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    Node {
        hash_value: hasher.finish(),
        left: None,
        right: None,
    }
}

// Construct a node from left node and right node
fn construct_node(left: Node, right: Node) -> Node {
    Node {
        hash_value: combine_hash(left.hash_value, right.hash_value),
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    }
}

// Construct merkle tree with given Vec<T>
pub fn construct_tree<T: Hash>(values: Vec<T>) -> RootNode {
    let mut i = 1;
    let mut j = 0;
    let len = values.len();
    while i <= len {
        i *= 2;
        j += 1;
    }

    let mut empty_vecs = vec![Node::default(); i - len];

    let mut nodes = values
        .into_iter()
        .map(construct_leaf_node)
        .collect::<Vec<Node>>();
    nodes.append(&mut empty_vecs);
    for _ in 0..j {
        let mut parents = vec![];
        for k in 0..nodes.len() / 2 {
            parents.push(construct_node(
                nodes[2 * k].clone(),
                nodes[2 * k + 1].clone(),
            ));
        }
        nodes = parents;
    }

    let top_node = nodes[0].clone();
    RootNode {
        hash_value: top_node.hash_value,
        top_node: Box::new(top_node),
        depth: j,
    }
}

// Construct merkle proof for given index
pub fn construct_merkle_proof(merkle_tree: &RootNode, i: usize) -> Result<Vec<u64>, Error> {
    let mut i = i;
    let mut merkle_proof = vec![];
    let mut d = 2_usize.pow(merkle_tree.depth as u32);
    let mut node = merkle_tree.top_node.clone();

    if i >= d {
        ()
    }

    while d != 1 {
        merkle_proof.push(if i < d / 2 {
            let hash = node.right.unwrap().hash_value;
            node = node.left.unwrap();
            hash
        } else {
            i = i - d / 2;
            let hash = node.left.unwrap().hash_value;
            node = node.right.unwrap();
            hash
        });

        d /= 2;
    }

    merkle_proof.reverse();
    Ok(merkle_proof)
}

pub fn verify_merkle_proof<T: Hash>(
    root: RootNode,
    value: T,
    i: usize,
    inclusion_proof: Vec<u64>,
) -> bool {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let mut hash = hasher.finish();
    let mut i = i;

    for p in inclusion_proof {
        hash = if i % 2 == 0 {
            combine_hash(hash, p)
        } else {
            combine_hash(p, hash)
        };
        i /= 2;
    }

    hash == root.hash_value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let values = vec![1, 3, 8];
        let node = construct_tree(values);
        let merkle_proof = construct_merkle_proof(&node, 2).unwrap();
        assert!(verify_merkle_proof(node, 8, 2, merkle_proof));
    }
}
