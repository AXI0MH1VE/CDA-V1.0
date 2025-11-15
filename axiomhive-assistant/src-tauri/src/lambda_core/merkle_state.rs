//! Merkle Tree Implementation for Constitutional State Tracking
//! Ensures reproducibility and immutability of CDA-v1.0 configurations

use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Merkle tree node
#[derive(Debug, Clone)]
struct Node {
    hash: [u8; 32],
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: Option<String>,
}

/// Merkle tree for tracking constitutional states
#[derive(Debug)]
pub struct MerkleTree {
    root: Option<Node>,
    leaves: HashMap<String, Node>,
}

impl MerkleTree {
    /// Create new empty Merkle tree
    pub fn new() -> Self {
        Self {
            root: None,
            leaves: HashMap::new(),
        }
    }

    /// Add a constitutional axiom to the tree
    pub fn add_axiom(&mut self, axiom_id: &str, content: &str) {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash = hasher.finalize();

        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&hash);

        let node = Node {
            hash: hash_array,
            left: None,
            right: None,
            data: Some(content.to_string()),
        };

        self.leaves.insert(axiom_id.to_string(), node);
        self.rebuild_root();
    }

    /// Get root hash for state verification
    pub fn get_root_hash(&self) -> Option<String> {
        self.root.as_ref().map(|node| hex::encode(node.hash))
    }

    /// Verify an axiom's inclusion in the tree
    pub fn verify_axiom(&self, axiom_id: &str, content: &str) -> bool {
        if let Some(node) = self.leaves.get(axiom_id) {
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            let hash = hasher.finalize();

            let mut hash_array = [0u8; 32];
            hash_array.copy_from_slice(&hash);

            node.hash == hash_array
        } else {
            false
        }
    }

    /// Get current state hash for constitutional auditability
    pub fn get_state_hash(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.get_root_hash()
            .ok_or_else(|| "No constitutional state available".into())
    }

    /// Rebuild the tree after adding leaves
    fn rebuild_root(&mut self) {
        let leaves: Vec<_> = self.leaves.values().collect();
        if leaves.is_empty() {
            self.root = None;
            return;
        }

        if leaves.len() == 1 {
            self.root = Some(leaves[0].clone());
            return;
        }

        // Build tree by pairing leaves
        let mut current_level: Vec<Node> = leaves.iter().cloned().map(|node| node.clone()).collect();

        while current_level.len() > 1 {
            let mut next_level: Vec<Node> = Vec::new();

            for chunk in current_level.chunks(2) {
                match chunk {
                    [left, right] => {
                        let combined_hash = MerkleTree::combine_hashes(&left.hash, &right.hash);
                        let parent = Node {
                            hash: combined_hash,
                            left: Some(Box::new(left.clone())),
                            right: Some(Box::new(right.clone())),
                            data: None,
                        };
                        next_level.push(parent);
                    },
                    [single] => {
                        next_level.push(single.clone());
                    },
                    _ => unreachable!(),
                }
            }

            current_level = next_level;
        }

        self.root = Some(current_level[0].clone());
    }

    /// Combine two hashes
    fn combine_hashes(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        let hash = hasher.finalize();

        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&hash);
        hash_array
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}
