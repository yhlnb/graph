mod graph;
mod hashfunc;
use graph::Edges;
use graph::LabelMatrix;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Node {
    value: String,
    index: (usize, usize),
}
impl Node {
    pub fn new(v: String, col: usize, vertex: usize) -> Self {
        Node {
            value: v,
            index: (col, vertex),
        }
    }
    pub fn get_index(&self) -> (usize, usize) {
        self.index
    }
}

fn main() {
    let (n, k) = (5, 5);
    let edge = Edges::chung(n);
    let gra = Arc::new(LabelMatrix::new(&edge, k, &[1]));
    let (tx,rx) = channel();
    for col in 1..k {
        let tx = tx.clone();
        let mut map = HashMap::new();
        let edge = edge.clone(); 
        let gra = gra.clone();
        thread::spawn(move || {
            for vertex in 0..n {
                let v2 = &edge.0[vertex];
                let value = &gra.0[col - 1][vertex];
                let node = Node::new(value.to_string(), col - 1, vertex);
                let mut tail = vec![];
                for i in v2.iter() {
                    let value = &gra.0[col][*i];
                    let node_tail = Node::new(value.to_string(), col, *i);
                    tail.push(node_tail);
                }
                map.insert(node, tail);
            }
            tx.send(map).unwrap();
        });
    }
    for _ in 1..k {
        let j = rx.recv().unwrap();
        println!("{:?}", j);
    }
}