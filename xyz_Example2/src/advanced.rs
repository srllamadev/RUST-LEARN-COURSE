// Advanced Encryption Example: File Encryption with Multiple Algorithms
// Ejemplo de Cifrado Avanzado: Cifrado de Archivos con Múltiples Algoritmos

use std::fs;
use std::io::{self, Write};

/// Advanced encryption manager with multiple cipher options
/// Gestor de cifrado avanzado con múltiples opciones de cifrado
pub struct EncryptionManager {
    algorithm: CipherType,
    key: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum CipherType {
    Caesar(u8),      // Caesar cipher with shift
    XOR,             // XOR cipher
    Vigenere,        // Vigenère cipher
}

impl EncryptionManager {
    /// Create a new encryption manager
    /// Crear un nuevo gestor de cifrado
    pub fn new(algorithm: CipherType, key: &str) -> Self {
        Self {
            algorithm,
            key: key.as_bytes().to_vec(),
        }
    }
    
    /// Encrypt data using the selected algorithm
    /// Cifrar datos usando el algoritmo seleccionado
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        match &self.algorithm {
            CipherType::Caesar(shift) => self.caesar_encrypt(data, *shift),
            CipherType::XOR => self.xor_encrypt(data),
            CipherType::Vigenere => self.vigenere_encrypt(data),
        }
    }
    
    /// Decrypt data using the selected algorithm
    /// Descifrar datos usando el algoritmo seleccionado
    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        match &self.algorithm {
            CipherType::Caesar(shift) => self.caesar_decrypt(data, *shift),
            CipherType::XOR => self.xor_decrypt(data),
            CipherType::Vigenere => self.vigenere_decrypt(data),
        }
    }
    
    /// Encrypt a file and save to a new file
    /// Cifrar un archivo y guardarlo en un nuevo archivo
    pub fn encrypt_file(&self, input_path: &str, output_path: &str) -> io::Result<()> {
        let data = fs::read(input_path)?;
        let encrypted = self.encrypt(&data);
        fs::write(output_path, encrypted)?;
        println!("File encrypted: {} -> {}", input_path, output_path);
        Ok(())
    }
    
    /// Decrypt a file and save to a new file
    /// Descifrar un archivo y guardarlo en un nuevo archivo
    pub fn decrypt_file(&self, input_path: &str, output_path: &str) -> io::Result<()> {
        let data = fs::read(input_path)?;
        let decrypted = self.decrypt(&data);
        fs::write(output_path, decrypted)?;
        println!("File decrypted: {} -> {}", input_path, output_path);
        Ok(())
    }
    
    // Private implementation methods / Métodos de implementación privados
    
    fn caesar_encrypt(&self, data: &[u8], shift: u8) -> Vec<u8> {
        data.iter()
            .map(|&b| {
                if b.is_ascii_alphabetic() {
                    let base = if b.is_ascii_lowercase() { b'a' } else { b'A' };
                    (((b - base) + shift) % 26) + base
                } else {
                    b
                }
            })
            .collect()
    }
    
    fn caesar_decrypt(&self, data: &[u8], shift: u8) -> Vec<u8> {
        self.caesar_encrypt(data, 26 - shift)
    }
    
    fn xor_encrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &b)| b ^ self.key[i % self.key.len()])
            .collect()
    }
    
    fn xor_decrypt(&self, data: &[u8]) -> Vec<u8> {
        // XOR is its own inverse / XOR es su propia inversa
        self.xor_encrypt(data)
    }
    
    fn vigenere_encrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &b)| {
                if b.is_ascii_alphabetic() {
                    let key_char = self.key[i % self.key.len()];
                    let key_shift = if key_char.is_ascii_alphabetic() {
                        (key_char.to_ascii_uppercase() - b'A') as u8
                    } else {
                        key_char % 26
                    };
                    
                    let base = if b.is_ascii_lowercase() { b'a' } else { b'A' };
                    (((b - base) + key_shift) % 26) + base
                } else {
                    b
                }
            })
            .collect()
    }
    
    fn vigenere_decrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &b)| {
                if b.is_ascii_alphabetic() {
                    let key_char = self.key[i % self.key.len()];
                    let key_shift = if key_char.is_ascii_alphabetic() {
                        (key_char.to_ascii_uppercase() - b'A') as u8
                    } else {
                        key_char % 26
                    };
                    
                    let base = if b.is_ascii_lowercase() { b'a' } else { b'A' };
                    (((b - base + 26) - key_shift) % 26) + base
                } else {
                    b
                }
            })
            .collect()
    }
}

