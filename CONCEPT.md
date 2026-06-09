# Liaison — Technical Concept
*First implementation of the Initial Gravity paradigm*

## The paradigm — three principles

1. **Petrification by default** — every component is silent at rest
2. **Decision subsidiarity** — each decision at the lowest capable level
3. **Arrhythmia detection** — anomaly by deviation, not by blacklist

## Architecture

Three components:

**The Gate** — filters every outbound packet
- Android: VPNService API (no root)
- Linux: nftables OUTPUT hook

**The Vault** — time-boxed permission registry
- Permission: {uid, dst_cidr, port, protocol, expires_at, max_bytes}
- Revocation enforced by timerfd / AlarmManager

**The Scope** — cryptographic audit chain
- Every event sealed with SHA-256
- Tamper-evident: broken chain = immediate alert

## What makes it different

| Feature | NetGuard | GrapheneOS | Liaison |
|---|---|---|---|
| Time-boxed permissions | No | No | Yes |
| Arrhythmia scoring | No | No | Yes |
| Cryptographic audit | No | No | Yes |

## Known limitations

- Exfiltration via authorized channel: out of scope
- Android third-party channels: ~85% mitigation only
- C to Rust transport: stub, not implemented
- No tests yet

See LIMITATIONS.md for full list.

## References

- NIST SP 800-41 Rev.1
- CVE-2021-44228 (Log4Shell)
- CISA AA21-042A (Oldsmar)
- NetGuard: github.com/M66B/NetGuard