use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

pub type VertexLabel = Vec<u8>;
#[derive(Clone,Debug)]
pub struct Edges(Vec<Vec<usize>>);
impl Edges {
pub fn b1(a: f64, r: f64) -> f64 {
        let b = r + 2.0 * a;
        b
}
pub fn hb(x: f64) -> f64 {
    //二元熵函数
        let y = -x * (x.log2()) - (1.0 - x) * ((1.0 - x).log2());
        y
}

pub fn Chung(n: usize, a: f64, r: f64) -> Self {
    let mut rng = rand::thread_rng();
    let mut map1:HashMap<usize, HashSet<usize> = HashMap::new() ;
    let b = b1(a, r); //生成a，b的（0，1）的随机数
    let k = (hb(a) + hb(b)) / (hb(a) - b * hb(a / b)); //取k临界值
    let d = k.ceil() as i64;
        for i in 0.. d * n {
            let j = rng.gen_range(0, d * n);
            if map1.get(&(i % n)) == None {
            let set: HashSet<(i64, i64)> = HashSet::new();
            map1.insert(i % n, set);
        }
        map1.get_mut(&(i % n)).unwrap().insert(j % n);    
        }
        let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 0..n {
            edges.push(map1.get(&i).unwrap().clone());
        }
        Edges(edges)
}

pub fn get_parent(&self,vertex:usize) -> Vec<usize> {
        let mut parents = vec![]; 
        let mut n_parents = 0;
        for (source_index, edges) in self.0.iter().enumerate() {
            if edges.contains(&vertex) {
                parents.push(source_index);
                n_parents += 1;
                if n_parents == d {
                    break;
                }
            }
        }
        parents
    }

    fn n(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub struct LabelMatrix(pub Vec<Vec<VertexLabel>>);
impl LabelMatrix {
    pub fn new(edges:&Edges,k : usize , nonce: &[u8]) -> Self {
        let n = edges.n();
        let mut lable_matrix : Vec<Vec<VertexLabel>> = vec![vec![];k] ;

        
//没写完
    }
}