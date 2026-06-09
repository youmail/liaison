# Known Limitations

Read this before using or evaluating this code.

## Critical bugs (fix before any deployment)

**1. Division by zero — liaison_edge.c**
If freq_ms or vol_octets is 0 at init, the program crashes.
Fix: add guard in liaison_initialiser_profil()

**2. Integer overflow — liaison_edge.c**
delta_freq * 50 can overflow uint32_t silently.
Fix: cast to uint64_t before multiplication.

**3. Timestamp wraparound — liaison_edge.c**
uint32_t wraps after ~49 days. Causes false GUILLOTINE.
Fix: wraparound-safe subtraction.

**4. unwrap() panic — main.rs**
SystemTime::now().unwrap() panics on corrupted clock.
Fix: proper error handling.

**5. Unbounded Vec — main.rs**
chaine_logs grows forever. Memory exhaustion under attack.
Fix: ring buffer with max size.

## Missing pieces

- C to Rust transport: stub only, not implemented
- No unit tests, no integration tests
- Android VPNService layer: not started
- No authentication on C to Rust channel

## Design limitations

- Max 16 UIDs per device (compile-time constant)
- Arrhythmia score weights not empirically calibrated
- False positive rate unknown
- Model adaptation requires 100 clean cycles (arbitrary)

## Summary

| Issue | Severity | Status |
|---|---|---|
| Division by zero | Critical | Open |
| Integer overflow | Critical | Open |
| Timestamp wrap | High | Open |
| Transport C to Rust | Blocking | Not implemented |
| No tests | High | Not started |