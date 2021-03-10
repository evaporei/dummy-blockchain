use dummy_blockchain::{Block, Blockchain};
use std::fmt;

#[derive(Default, Debug)]
struct Data {
    amount: usize,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let mut chain = Blockchain::new();
    chain.add_block(Block::new(1, "2021-04-25".to_string(), Data { amount: 5 }));
    chain.add_block(Block::new(2, "2021-05-02".to_string(), Data { amount: 10 }));

    println!("{:#?}", chain);
    println!("Is blockchain valid? {}", chain.check_valid());
}
