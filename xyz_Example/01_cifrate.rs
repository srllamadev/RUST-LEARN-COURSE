use aes::Aes256; // AES con clave de 256 bits
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex; // Para definir bytes con hex

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn main() {
    // ============================
    // KEY AND IV (Initialization Vector)
    // ============================
    // AES-256 necesita una clave de 32 bytes (256 bits)
    let key = b"verysecretkeyverysecretkey1234"; // 32 bytes
    // IV debe ser de 16 bytes para AES
    let iv = b"uniqueinitvector"; 

    // ============================
    // MESSAGE TO ENCRYPT
    // ============================
    let plaintext = b"Hello students, welcome to Rust crypto!";

    // ============================
    // ENCRYPTION
    // ============================
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext);

    println!("Original text: {:?}", String::from_utf8_lossy(plaintext));
    println!("Encrypted (hex): {:?}", hex::encode(&ciphertext));

    // ============================
    // DECRYPTION
    // ============================
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

    println!("Decrypted text: {:?}", String::from_utf8_lossy(&decrypted_ciphertext));
}
