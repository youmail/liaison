# Liaison

> *A first implementation of the **Initial Gravity** paradigm.*
> **Only intentional connections.**

## The problem

Applications communicate freely by default. This is why ransomware spreads, why malware exfiltrates data undetected, why compromised IoT sensors become attack vectors.

Initial Gravity fixes this with one principle: **total silence by default**. Every outbound connection is an explicit, time-boxed, auditable exception.

## Status

Prototype stage. Core algorithm exists. Not production-ready.
See LIMITATIONS.md for known issues.

## Architecture

Three components:
- **The Gate** — filters every outbound packet
- **The Vault** — stores time-boxed permissions
- **The Scope** — cryptographic audit chain

## License

GPLv3

---
*Initial Gravity is the paradigm. Liaison is its first cell.*