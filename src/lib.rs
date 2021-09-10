use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct NFT {
    pub nft_addresses: Vec<String>
}

impl NFT {
    pub fn new(addresses_vec: Vec<String>) -> Self {
        NFT { nft_addresses: addresses_vec }
    }

    pub fn get_next_bundle(&mut self, bundle_size: usize) -> Option<Vec<String>> {
        let mut bundle_vec = Vec::new();
        let mut rng = thread_rng();

        if self.nft_addresses.len() > 1 && bundle_size <= self.nft_addresses.len() {
            self.nft_addresses.shuffle(&mut rng);
            for _ in 0..bundle_size {
                bundle_vec.push(self.nft_addresses[0].clone());
                self.nft_addresses.remove(0);
            }
            return Some(bundle_vec)
        } else {
            return None
        }
    }
}
