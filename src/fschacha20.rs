use crate::chacha20::chacha20::chacha20_block;
pub struct FSChaCha20 {
    key: [u8; 32],
    nonce: [u8; 12],
    counter: u32,
    block_counter: u64,
}

impl FSChaCha20 {
    const REKEY_INTERVAL: u64 = 224;

    pub fn new(key: &[u8; 32], nonce: &[u8; 12]) -> Self {
        Self {
            key: *key,
            nonce: *nonce,
            counter: 0,
            block_counter: 0,
        }
    }

    pub fn set_counter(&mut self, counter: u32) {
        self.counter = counter;
    }

    pub fn encrypt(&mut self, plaintext: &mut [u8]) {
        let mut block = [0u8; 64];
        for chunk in plaintext.chunks_mut(64) {
            self.next_block(&mut block);
            for (i, byte) in chunk.iter_mut().enumerate() {
                *byte ^= block[i];
            }
        }
        self.block_counter += plaintext.len() as u64 / 64;
        if self.block_counter >= Self::REKEY_INTERVAL {
            self.rekey();
            self.block_counter = 0;
        }
    }

    pub fn decrypt(&mut self, ciphertext: &mut [u8]) {
        self.encrypt(ciphertext);
    }

    pub fn next_block(&mut self, block: &mut [u8]) {
        chacha20_block(&self.key, &self.nonce, self.counter, block.as_mut().try_into().unwrap());
        self.counter = self.counter.checked_add(1).unwrap();
        if self.counter == 0 {
            panic!("FSChaCha20 counter wrapped around");
        }
    }

    fn rekey(&mut self) {
        let mut key_stream = [0u8; 64];
        self.next_block(&mut key_stream);
        for i in 0..32 {
            self.key[i] ^= key_stream[i];
        }
        self.counter = 0;
    }
}
