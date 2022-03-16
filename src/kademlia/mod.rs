//#![allow(dead_code)]
//#![allow(unused_variables)]

//use std::cmp;

use log::debug;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

pub type NodeID = u128;
pub type Key = u128;

#[allow(dead_code)]
pub type Nonce = usize;
#[allow(dead_code)]
pub type Value = Vec<u8>;

// t.co/Qu1IUXfCms
pub type NodeAddress = IpAddr;
pub type NodePort = usize;

fn iddiff(a: &NodeID, b: &NodeID) -> NodeID {
    *a ^ *b
}

fn bucket_id(a: &NodeID, b: &NodeID, key_space: usize) -> usize {
    let distance = iddiff(a, b);
    let base_line = NodeID::BITS - key_space as u32;
    (NodeID::leading_zeros(distance) - base_line) as usize
}

fn constrain_bucket(bucket: usize, limit: usize) -> usize {
    if bucket >= limit {
        debug!("inserting into a bucket {} that doesn't exist yet", bucket);
        limit - 1
    } else {
        debug!("bucket is a 'near' neighborhood");
        bucket
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct NodeDescription {
    id: NodeID,
    address: NodeAddress,
    port: NodePort,
}

impl NodeDescription {
    fn _dummy(id: NodeID) -> NodeDescription {
        NodeDescription {
            id,
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5000,
        }
    }
}

impl Default for NodeDescription {
    fn default() -> Self {
        NodeDescription {
            id: 0,
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5000,
        }
    }
}

pub trait KademliaNode {
    fn find_node(key: Key);
    fn find_value(key: Key);
    fn store(key: Key, val: Value);
    fn ping(node: NodeID);
}

//pub struct Neighborhood { }
type Neighborhood = Vec<NodeDescription>;

#[derive(Debug, Default)]
pub struct VectorRoutingTable {
    pub id: NodeID,
    pub key_space: usize,
    pub kay: usize,
    pub hoods: Vec<Neighborhood>,
}

trait RTable {
    fn population(&self) -> usize;
    fn insert(&mut self, other: NodeDescription);
    fn lookup(&self, key: Key) -> Neighborhood;
    fn kay(&self) -> usize;
}

impl VectorRoutingTable {
    pub fn new(id: NodeID, key_space: usize, kay: usize) -> VectorRoutingTable {
        let mut hoods: Vec<Neighborhood> = Vec::with_capacity(key_space);
        hoods.push(vec![]);
        VectorRoutingTable {
            id,
            key_space,
            kay,
            hoods,
        }
    }
    fn _pop_by_hood(&self) -> Vec<usize> {
        self.hoods.iter().map(|h| h.len()).collect()
    }
}

impl RTable for VectorRoutingTable {
    fn kay(&self) -> usize {
        self.kay
    }

    fn population(&self) -> usize {
        self.hoods.iter().map(|n| n.len()).sum()
    }

    fn lookup(&self, key: Key) -> Neighborhood {
        let bucket = bucket_id(&self.id, &key, self.key_space);
        let start_at = constrain_bucket(bucket, self.hoods.len());
        debug!("starting at bucket:{}", start_at);
        self.hoods[start_at..]
            .iter()
            .flatten()
            .take(self.kay())
            .map(|x| *x)
            .collect()
    }

    fn insert(&mut self, other: NodeDescription) {
        let bucket = bucket_id(&self.id, &other.id, self.key_space);
        debug!("bucket:{}", bucket);
        let initial_len = self.hoods.len();
        let last_bucket = initial_len - 1;
        let insert_at = constrain_bucket(bucket, self.hoods.len());

        if self.hoods[insert_at].len() < self.kay() {
            debug!("plenty of room for the desc");
            self.hoods[insert_at].push(other);
        } else {
            debug!("No room");
            if insert_at == last_bucket {
                debug!("partitioning furthest hood:{:?}", self.hoods[insert_at]);
                let mut new_buckets: HashMap<usize, Neighborhood> = HashMap::new();
                for (k, v) in self.hoods[insert_at]
                    .iter()
                    .map(|desc| (bucket_id(&desc.id, &self.id, self.key_space), *desc))
                {
                    new_buckets.entry(k).or_insert_with(Vec::new).push(v);
                }
                //let split_into = new_buckets.len();
                debug!("\tsplit bucket into {}", new_buckets.len());
                let closest = new_buckets.keys().max();
                let furthest = new_buckets.keys().min();
                debug!("\tclosest bucket {}", closest.unwrap());
                debug!("\tfurthest bucket {}", furthest.unwrap());
                debug!("\tdealing with orig buck:{}", last_bucket);
                match new_buckets.get(&last_bucket) {
                    Some(content) => self.hoods[last_bucket] = content.to_vec(),
                    None => self.hoods[last_bucket] = Vec::with_capacity(self.kay()),
                }
                for buck_id in (initial_len)..=*closest.expect("should have split into buckets") {
                    debug!("\tdealing with buck:{}", buck_id);
                    match new_buckets.get(&buck_id) {
                        Some(content) => {
                            debug!("\t\thad data");
                            self.hoods.push(content.to_vec())
                        }
                        None => self.hoods.push(Vec::with_capacity(self.kay())),
                    }
                }
                //debug!("\n at_dist hood:{:?}", at_dist);
                //debug!("\n further hood:{:?}", farther);
                self.hoods[bucket].push(other);
            } else {
                debug!("No room, take a hike");
                // possibly maintain pool of replacements
            };
        };
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_table_initialization() {
        let table = VectorRoutingTable::new(0, 1, 2);
        let pop = table.population();
        assert_eq!(0, pop);

        let table = VectorRoutingTable::default();
        let pop = table.population();
        assert_eq!(0, pop);
    }

    #[test]
    fn test_table_population() {
        let mut table = VectorRoutingTable::default();
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
    fn test_table_insert() {
        setup();
        let key_space = 4;
        let kay = 2;
        let mut table = VectorRoutingTable::new(0, key_space, kay);
        assert_eq!(1, table.hoods.len());
        assert_eq!(0, table.hoods[0].len());
        assert_eq!(0, table.population());
        println!("sizes:{:?}", table._pop_by_hood());
        println!("first insert");
        table.insert(NodeDescription::_dummy(4));
        println!("sizes:{:?}", table._pop_by_hood());
        assert_eq!(1, table.hoods.len());
        assert_eq!(1, table.population());
        assert_eq!(1, table.hoods[0].len());
        println!("second insert");
        table.insert(NodeDescription::_dummy(3));
        println!("sizes:{:?}", table._pop_by_hood());
        assert_eq!(1, table.hoods.len());
        assert_eq!(2, table.population());
        assert_eq!(2, table.hoods[0].len());
        println!("third insert");
        table.insert(NodeDescription::_dummy(2));
        println!("sizes:{:#?}", table._pop_by_hood());
        assert_eq!(3, table.hoods.len());
        assert_eq!(3, table.population());
        assert_eq!(0, table.hoods[0].len());
        assert_eq!(1, table.hoods[1].len());
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

    #[test]
    fn test_lookup() {
        setup();
        let key_space = 4;
        let kay = 2;
        let mut table = VectorRoutingTable::new(0, key_space, kay);
        table.insert(NodeDescription::_dummy(1)); //dist = 1 => bucket: 3
        table.insert(NodeDescription::_dummy(2)); //dist = 2 => bucket: 3
        table.insert(NodeDescription::_dummy(3)); //dist = 3 => bucket: 2
        table.insert(NodeDescription::_dummy(4)); //dist = 4 => bucket: 1
        let hood = table.lookup(2);
        assert_eq!(
            hood,
            vec![NodeDescription::_dummy(2), NodeDescription::_dummy(3)]
        );
        let hood = table.lookup(1);
        assert_eq!(
            hood,
            vec![NodeDescription::_dummy(1), NodeDescription::_dummy(2)]
        );
    }
}
