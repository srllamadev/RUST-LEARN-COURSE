# Data Encryption Examples in Rust / Ejemplos de Cifrado de Datos en Rust

This project demonstrates various data encryption techniques implemented in Rust, from simple classical ciphers to more structured approaches.

Este proyecto demuestra varias técnicas de cifrado de datos implementadas en Rust, desde cifrados clásicos simples hasta enfoques más estructurados.

## Examples Included / Ejemplos Incluidos

### 1. Caesar Cipher / Cifrado César
- **Description**: Classic substitution cipher where each letter is shifted by a fixed number of positions
- **Descripción**: Cifrado de sustitución clásico donde cada letra se desplaza un número fijo de posiciones
- **Security**: Very weak, easily broken / Muy débil, fácil de romper
- **Use case**: Educational purposes / Propósitos educativos

### 2. XOR Cipher / Cifrado XOR
- **Description**: Bitwise XOR operation between plaintext and key
- **Descripción**: Operación XOR bit a bit entre texto plano y clave
- **Security**: Depends on key strength / Depende de la fortaleza de la clave
- **Use case**: Simple obfuscation / Ofuscación simple

### 3. ROT13 / ROT13
- **Description**: Special case of Caesar cipher with 13-position shift
- **Descripción**: Caso especial del cifrado César con desplazamiento de 13 posiciones
- **Security**: No security, used for text obfuscation / Sin seguridad, usado para ofuscación
- **Use case**: Hide spoilers, simple text transformation / Ocultar spoilers, transformación simple

### 4. Base64 Encoding / Codificación Base64
- **Description**: Encoding scheme to represent binary data in ASCII
- **Descripción**: Esquema de codificación para representar datos binarios en ASCII
- **Security**: Not encryption, just encoding / No es cifrado, solo codificación
- **Use case**: Data transmission, storage / Transmisión y almacenamiento de datos

## Code Structure / Estructura del Código

### Main Functions / Funciones Principales
- `caesar_encrypt()` / `caesar_decrypt()`: Caesar cipher implementation
- `xor_encrypt()` / `xor_decrypt()`: XOR cipher operations
- `rot13()`: ROT13 transformation (self-inverse)
- `simple_base64_encode()` / `simple_base64_decode()`: Basic Base64 implementation

### Utility Functions / Funciones Utilitarias
- `bytes_to_hex()`: Convert bytes to hexadecimal representation
- `SimpleEncryption` struct: Reusable encryption interface

## Security Notes / Notas de Seguridad

⚠️ **Important / Importante**: These examples are for educational purposes only. For production applications, use well-tested cryptographic libraries.

⚠️ **Importante**: Estos ejemplos son solo para propósitos educativos. Para aplicaciones de producción, usa librerías criptográficas bien probadas.

### Recommended Libraries / Librerías Recomendadas
- `ring`: Modern cryptographic operations
- `rustls`: TLS implementation
- `aes`: AES encryption
- `chacha20poly1305`: ChaCha20-Poly1305 AEAD

## Running the Examples / Ejecutar los Ejemplos

```bash
# Navigate to the project directory
# Navegar al directorio del proyecto
cd xyz_Example2

# Run the examples
# Ejecutar los ejemplos
cargo run

# Run tests
# Ejecutar pruebas
cargo test
```

## Expected Output / Salida Esperada

```
=== DATA ENCRYPTION EXAMPLES / EJEMPLOS DE CIFRADO DE DATOS ===

1. === CAESAR CIPHER / CIFRADO CÉSAR ===
Original message / Mensaje original: Hello Rust World!
Encrypted / Cifrado: Khoor Uxvw Zruog!
Decrypted / Descifrado: Hello Rust World!

2. === XOR CIPHER / CIFRADO XOR ===
Original message / Mensaje original: Secret Message
Key / Clave: MyKey
Encrypted (hex) / Cifrado (hex): 0e1a0704190d5f0b0004081a0a
Decrypted / Descifrado: Secret Message

3. === ROT13 CIPHER / CIFRADO ROT13 ===
Original message / Mensaje original: This is a ROT13 example!
ROT13 encoded / Codificado ROT13: Guvf vf n EBG13 rknzcyr!
ROT13 decoded / Decodificado ROT13: This is a ROT13 example!

4. === BASE64 ENCODING / CODIFICACIÓN BASE64 ===
Original message / Mensaje original: Hello, Base64 World!
Base64 encoded / Codificado Base64: SGVsbG8sIEJhc2U2NCBXb3JsZCE=
Base64 decoded / Decodificado Base64: Hello, Base64 World!
```

## Learning Objectives / Objetivos de Aprendizaje

After studying this example, you should understand:
Después de estudiar este ejemplo, deberías entender:

- Basic encryption concepts / Conceptos básicos de cifrado
- Difference between encoding and encryption / Diferencia entre codificación y cifrado
- Implementation of classical ciphers in Rust / Implementación de cifrados clásicos en Rust
- Use of iterators and closures for data transformation / Uso de iteradores y closures para transformación de datos
- Error handling and input validation / Manejo de errores y validación de entrada
- Unit testing for cryptographic functions / Pruebas unitarias para funciones criptográficas

## Advanced Topics / Temas Avanzados

For production-ready encryption, consider:
Para cifrado listo para producción, considera:

1. **Key Management**: Secure key generation and storage
2. **Authenticated Encryption**: Prevents tampering (e.g., AES-GCM)
3. **Key Derivation**: PBKDF2, scrypt, Argon2 for password-based encryption
4. **Random Number Generation**: Cryptographically secure randomness
5. **Side-Channel Attacks**: Timing attacks, power analysis protection

## Further Reading / Lectura Adicional

- [Rust Crypto Libraries](https://github.com/RustCrypto)
- [The Rustonomicon - Unsafe Rust](https://doc.rust-lang.org/nomicon/)
- [Applied Cryptography by Bruce Schneier](https://www.schneier.com/books/applied_cryptography/)
- [Crypto 101](https://www.crypto101.io/)

## License / Licencia

This educational example is provided under MIT license for learning purposes.
Este ejemplo educativo se proporciona bajo licencia MIT para propósitos de aprendizaje.