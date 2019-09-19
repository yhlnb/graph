mod graph;
mod hashfunc;
use graph::Edges;
use graph::LabelMatrix;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Node {
    value: Vec<u8>,
    index: (usize, usize),
}
impl Node {
    pub fn new(v: Vec<u8>, col: usize, vertex: usize) -> Self {
        Node {
            value: v,
            index: (col, vertex),
        }
    }
    pub fn get_value(&self) -> Vec<u8> {
        self.value.to_vec()
    }
    pub fn get_index(&self) -> (usize, usize) {
        self.index
    }
}

fn main() {
    let (n, k) = (1000, 50);
    let mut v = vec![];
    let edge = Arc::new(Edges::chung(n));
    let gra = Arc::new(LabelMatrix::new(&edge, k, &[0]));  //有些问题
    for col in 1..k {
        let edge = edge.clone(); //这没法用指针。。。
        let gra = gra.clone();
        let child = thread::spawn(move || {
            let mut map = HashMap::new();
            for vertex in 0..n {
                let v2 = &edge.0[vertex];
                let value = &gra.0[col - 1][vertex];
                let node = Node::new(value.to_vec(), col - 1, vertex);
                let mut tail = vec![];
                for i in v2.iter() {
                    let value = &gra.0[col][*i];
                    let node_tail = Node::new(value.to_vec(), col, *i);
                    tail.push(node_tail);
                }
                map.insert(node, tail);
            }
            println!("{:?}", map);
        });
        v.push(child);
    }
    for child in v {
        child.join().unwrap();
    }
}
