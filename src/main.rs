use blockchain::*;

fn main() {
    let difficulty: u128 = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(
        0, 
        now(), 
        vec![0; 32],
        0, 
        String::from("Genesis block!"),
        difficulty
    );

    block.mine();   

    println!("Mined genesis block {:?}", block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(
            i, 
            now(), 
            last_hash,
            0, 
            String::from("Another block"),
            difficulty
        );
    
        block.mine();   
        println!("Mined another block {:?}", block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);

        println!("Verify: {}", blockchain.verify());
    }
}
