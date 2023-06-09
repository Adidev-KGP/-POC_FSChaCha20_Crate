#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_fs_chacha20(){
        let key = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];

        let nonce = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let mut fs_chacha20 = FSChaCha20::new(&key, &nonce);
        fs_chacha20.set_rekey_interval(256);
        let mut plaintext = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
        ];

        let mut test_cipher = [
            0xa9, 0x3d, 0xf4, 0xef, 0x03, 0x01, 0x1f, 0x3d,
            0xb9, 0x5f, 0x60, 0xd9, 0x96, 0xe1, 0x78, 0x5d,
            0xf5, 0xde, 0x38, 0xfc, 0x39, 0xbf, 0xcb, 0x66,
            0x3a, 0x47, 0xbb, 0x55, 0x61, 0x92, 0x83, 0x49
        ];

        let mut ciphertext = plaintext.clone();
        fs_chacha20.encrypt(&mut ciphertext);

        assert_eq!(ciphertext, test_cipher);

    }
}
