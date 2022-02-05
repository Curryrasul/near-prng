use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, BlockHeight, PanicOnDefault};

// generator is chacha function (read it https://bashtage.github.io/randomgen/bit_generators/chacha.html)
use rand_chacha::{self, rand_core::SeedableRng};
// traits for using generator functions
use rand_core::RngCore;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // block_index is = to block's index
    block_index: BlockHeight,
    // current seed for generator
    seed: u64,
}

#[near_bindgen]
impl Contract {
    // constructor function
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Contract already initialized");

        Self {
            block_index: env::block_index(),
            // converting random_seed ([u8; 32]) to u64
            seed: u64::from_be_bytes((env::random_seed())[..8].try_into().unwrap()),
        }
    }

    pub fn generate(&mut self) -> u64 {
        // if the block is new (not the same block from previous function call), state has to be updated
        if env::block_index() != self.block_index {
            self.block_index = env::block_index();
            self.seed = u64::from_be_bytes((env::random_seed())[..8].try_into().unwrap());
        }

        // number generation
        let mut gen = rand_chacha::ChaCha12Rng::seed_from_u64(self.seed);
        self.seed = gen.next_u64();

        log!("Generated number: {}", self.seed);
        log!("Current block: {}", self.block_index);

        self.seed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(block_index: BlockHeight, random_seed: Vec<u8>) -> VMContext {
        VMContextBuilder::new()
            .block_index(block_index)
            .random_seed(random_seed)
            .build()
    }

    #[test]
    fn test_init() {
        let block_index = 123456789u64;
        let random_seed: Vec<u8> = (0..32).map(|x| x).collect();

        let context = get_context(block_index, random_seed);
        testing_env!(context);

        let contract = Contract::new();

        assert!(contract.seed == u64::from_be_bytes((env::random_seed())[..8].try_into().unwrap()));
        assert!(contract.block_index == block_index);
    }

    #[test]
    fn test_generate() {
        let mut block_index = 123456789u64;
        let mut random_seed: Vec<u8> = (0..32).map(|x| x).collect();
        let mut context = get_context(block_index, random_seed);
        testing_env!(context);

        let mut contract = Contract::new();

        let first_seed = contract.seed;
        contract.generate();
        let second_seed = contract.seed;

        assert!(first_seed != second_seed);
        assert!(contract.block_index == block_index);

        block_index = 987654321u64;
        random_seed = (0..32).rev().map(|x| x).collect();
        context = get_context(block_index, random_seed);
        testing_env!(context);

        assert!(contract.block_index != block_index);

        contract.generate();

        assert!(contract.block_index == block_index);
    }
}
