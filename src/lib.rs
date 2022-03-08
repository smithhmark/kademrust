mod kademlia;

use crate::kademlia as k;

pub struct Node {
    pub id: k::NodeID,
    pub table: k::RoutingTable,
}

impl Node {
    pub fn new(id: k::NodeID) -> Self {
        let table = k::RoutingTable {
            id,
            key_space: 1,
            kay: 5,
            hoods: vec![],
        };
        Node { id, table }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn build_node_simple() {
        let node = Node::new(1);
        assert_eq!(1, node.id);
    }
}