/// Demo function showing advanced encryption features
/// Función de demostración que muestra características de cifrado avanzado
pub fn advanced_encryption_demo() {
    println!("=== ADVANCED ENCRYPTION DEMO / DEMO DE CIFRADO AVANZADO ===\n");
    
    // Test different algorithms / Probar diferentes algoritmos
    let test_message = "This is a secret message for advanced encryption!";
    
    // Caesar cipher demo / Demo de cifrado César
    println!("1. Caesar Cipher (shift 7) / Cifrado César (desplazamiento 7):");
    let caesar_manager = EncryptionManager::new(CipherType::Caesar(7), "");
    let caesar_encrypted = caesar_manager.encrypt(test_message.as_bytes());
    let caesar_decrypted = caesar_manager.decrypt(&caesar_encrypted);
    
    println!("Original: {}", test_message);
    println!("Encrypted: {}", String::from_utf8_lossy(&caesar_encrypted));
    println!("Decrypted: {}", String::from_utf8_lossy(&caesar_decrypted));
    println!();
    
    // XOR cipher demo / Demo de cifrado XOR
    println!("2. XOR Cipher / Cifrado XOR:");
    let xor_manager = EncryptionManager::new(CipherType::XOR, "SECRETKEY");
    let xor_encrypted = xor_manager.encrypt(test_message.as_bytes());
    let xor_decrypted = xor_manager.decrypt(&xor_encrypted);
    
    println!("Original: {}", test_message);
    println!("Encrypted (hex): {}", bytes_to_hex(&xor_encrypted));
    println!("Decrypted: {}", String::from_utf8_lossy(&xor_decrypted));
    println!();
    
    // Vigenère cipher demo / Demo de cifrado Vigenère
    println!("3. Vigenère Cipher / Cifrado Vigenère:");
    let vigenere_manager = EncryptionManager::new(CipherType::Vigenere, "RUST");
    let vigenere_encrypted = vigenere_manager.encrypt(test_message.as_bytes());
    let vigenere_decrypted = vigenere_manager.decrypt(&vigenere_encrypted);
    
    println!("Original: {}", test_message);
    println!("Key: RUST");
    println!("Encrypted: {}", String::from_utf8_lossy(&vigenere_encrypted));
    println!("Decrypted: {}", String::from_utf8_lossy(&vigenere_decrypted));
    println!();
    
    // File encryption demo / Demo de cifrado de archivos
    println!("4. File Encryption Demo / Demo de Cifrado de Archivos:");
    
    // Create a sample file / Crear un archivo de muestra
    let sample_content = "This is a sample file content for encryption testing.\nLine 2 of the file.\nEnd of file.";
    if let Err(e) = fs::write("sample.txt", sample_content) {
        println!("Error creating sample file: {}", e);
        return;
    }
    
    // Encrypt the file / Cifrar el archivo
    let file_manager = EncryptionManager::new(CipherType::XOR, "FILEKEY123");
    match file_manager.encrypt_file("sample.txt", "sample_encrypted.txt") {
        Ok(_) => println!("File encryption successful / Cifrado de archivo exitoso"),
        Err(e) => println!("File encryption failed: {}", e),
    }
    
    // Decrypt the file / Descifrar el archivo
    match file_manager.decrypt_file("sample_encrypted.txt", "sample_decrypted.txt") {
        Ok(_) => println!("File decryption successful / Descifrado de archivo exitoso"),
        Err(e) => println!("File decryption failed: {}", e),
    }
    
    // Verify the decryption / Verificar el descifrado
    if let Ok(decrypted_content) = fs::read_to_string("sample_decrypted.txt") {
        println!("Decrypted file content matches original: {}", 
                 decrypted_content == sample_content);
    }
    
    // Clean up demo files / Limpiar archivos de demo
    let _ = fs::remove_file("sample.txt");
    let _ = fs::remove_file("sample_encrypted.txt");
    let _ = fs::remove_file("sample_decrypted.txt");
}

