

//useless



use sha3::{Digest,Sha3_256};
use crate::graph::VertexLabel;



#[derive(Debug)]
pub struct Hasher(Sha3_256);
impl Hasher {
    pub fn new() -> Self{
        Hasher(Sha3_256::new())
    }

    pub fn digest(&mut self) -> Vec<u8> {
        self.0.result_reset().to_vec()
    }

    pub fn label_sources(&mut self , nonce: &[u8] ,i:usize) -> Vec<u8> {
        self.0.input(nonce);
        self.0.input(&i.to_be_bytes());
        self.digest()
    }

    pub fn label_non_source(&mut self, parent_labels: &[&VertexLabel]) -> Vec<u8> {
        for parent_label in parent_labels {
            self.0.input(parent_label);
        }
        self.digest()
    }


}