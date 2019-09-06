mod hashfunc;
mod graph;
use graph::Edges;
use graph::LabelMatrix;



fn main() {
    let edge = Edges::Chung(5);
    let gra = LabelMatrix::new(&edge,2,&[0]);

    println!("{:?}",gra);
}