/// Interactive CLI for encryption operations
/// CLI interactivo para operaciones de cifrado
pub fn interactive_encryption_cli() {
    println!("=== INTERACTIVE ENCRYPTION CLI / CLI DE CIFRADO INTERACTIVO ===");
    println!("Available algorithms / Algoritmos disponibles:");
    println!("1. Caesar Cipher / Cifrado César");
    println!("2. XOR Cipher / Cifrado XOR");
    println!("3. Vigenère Cipher / Cifrado Vigenère");
    println!();
    
    print!("Select algorithm (1-3) / Selecciona algoritmo (1-3): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let algorithm = match input.trim() {
        "1" => {
            print!("Enter shift value (1-25) / Ingresa valor de desplazamiento (1-25): ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let shift: u8 = input.trim().parse().unwrap_or(3);
            CipherType::Caesar(shift)
        },
        "2" => CipherType::XOR,
        "3" => CipherType::Vigenere,
        _ => {
            println!("Invalid selection, using Caesar cipher / Selección inválida, usando cifrado César");
            CipherType::Caesar(3)
        }
    };
    
    print!("Enter encryption key / Ingresa clave de cifrado: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let key = input.trim().to_string();
    
    print!("Enter message to encrypt / Ingresa mensaje a cifrar: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let message = input.trim().to_string();
    
    let manager = EncryptionManager::new(algorithm, &key);
    let encrypted = manager.encrypt(message.as_bytes());
    let decrypted = manager.decrypt(&encrypted);
    
    println!("\nResults / Resultados:");
    println!("Original: {}", message);
    println!("Encrypted: {}", String::from_utf8_lossy(&encrypted));
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod advanced_tests {
    use super::*;

    #[test]
    fn test_encryption_manager_caesar() {
        let manager = EncryptionManager::new(CipherType::Caesar(5), "");
        let original = b"Hello World";
        let encrypted = manager.encrypt(original);
        let decrypted = manager.decrypt(&encrypted);
        assert_eq!(original, &decrypted[..]);
    }

    #[test]
    fn test_encryption_manager_xor() {
        let manager = EncryptionManager::new(CipherType::XOR, "testkey");
        let original = b"Secret message for testing";
        let encrypted = manager.encrypt(original);
        let decrypted = manager.decrypt(&encrypted);
        assert_eq!(original, &decrypted[..]);
    }

    #[test]
    fn test_encryption_manager_vigenere() {
        let manager = EncryptionManager::new(CipherType::Vigenere, "KEY");
        let original = b"Hello World";
        let encrypted = manager.encrypt(original);
        let decrypted = manager.decrypt(&encrypted);
        assert_eq!(original, &decrypted[..]);
    }

    #[test]
    fn test_different_algorithms_produce_different_results() {
        let original = b"Test message";
        
        let caesar = EncryptionManager::new(CipherType::Caesar(3), "");
        let xor = EncryptionManager::new(CipherType::XOR, "key");
        let vigenere = EncryptionManager::new(CipherType::Vigenere, "KEY");
        
        let caesar_encrypted = caesar.encrypt(original);
        let xor_encrypted = xor.encrypt(original);
        let vigenere_encrypted = vigenere.encrypt(original);
        
        // All should be different from original
        assert_ne!(original, &caesar_encrypted[..]);
        assert_ne!(original, &xor_encrypted[..]);
        assert_ne!(original, &vigenere_encrypted[..]);
        
        // All should be different from each other
        assert_ne!(caesar_encrypted, xor_encrypted);
        assert_ne!(caesar_encrypted, vigenere_encrypted);
        assert_ne!(xor_encrypted, vigenere_encrypted);
    }
}