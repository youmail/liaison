// protocol.rs — Shared binary structure C/Rust
// License: GPLv3

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SignalArythmie {
    pub timestamp_ms:  u32,
    pub uid:           u32,
    pub score_calcule: u8,
    pub decision:      u8,  // 1=MARQUAGE 2=GUILLOTINE
    pub _padding:      u16,
    pub volume_paquet: u32,
}

impl SignalArythmie {
    pub fn is_critical(&self) -> bool {
        self.decision == 2
    }
    pub fn is_suspicious(&self) -> bool {
        self.decision == 1
    }
}