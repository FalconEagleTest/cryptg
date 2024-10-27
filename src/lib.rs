/// Encrypts the input plain text with the 32 bytes key and IV.
#[no_mangle]
fn encrypt_ige<'a>(plain: &'a [u8], key_array: &'a mut [u8; 32], iv_array: &'a mut [u8; 32]) ->  String  {

    let cipher = grammers_crypto::encrypt_ige(plain, &key_array, &iv_array);
   
    match String_result::from_utf8(cipher) {
        Ok(String_result) => return String_result,
        Err(e) => return"Error: {e}".to_string(),
    }
    


}

/// Decrypts the input cipher text with the 32 bytes key and IV.
#[no_mangle]
fn decrypt_ige<'a>(cipher: &'a [u8], key_array: &'a mut [u8; 32], iv_array: &'a mut [u8; 32]) -> String  {

    let plain = grammers_crypto::decrypt_ige(cipher, &key_array, &iv_array);
    match String_result::from_utf8(plain) {
        Ok(String_result) => return String_result,
        Err(e) => return "Error: {e}".to_string(),
    }

}

/// Factorizes the pair of primes ``pq`` into ``(p, q)``.
#[no_mangle]
fn factorize_pq_pair(pq: u64) -> (u64, u64) {
    grammers_crypto::factorize::factorize(pq)
}
