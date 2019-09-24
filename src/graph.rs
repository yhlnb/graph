
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub type VertexLabel = String;
pub const IN_DEGREE: usize = 2;

#[derive(Clone, Debug)]
pub struct Edges(pub Vec<Vec<usize>>);
impl Edges {
    pub fn chung(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut map1 = HashMap::new();
        for i in 0..n * IN_DEGREE {
            let j = rng.gen_range(0, IN_DEGREE * n);   
            map1.entry(i % n).or_insert(HashSet::new());
            map1.get_mut(&(i % n)).unwrap().insert(j % n);
        }
        let mut edges: Vec<Vec<usize>> = vec![];
        for i in 0..n {
            let s1 = map1.get(&i).unwrap();
            let mut v1 = vec![];
            for j in s1.iter() {
                v1.push(*j);
            }
            edges.push(v1);
        }
        Edges(edges)
    }
    pub fn get_parents(&self, vertex: usize) -> Vec<usize> {
        let mut parents = vec![];
        for (source_index, edges) in self.0.iter().enumerate() {
            if edges.contains(&vertex) {
                parents.push(source_index);
            }
        }
        parents
    }
    fn n(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone)]
pub struct LabelMatrix(pub Vec<Vec<VertexLabel>>);
impl LabelMatrix {
    pub fn new(edges: &Edges, k: usize, nonce: &[u8]) -> Self {
        let n = edges.n();
        let mut lable_matrix: Vec<Vec<VertexLabel>> = vec![vec![]; k];
        for i in 0..n {
            let mut hasher = Sha256::new();
            hasher.input(nonce);
            lable_matrix[0].push(hasher.result_str())
        }

        for col in 1..k {
            for vertex in 0..n {
                let parent_label: Vec<&VertexLabel> = edges
                    .get_parents(vertex)
                    .iter()
                    .map(|parent_index| &lable_matrix[col - 1][*parent_index])
                    .collect();
                let mut hasher = Sha256::new();
                for j in parent_label.iter() {
                    hasher.input_str(*j); 
                }
                let vertex_label = hasher.result_str();
                lable_matrix[col].push(vertex_label);
            }
        }
        LabelMatrix(lable_matrix)
    }
}
