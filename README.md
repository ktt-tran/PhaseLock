# Audio-Signal Based Encryption

An experimental encryption approach that explores the use of audio signals as a source for cryptographic key generation.

The goal of this project is to investigate whether the characteristics of an audio signal can be used to create a reproducible key that allows secure encryption and decryption of data.

## Overview

This project keeps the same frontend facing concept of protecting user files using audio files to encrypt and decrpyt them. The AEAD implementation of this program used standard encryption alogrithmic apporach which is widely known as one of the most secure encryption scheme to translate the byte code of an audio file to generate an encryption key.

This project is an exploratory alternative approach: using fundemental information of an audio signal as the foundation for encrypting files and later decrpyt them.

Instead of just using the byte code of an audio file to encrypt data, which can technically be done with any file, the characteristics of an audio signal are analyzed to encrypt files and those same properties can be used to unlock them.

The idea here is that the encryption key can be re-created anywhere by reproducing a nearly identical audio message that was used to lock files, using speech as the password.

## Core Idea: Signal Processing Approach

The program explores using audio signal properties such as:

- Frequency characteristics
- Dominant frequencies
- Spectral information
- Time-domain patterns
- Signal similarity measurements

Audio signals can be compared using techniques such as cross-correlation to determine whether two signals contain similar patterns.

The goal is not to store the original audio file as the key, but to derive a consistent representation of the signal that can be used for secure key generation.

## Audio Matching

One of the main concepts being explored is signal similarity verification.

During unlocking:

The program uses measurements such as cross-correlation or signal error analysis to determine whether two audio signals are equivalent enough to unlock the encrypted data.

## Research Questions

This project explores questions such as:

- Is speech recognition by comparing signal properties a practical way of protecting user data?
- How much variation can an audio signal tolerate while remaining recognizable?
- Can signal processing techniques provide secure authentication?

## Current Status

This project is currently in the research and development stage.

Current areas of exploration:

- Audio feature extraction
- Signal comparison methods
- Key derivation strategies
- Encryption integration

## Disclaimer

This project is an experimental exploration of combining digital signal processing with cryptographic systems.

It does not replace established key management methods and should not be considered a complete security solution without further testing and evaluation.
