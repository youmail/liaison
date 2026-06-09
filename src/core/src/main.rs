// main.rs — Liaison Gateway entry point
// License: GPLv3
// Status: PROTOTYPE — see LIMITATIONS.md

use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

mod protocol;
mod vault;

use protocol::SignalArythmie;
use vault::TheVault;

pub struct ScopeEntry {
    pub index: u64,
    pub signal: SignalArythmie,
    pub hash: [u8; 32],
}

pub struct TheScope {
    pub logs: Vec<ScopeEntry>,
    pub last_hash: [u8; 32],
}

impl TheScope {
    pub fn new() -> Self {
        Self { logs: Vec::new(), last_hash: [0u8; 32] }
    }

    pub fn seal(&mut self, signal: SignalArythmie) {
        // TODO: replace unwrap() with error handling
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let index = self.logs.len() as u64;
        let mut h = Sha256::new();
        h.update(&self.last_hash);
        h.update(&index.to_be_bytes());
        h.update(&now.to_be_bytes());
        h.update(&signal.uid.to_be_bytes());
        h.update(&[signal.score_calcule, signal.decision]);

        let hash: [u8; 32] = h.finalize().into();

        println!("[SCOPE] #{} uid={} score={} decision={} \
                  hash={:02x}{:02x}{:02x}{:02x}...",
                 index, signal.uid,
                 signal.score_calcule, signal.decision,
                 hash[0], hash[1], hash[2], hash[3]);

        self.logs.push(ScopeEntry { index, signal, hash });
        self.last_hash = hash;
    }
}

fn main() {
    println!("[LIAISON] Starting homeostatic loop...");

    let mut scope = TheScope::new();
    let mut vault = TheVault::new();

    // Simulation — 105 events from UID 1042
    for i in 1u32..=105 {
        let signal = SignalArythmie {
            timestamp_ms: 1000 * i,
            uid: 1042,
            score_calcule: 45,
            decision: 1,
            _padding: 0,
            volume_paquet: 310,
        };
        scope.seal(signal);
        vault.evaluer(signal.uid,
                      signal.score_calcule,
                      signal.volume_paquet);
    }

    println!("[LIAISON] Done. {} events sealed.", scope.logs.len());
}