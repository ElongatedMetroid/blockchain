use blockchain::*;

fn main() {
    let mut block = Block::new(
        0, 
        0, 
        vec![0; 32],
        0, 
        String::from("Genesis block!"),
        0x00000000ffffffffffffffffffffffff
    );

    println!("{:?}", block);

    block.mine();   

    println!("{:?}", block);
}
