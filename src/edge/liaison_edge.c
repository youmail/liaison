/*
 * liaison_edge.c — The Gate (Filtering Membrane)
 * Initial Gravity paradigm — first implementation
 * License: GPLv3
 * Status: PROTOTYPE — see LIMITATIONS.md
 */

#include <stdint.h>

#define LIAISON_MAX_UIDS   16
#define LIAISON_ALPHA_SHIFT 3
#define SCORE_CRITIQUE     75
#define SCORE_MARQUAGE     30

typedef enum {
    FIBRE_NEUTRE = 0,
    MARQUAGE     = 1,
    GUILLOTINE   = 2
} DecisionMembrane;

typedef struct {
    uint32_t uid;
    uint32_t frequence_attendue_ms;
    uint32_t volume_attendu_octets;
} ProfilNormalite;

typedef struct {
    uint32_t uid;
    uint32_t dernier_timestamp_ms;
    uint32_t moyenne_intervalle_ewma;
    uint32_t moyenne_volume_ewma;
} EtatOrgane;

static ProfilNormalite vault_registre[LIAISON_MAX_UIDS];
static EtatOrgane      scope_memoire[LIAISON_MAX_UIDS];
static uint8_t         uids_enregistres = 0;

void liaison_initialiser_profil(uint32_t uid,
                                 uint32_t freq_ms,
                                 uint32_t vol_octets) {
    if (uids_enregistres >= LIAISON_MAX_UIDS) return;
    /* TODO: guard freq_ms > 0 and vol_octets > 0 */
    vault_registre[uids_enregistres].uid = uid;
    vault_registre[uids_enregistres].frequence_attendue_ms = freq_ms;
    vault_registre[uids_enregistres].volume_attendu_octets = vol_octets;
    scope_memoire[uids_enregistres].uid = uid;
    scope_memoire[uids_enregistres].dernier_timestamp_ms = 0;
    scope_memoire[uids_enregistres].moyenne_intervalle_ewma = freq_ms;
    scope_memoire[uids_enregistres].moyenne_volume_ewma = vol_octets;
    uids_enregistres++;
}

DecisionMembrane liaison_evaluer_paquet(uint32_t uid,
                                         uint32_t timestamp_ms,
                                         uint32_t taille_octets) {
    int32_t idx = -1;
    for (uint8_t i = 0; i < uids_enregistres; i++) {
        if (scope_memoire[i].uid == uid) { idx = i; break; }
    }
    if (idx == -1) return GUILLOTINE;

    EtatOrgane *scope = &scope_memoire[idx];
    ProfilNormalite *vault = &vault_registre[idx];

    /* TODO: handle timestamp wraparound after 49 days */
    uint32_t intervalle = (scope->dernier_timestamp_ms == 0)
        ? vault->frequence_attendue_ms
        : (timestamp_ms - scope->dernier_timestamp_ms);

    scope->moyenne_intervalle_ewma = (uint32_t)(
        (int32_t)scope->moyenne_intervalle_ewma +
        (((int32_t)intervalle -
          (int32_t)scope->moyenne_intervalle_ewma)
         >> LIAISON_ALPHA_SHIFT));

    scope->moyenne_volume_ewma = (uint32_t)(
        (int32_t)scope->moyenne_volume_ewma +
        (((int32_t)taille_octets -
          (int32_t)scope->moyenne_volume_ewma)
         >> LIAISON_ALPHA_SHIFT));

    scope->dernier_timestamp_ms = timestamp_ms;

    /* TODO: cast to uint64_t to avoid overflow */
    uint32_t score = 0;
    if (scope->moyenne_intervalle_ewma < vault->frequence_attendue_ms) {
        uint32_t d = vault->frequence_attendue_ms
                   - scope->moyenne_intervalle_ewma;
        score += (d * 50) / vault->frequence_attendue_ms;
    }
    if (scope->moyenne_volume_ewma > vault->volume_attendu_octets) {
        uint32_t d = scope->moyenne_volume_ewma
                   - vault->volume_attendu_octets;
        score += (d * 50) / vault->volume_attendu_octets;
    }

    if (score > SCORE_CRITIQUE) return GUILLOTINE;
    if (score > SCORE_MARQUAGE) return MARQUAGE;
    return FIBRE_NEUTRE;
}