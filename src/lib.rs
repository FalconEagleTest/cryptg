/// Encrypts the input plain text with the 32 bytes key and IV.
#[no_mangle]
fn encrypt_ige(plain: &[u8], key: &[u8], iv: &[u8]) ->  Vec<u8> {
    let mut key_array = [0; 32];

    key_array.copy_from_slice(key);

    let mut iv_array = [0; 32];

    iv_array.copy_from_slice(iv);

    let cipher = grammers_crypto::encrypt_ige(plain, &key_array, &iv_array);
    return cipher;

}

/// Decrypts the input cipher text with the 32 bytes key and IV.
#[no_mangle]
fn decrypt_ige(cipher: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut key_array = [0; 32];
    
    key_array.copy_from_slice(key);

    let mut iv_array = [0; 32];

    iv_array.copy_from_slice(iv);

    let plain = grammers_crypto::decrypt_ige(cipher, &key_array, &iv_array);
    return plain;

}

/// Factorizes the pair of primes ``pq`` into ``(p, q)``.
#[no_mangle]
fn factorize_pq_pair(pq: u64) -> (u64, u64) {
    grammers_crypto::factorize::factorize(pq)
}
