use std::collections::HashSet;

use crate::*;

#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

#[derive(Clone)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.inputs.iter().flat_map(|input| input.bytes()).collect::<Vec<u8>>());
        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes
    }
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs
            .iter()
            // Map all the inputs to the value in the input
            .map(|input| input.value)
            // Sum all the inputs
            .sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs
            .iter()
            // Map all the outputs to the value in the output
            .map(|output| output.value)
            // Sum all the outputs
            .sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs.iter().map(|input| input.hash()).collect()
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs.iter().map(|output| output.hash()).collect()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}
