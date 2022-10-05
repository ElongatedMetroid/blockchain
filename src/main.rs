use blockchain::*;

fn main() {
    let mut block = Block::new(0, 0, Vec::new(), 0, String::from("AMONG US"));

    println!("{:?}", block);

    let h = block.hash();
    block.set_hash(h);

    println!("{:?}", block);
}
