# PhaseLock

PhaseLock is a file encryption application that uses audio files as a key source to protect files.

The idea behind PhaseLock is instead of using only a password, users can use an audio file to generate the cryptographic key needed to encrypt and decrypt their files.

The current implementation uses modern authenticated encryption (AEAD) to securely protect user data.

## What is PhaseLock?

PhaseLock converts an audio file into cryptographic key material and uses that key with an AEAD encryption algorithm to encrypt files.

The contents audio file itself is not used to encrypt the data directly. Instead, it is processed as binary data and used to create the encryption key.

Because PhaseLock reads files as bytes, technically any file type can be used as a key source. However, audio files are the intended use case because they provide a unique and personal way for users to create their keys.

## Features

### Audio-Based Encryption

- Use audio files as the key source for encryption and decryption
- Supports common audio formats (WAV, MP3, FLAC, OGG, and M4A)
- Encrypt any type of file using the generated key

### AEAD Encryption

PhaseLock uses Authenticated Encryption with Associated Data (AEAD).

- **Standared** — standard for most industry cryptography professional works
- **Confidentiality** — protects the contents of encrypted files
- **Integrity** — detects if encrypted files have been modified and prevents re-creation of audio file to unlock personal data

### Password Protection

Users can optionally add a password for additional protection.

Files can be secured using:

- Audio key
- Password
- Audio key + password

### Secure File Recovery

A file can only be decrypted when the correct key information is provided.

Using a different audio file, modified audio file, or incorrect password will prevent successful decryption.

## Secure Viewer (Coming Soon)

A secure viewer is currently being developed.

The goal is to allow users to view encrypted files without fully decrypting them and creating an unprotected copy on their device.

Planned features include:

- Preview encrypted files
- Controlled extraction
