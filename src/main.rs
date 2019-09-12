mod graph;
mod hashfunc;
use graph::Edges;
use graph::LabelMatrix;

#[derive(Debug)]
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
    let n = 100;
    let k = 50;
    let edge = Edges::chung(n);
    let gra = LabelMatrix::new(&edge, k, &[0]);
    for col in 0..k {
        for vertex in 0..n {
            let v1 = &gra.0[col][vertex];
            let node = Node::new(v1.to_vec(), col, vertex);
            println!("{:?}", node);
        }
    }
}
