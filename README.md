# NEAR Protocol PRNG demo (smart-contract in Rust)

## How it works ?

Generator is deterministic, and it is determined by random_seed() from [near-sdk](https://docs.rs/near-sdk/latest/near_sdk/).

random_seed() returns 32 byte vector, and it is the same for the same block. So the problem is that the generator function will return the same output with the same seed (happens, when more than one generate function call). 

The solution is to save block_height from the last function call, and if it is the same - we can generate random number not from random_seed(), but f.e. from the previous generated number.
