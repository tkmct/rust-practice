use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub struct MerkleTree {}

#[derive(Debug, Clone)]
struct Node {
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
fn construct_tree<T: Hash>(values: Vec<T>) -> Node {
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

    nodes[0].clone()
}

// Construct merkle proof for given index
fn construct_merkle_proof(i: u64) -> Vec<u64> {
    vec![]
}

fn verify_merkle_proof<T: Hash>(root: Node, value: T, inclusion_proof: Vec<u64>) -> bool {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let root_hash = inclusion_proof
        .iter()
        .fold(hasher.finish(), |a, b| combine_hash(a, *b));
    root_hash == root.hash_value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let values = vec![1, 3, 8, 2, 4];
        let node = construct_tree(values);
        println!("{:?}", node);
        assert_eq!(1, 2);
    }
}
