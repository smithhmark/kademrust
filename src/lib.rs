pub type NodeID = u128;
pub type Key = usize;
pub type Nonce = usize;
pub type Value = Vec<u8>;

// t.co/Qu1IUXfCms
pub type NodeAddress = String;
pub type NodePort = usize;

fn iddiff(a: &NodeID, b: &NodeID) -> NodeID {
    *a ^ *b
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct NodeDescription {
    id: NodeID,
    address: NodeAddress,
    port: NodePort,
}

//pub struct Neighborhood { }
type Neighborhood = Vec<NodeDescription>;

#[derive(Debug, Default)]
pub struct RoutingTable {
    id: NodeID,
    key_space: usize,
    kay: usize,
    hoods: Vec<Neighborhood>,
}

trait RTable {
    fn population(&self) -> usize;
    fn insert(&mut self, other: NodeDescription);
    fn kay(&self) -> usize;
}

impl RoutingTable {
    fn new(id: NodeID, key_space: usize, kay: usize) -> RoutingTable {
        let mut hoods: Vec<Neighborhood> = Vec::with_capacity(key_space);
        for _i in 0..key_space {
            hoods.push(vec![]);
        }
        RoutingTable {
            id,
            key_space,
            kay,
            hoods,
        }
    }
}

impl RTable for RoutingTable {
    fn kay(&self) -> usize {
        self.kay
    }
    fn population(&self) -> usize {
        self.hoods.iter().map(|n| n.len()).sum()
    }

    fn insert(&mut self, other: NodeDescription) {
        let diff = iddiff(&self.id, &other.id);
        if diff > self.hoods.len() as NodeID{
            let insert_at = self.hoods.len()-1;
            self.hoods[insert_at].push(other);
            
        } else {
            self.hoods[diff as usize].push(other);
        }
    }
}

pub trait KademliaNode {
    fn find_node(key: Key);
    fn find_value(key: Key);
    fn store(key: Key, val: Value);
    fn ping(node: NodeID);
}

pub struct Node {
    pub id: NodeID,
    pub table: RoutingTable,
}

impl Node {
    fn new(id: NodeID) -> Self {
        let table = RoutingTable {
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

    #[test]
    fn test_table_initialization() {
        let table = RoutingTable::new(0, 1, 2);
        let pop = table.population();
        assert_eq!(0, pop);

        let table = RoutingTable::default();
        let pop = table.population();
        assert_eq!(0, pop);
    }

    #[test]
    fn test_table_population() {
        let mut table = RoutingTable::default();
        assert_eq!(0, table.population());
        assert_eq!(0, table.hoods.len());
        table.hoods.push(vec![]);
        assert_eq!(1, table.hoods.len());
        assert_eq!(0, table.population());
        assert_eq!(0, table.hoods[0].len());
        table.hoods[0].push(NodeDescription::default());
        assert_eq!(1, table.population());
    }

    #[test]
    fn test_table_index() {
        let desc = NodeDescription::default();
        let mut table = RoutingTable::new(1, 2, 5);
        assert_eq!(0, table.population());
        table.insert(desc);
        assert_eq!(1, table.population());
    }

    #[test]
    fn test_iddiff() {
        assert_eq!(iddiff(&0, &0), 0);
        assert_eq!(iddiff(&1, &1), 0);
        assert_eq!(iddiff(&42, &42), 0);

        assert_eq!(iddiff(&0, &1), 1);
        assert_eq!(iddiff(&2, &0), 2);
        assert_eq!(iddiff(&1, &2), 3);
    }
}
