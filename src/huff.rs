use std::cmp::Ordering;

#[allow(dead_code)]
pub enum HuffmanNode {
    Leaf {
        weight: usize,
        char: u8,
    },
    Internal {
        weight: usize,
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
}

impl HuffmanNode {
    fn weight(&self) -> usize {
        match self {
            HuffmanNode::Leaf { weight, .. } => *weight,
            HuffmanNode::Internal { weight, .. } => *weight,
        }
    }

    #[allow(dead_code)]
    fn is_leaf(&self) -> bool {
        match self {
            HuffmanNode::Leaf { .. } => false,
            HuffmanNode::Internal { .. } => true,
        }
    }
}

// impls to enable priority queue via `BinaryHeap`
impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl Eq for HuffmanNode {}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight().cmp(&self.weight())
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
