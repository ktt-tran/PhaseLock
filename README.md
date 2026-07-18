# PhaseLock


PhaseLock is a desktop file encryption application written entirely in **Rust** that uses audio files as a key source to protect files.

The idea behind PhaseLock is that instead of using only a password, users can use an audio file to generate the cryptographic key needed to encrypt and decrypt their files.

The current implementation uses modern authenticated encryption (AEAD) to securely protect user data through an easy-to-use graphical interface.

---

## What is PhaseLock?

PhaseLock converts an audio file into cryptographic key material and uses that key with an AEAD encryption algorithm to encrypt files.

Audio data itself is not used to encrypt the data directly. Instead, the audio file is processed as bytes and used to create the encryption key.

Because PhaseLock reads files as bytes, technically any file type can be used as a key source. However, audio files are the intended use case because they provide a unique and personal way for users to create their keys.

PhaseLock supports encrypting individual files, multiple files, or entire folders into a single compressed **`.lock`** archive that can later be securely decrypted or viewed.

---

## Features

### Desktop Application

- Written entirely in **Rust**
- Modern graphical interface built with **egui/eframe**
- Runs completely offline
- Cross-platform architecture (Windows, Linux, and macOS)

### Audio-Based Encryption

- Use audio files as the key source for encryption and decryption
- Supports common audio formats (WAV, MP3, FLAC, OGG, and M4A)
- Encrypt any type of file using the generated key
- Encrypt individual files, multiple files, or entire folders

### AEAD Encryption

PhaseLock uses **Authenticated Encryption with Associated Data (AEAD)**.

- **Industry Standard** — modern authenticated encryption used throughout the security industry
- **Confidentiality** — protects the contents of encrypted files
- **Integrity** — detects if encrypted files have been modified before decryption

### Password Protection

Users can optionally add a password for additional protection.

Files can be secured using:

- Audio key
- Password
- Audio key + password

### Secure File Recovery

A file can only be decrypted when the correct key information is provided.

Using a different audio file, modified audio file, or incorrect password will prevent successful decryption.

### Secure Viewer

Preview encrypted files without extracting all of the data onto disk.

- Preview supported encrypted files directly from memory
- Open encrypted files for temporary viewing
- Controlled extraction when desired
- Text-based files can be viewed without permanently creating an unencrypted copy on disk

### Archive Support

- Compress multiple files and folders into a single `.lock` archive
- Uses Zstandard (Zstd) compression before encryption
- Restore the original files and folder structure during decryption