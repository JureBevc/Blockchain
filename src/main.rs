mod block;
mod util;


fn main() {
    let mut block = block::Block {
        index: 0,
        timestamp: 0,
        hash: Vec::new(),
        difficulty: 0x00FFFFFFFFFFFFFF,
        payload: String::from("Banana"),
        nonce: 0
    };

    block.find_hash();
    println!("{}", block.to_string());
}
