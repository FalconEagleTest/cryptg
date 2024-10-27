/// Encrypts the input plain text with the 32 bytes key and IV.
#[no_mangle]
fn encrypt_ige(plain: &mut [u8], key_array: &mut [u8], iv_array: &mut [u8]) -> [u8; 1024]  {

    let cipher = grammers_crypto::encrypt_ige(plain, &key_array, &iv_array);
    let s = format!("{:?}", &cipher);


    let mut array = [0u8; 1024];
    for (&x, p) in cipher.iter().zip(array.iter_mut()) {
        *p = x;
    }
    return array;
    
    
    
    


}

/// Decrypts the input cipher text with the 32 bytes key and IV.
#[no_mangle]
fn decrypt_ige(cipher: &mut [u8], key_array: &mut [u8], iv_array: &mut [u8]) ->  [u8; 1024]   {

    let plain = grammers_crypto::decrypt_ige(cipher, &key_array, &iv_array);
    let s = format!("{:?}", &plain);
    let mut array = [0u8; 1024];
    for (&x, p) in plain.iter().zip(array.iter_mut()) {
        *p = x;
    }
    return array;

}

/// Factorizes the pair of primes ``pq`` into ``(p, q)``.
#[no_mangle]
fn factorize_pq_pair(pq: u64) -> (u64, u64) {
    grammers_crypto::factorize::factorize(pq)
}
