// vault.rs — Adaptive normalcy model (The Vault)
// License: GPLv3
// Status: PROTOTYPE — transport stub only

use std::collections::HashMap;
use crate::protocol::SignalArythmie;

const ADAPTATION_FACTOR: f32 = 0.05;
const CONFIANCE_MINIMALE: usize = 100;

#[derive(Debug, Clone)]
pub struct PatternNormalite {
    pub uid: u32,
    pub intervalle_cible_ms: u32,
    pub volume_cible_octets: u32,
    pub alertes_recentes: usize,
    pub cycles_valides: usize,
}

pub struct TheVault {
    pub registre: HashMap<u32, PatternNormalite>,
}

impl TheVault {
    pub fn new() -> Self {
        let mut v = Self { registre: HashMap::new() };
        v.registre.insert(1042, PatternNormalite {
            uid: 1042,
            intervalle_cible_ms: 1000,
            volume_cible_octets: 256,
            alertes_recentes: 0,
            cycles_valides: 0,
        });
        v
    }

    pub fn evaluer(&mut self, uid: u32, score: u8, volume: u32) {
        let p = match self.registre.get_mut(&uid) {
            Some(p) => p,
            None => return,
        };

        if score > 75 {
            p.alertes_recentes += 1;
            p.cycles_valides = 0;
            eprintln!("[VAULT] Critical alert UID {} hostility={}",
                      uid, p.alertes_recentes);
            return;
        }

        p.cycles_valides += 1;

        if p.cycles_valides >= CONFIANCE_MINIMALE
            && p.alertes_recentes == 0
        {
            let delta = volume as f32 - p.volume_cible_octets as f32;
            p.volume_cible_octets =
                (p.volume_cible_octets as f32
                 + delta * ADAPTATION_FACTOR) as u32;
            p.cycles_valides = 0;
            println!("[VAULT] Model updated UID {} new_volume={}",
                     uid, p.volume_cible_octets);
            // TODO: push to edge via transport (not implemented)
        }
    }
}