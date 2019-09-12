
use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;
use crate::hashfunc::Hasher;

pub type VertexLabel = Vec<u8>;
pub const IN_DEGREE: usize = 2;

#[derive(Clone,Debug)]
pub struct Edges(Vec<Vec<usize>>);
impl Edges {

pub fn chung(n: usize) -> Self {
    let mut rng = rand::thread_rng();
    let mut map1: HashMap<usize,HashSet<usize>> = HashMap::new() ;
        for i in 0.. n * IN_DEGREE {
            let j = rng.gen_range(0, IN_DEGREE * n);
            if map1.get(&(i % n)) == None {
            let set: HashSet<usize> = HashSet::new();
            map1.insert(i % n, set);
        }
        map1.get_mut(&(i % n)).unwrap().insert(j % n);    
        }
        let mut edges: Vec<Vec<usize>> = vec![];
        for i in 0..n {
            let s1 = map1.get(&i).unwrap();
            let mut v1 :Vec<usize> = vec![];
            for j in s1.iter() {
                v1.push(*j);
            }
            edges.push(v1);
        }
        Edges(edges)
}

pub fn get_parents(&self,vertex:usize) -> Vec<usize> {
        let mut parents = vec![]; 
        let mut n_parents = 0;
        for (source_index, edges) in self.0.iter().enumerate() {
            if edges.contains(&vertex) {
                parents.push(source_index);
                n_parents += 1;
                if n_parents == IN_DEGREE {
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
    pub fn new(edges:&Edges,k : usize ,nonce: &[u8]) -> Self {
        let n = edges.n();
        let mut lable_matrix : Vec<Vec<VertexLabel>> = vec![vec![];k] ;
        let mut hasher = Hasher::new() ;
        lable_matrix[0] = (0..n)
            .map(|i| hasher.label_sources(&nonce,i))
            .collect();

        for col in 1..k {
            for vertex in 0..n {
                let parent_label: Vec<&VertexLabel> = edges
                    .get_parents(vertex)
                    .iter()
                    .map(|parent_index| &lable_matrix[col - 1][*parent_index])
                    .collect();
                let vertex_label = hasher.label_non_source(&parent_label);
                lable_matrix[col].push(vertex_label);
            }
        }
        LabelMatrix(lable_matrix)
    }
}

