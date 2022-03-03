pub type NodeID = usize;

pub type NodeAddress = String;

fn iddiff(a: &NodeID, b: &NodeID) -> usize{
    (*a^*b).count_ones() as usize
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct NodeDescription {
    id: NodeID,
    address: NodeAddress,
}

//pub struct Neighborhood { }
type Neighborhood = Vec<NodeDescription>;

#[derive(Debug, Default)]
pub struct RoutingTable {
    id: NodeID,
    net_size: usize,
    key_space: usize,
    hoods: Vec<Neighborhood>,
}

trait RTable {
    fn population(&self) -> usize;
    fn insert(&mut self, other: NodeDescription);
}

impl RoutingTable {
    fn new(id: NodeID, net_size: usize, key_space: usize) -> RoutingTable {
        let hoods: Vec<Neighborhood> = Vec::with_capacity(key_space);
        RoutingTable {
            id,
            net_size,
            key_space,
            hoods,
        }
    }
}

impl RTable for RoutingTable {
    fn population(&self) -> usize {
        self.hoods.iter().map(|n| n.len()).sum()
    }

    fn insert(&mut self, other: NodeDescription) {
        // calc difference
        let diff = iddiff(&self.id, &other.id);
        println!("indexing at distance: {}", diff);
    }
}

pub struct Node {
    id: NodeID,
    table: RoutingTable,
}

impl Node {
    fn new(id: NodeID) -> Self {
        let table = RoutingTable {
            id,
            net_size: 1,
            key_space: 1,
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
        let mut table = RoutingTable::new(0, 1, 2);
        assert_eq!(0, table.population());
        table.insert(desc);
        assert_eq!(0, table.population());
    }

    #[test]
    fn test_iddiff(){
        assert_eq!(iddiff(&0,&0), 0);
        assert_eq!(iddiff(&1,&1), 0);
        assert_eq!(iddiff(&42,&42), 0);

        assert_eq!(iddiff(&0,&1), 1);
        assert_eq!(iddiff(&1,&2), 2);
    }
}
