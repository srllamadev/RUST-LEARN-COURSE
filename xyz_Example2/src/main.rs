// Data Encryption Examples in Rust
// Ejemplos de Cifrado de Datos en Rust

use std::collections::HashMap;
use std::io::{self, Write};

mod advanced;

fn main() {
    println!("=== DATA ENCRYPTION EXAMPLES / EJEMPLOS DE CIFRADO DE DATOS ===\n");
    
    loop {
        println!("Select demo / Selecciona demo:");
        println!("1. Basic encryption examples / Ejemplos básicos de cifrado");
        println!("2. Advanced encryption demo / Demo de cifrado avanzado");
        println!("3. Interactive encryption CLI / CLI de cifrado interactivo");
        println!("4. Exit / Salir");
        
        print!("Enter choice (1-4) / Ingresa opción (1-4): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => basic_encryption_examples(),
            "2" => advanced::advanced_encryption_demo(),
            "3" => advanced::interactive_encryption_cli(),
            "4" => {
                println!("Goodbye! / ¡Adiós!");
                break;
            },
            _ => println!("Invalid choice / Opción inválida\n"),
        }
        
        println!("\n{}\n", "=".repeat(60));
    }
}

fn basic_encryption_examples() {
    println!("=== BASIC ENCRYPTION EXAMPLES / EJEMPLOS BÁSICOS DE CIFRADO ===\n");

    // 1. Caesar Cipher Example / Ejemplo de Cifrado César
    caesar_cipher_demo();
    
    // 2. Simple XOR Cipher / Cifrado XOR Simple
    xor_cipher_demo();
    
    // 3. ROT13 Implementation / Implementación ROT13
    rot13_demo();
    
    // 4. Base64 Encoding (not encryption but encoding) / Codificación Base64
    base64_demo();
}

// === CAESAR CIPHER / CIFRADO CÉSAR ===
fn caesar_cipher_demo() {
    println!("1. === CAESAR CIPHER / CIFRADO CÉSAR ===");
    
    let message = "Hello Rust World!";
    let shift = 3;
    
    let encrypted = caesar_encrypt(message, shift);
    let decrypted = caesar_decrypt(&encrypted, shift);
    
    println!("Original message / Mensaje original: {}", message);
    println!("Encrypted / Cifrado: {}", encrypted);
    println!("Decrypted / Descifrado: {}", decrypted);
    println!();
}

fn caesar_encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = (((c as u8 - base) + shift) % 26) + base;
                shifted as char
            } else {
                c // Non-alphabetic characters remain unchanged
            }
        })
        .collect()
}

fn caesar_decrypt(text: &str, shift: u8) -> String {
    caesar_encrypt(text, 26 - shift) // Reverse the shift
}

// === XOR CIPHER / CIFRADO XOR ===
fn xor_cipher_demo() {
    println!("2. === XOR CIPHER / CIFRADO XOR ===");
    
    let message = "Secret Message";
    let key = "MyKey";
    
    let encrypted = xor_encrypt(message.as_bytes(), key.as_bytes());
    let decrypted = xor_decrypt(&encrypted, key.as_bytes());
    let decrypted_str = String::from_utf8_lossy(&decrypted);
    
    println!("Original message / Mensaje original: {}", message);
    println!("Key / Clave: {}", key);
    println!("Encrypted (hex) / Cifrado (hex): {}", bytes_to_hex(&encrypted));
    println!("Decrypted / Descifrado: {}", decrypted_str);
    println!();
}

fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

fn xor_decrypt(encrypted_data: &[u8], key: &[u8]) -> Vec<u8> {
    // XOR decryption is the same as encryption
    // El descifrado XOR es igual que el cifrado
    xor_encrypt(encrypted_data, key)
}

// === ROT13 CIPHER / CIFRADO ROT13 ===
fn rot13_demo() {
    println!("3. === ROT13 CIPHER / CIFRADO ROT13 ===");
    
    let message = "This is a ROT13 example!";
    let encoded = rot13(message);
    let decoded = rot13(&encoded); // ROT13 is its own inverse
    
    println!("Original message / Mensaje original: {}", message);
    println!("ROT13 encoded / Codificado ROT13: {}", encoded);
    println!("ROT13 decoded / Decodificado ROT13: {}", decoded);
    println!();
}

fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| {
            match c {
                'a'..='z' => ((c as u8 - b'a' + 13) % 26 + b'a') as char,
                'A'..='Z' => ((c as u8 - b'A' + 13) % 26 + b'A') as char,
                _ => c,
            }
        })
        .collect()
}

// === BASE64 ENCODING / CODIFICACIÓN BASE64 ===
fn base64_demo() {
    println!("4. === BASE64 ENCODING / CODIFICACIÓN BASE64 ===");
    
    let message = "Hello, Base64 World!";
    let encoded = simple_base64_encode(message.as_bytes());
    let decoded = simple_base64_decode(&encoded);
    let decoded_str = String::from_utf8_lossy(&decoded);
    
    println!("Original message / Mensaje original: {}", message);
    println!("Base64 encoded / Codificado Base64: {}", encoded);
    println!("Base64 decoded / Decodificado Base64: {}", decoded_str);
    println!();
}

// Simple Base64 implementation (for educational purposes)
// Implementación simple de Base64 (para propósitos educativos)
fn simple_base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
        
        result.push(ALPHABET[((b >> 18) & 63) as usize] as char);
        result.push(ALPHABET[((b >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 { ALPHABET[((b >> 6) & 63) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { ALPHABET[(b & 63) as usize] as char } else { '=' });
    }
    
    result
}

fn simple_base64_decode(encoded: &str) -> Vec<u8> {
    let mut decode_table = HashMap::new();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    for (i, c) in alphabet.chars().enumerate() {
        decode_table.insert(c, i as u8);
    }
    
    let mut result = Vec::new();
    let chars: Vec<char> = encoded.chars().filter(|&c| c != '=').collect();
    
    for chunk in chars.chunks(4) {
        if chunk.len() < 2 { break; }
        
        let b1 = decode_table.get(&chunk[0]).unwrap_or(&0);
        let b2 = decode_table.get(&chunk[1]).unwrap_or(&0);
        let b3 = if chunk.len() > 2 { decode_table.get(&chunk[2]).unwrap_or(&0) } else { &0 };
        let b4 = if chunk.len() > 3 { decode_table.get(&chunk[3]).unwrap_or(&0) } else { &0 };
        
        let combined = ((*b1 as u32) << 18) | ((*b2 as u32) << 12) | ((*b3 as u32) << 6) | (*b4 as u32);
        
        result.push((combined >> 16) as u8);
        if chunk.len() > 2 {
            result.push((combined >> 8) as u8);
        }
        if chunk.len() > 3 {
            result.push(combined as u8);
        }
    }
    
    result
}

// === UTILITY FUNCTIONS / FUNCIONES UTILITARIAS ===
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

// === ADVANCED ENCRYPTION STRUCT / ESTRUCTURA DE CIFRADO AVANZADA ===
pub struct SimpleEncryption {
    key: Vec<u8>,
}

impl SimpleEncryption {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.as_bytes().to_vec(),
        }
    }
    
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        xor_encrypt(data, &self.key)
    }
    
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8> {
        xor_decrypt(encrypted_data, &self.key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher() {
        let original = "Hello";
        let encrypted = caesar_encrypt(original, 3);
        let decrypted = caesar_decrypt(&encrypted, 3);
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_xor_cipher() {
        let original = b"Secret Message";
        let key = b"Key";
        let encrypted = xor_encrypt(original, key);
        let decrypted = xor_decrypt(&encrypted, key);
        assert_eq!(original, &decrypted[..]);
    }

    #[test]
    fn test_rot13() {
        let original = "Hello World";
        let encoded = rot13(original);
        let decoded = rot13(&encoded);
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_simple_encryption_struct() {
        let cipher = SimpleEncryption::new("MySecretKey");
        let original = b"This is a test message";
        let encrypted = cipher.encrypt(original);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(original, &decrypted[..]);
    }
}