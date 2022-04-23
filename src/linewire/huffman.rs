struct HuffmanTree {
    root: HuffmanNode,
    table: HuffmanTable,
}

struct HuffmanNode{
    left: Option<Self>,
    right: Option<Self>,
}

pub type HuffmanTable = HashMap<Vec<u8>, u8>;

impl Into<HashMap<Vec<u8>, u8>> for HuffmanTree{
    fn into(self) -> HashMap<Vec<u8>, u8> {
        todo!()
    }
}

impl HuffmanTree {
    pub fn new() -> Self{
        let tree = HuffmanTree{
            root: HuffmanNode{
                left: None,
                right: None,
            }
        }
    }
}