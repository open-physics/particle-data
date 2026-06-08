// Core library file with some structs and a parser.

/// A single decay channel of a particle.
///
/// e.g. Z_zero -> muon + anti-muon, with br_ratio = 0.034.
#[derive(Debug, Clone)]
pub struct DecayChannel {
    /// 0 = off, 1 = on, 2 = on for particle only, 3 = on for antiparticle only
    pub on_mode: i32,
    /// Branching ratio - fraction of decays going through this channel (0.0 to 1.0)
    pub br_ratio: f64,
    /// Matrix element mode used by the decay engine (0 = deneric phase space
    pub me_mode: i32,
    /// PDG IDs of the decay products (negative for antiparticles)
    pub products: Vec<i32>,
}

/// Full static data for a single particle species
#[derive(Debug, Clone)]
pub struct ParticleEntry {
    /// PDG Monte Carlo ID (always positive)
    /// `id` is always positive - the antiparticle is represented by `-id`.
    pub id: i32,
    /// Particle name, e.g. "mu-", "Z0", "photon", "pi+"
    pub name: String,
    /// Antiparticle name, or None if the particle is its own antiparticle (or self-conjugate)
    pub anti_name: Option<String>,
    /// 2S+1: 1 = scalar (Higgs), 2 = fermion (quark, lepton), 3 = vector boson (W, Z, photon, gluon)
    pub spin_type: i32,
    /// Electric charge * 3, e.g. -3 for electron, +2 for up quark, 0 for photon
    /// Stored as integer to avoid floating point imprecision
    pub charge_type: i32,
    /// Colour charge type: 0 = colourless, 1 = triplet (quark), -1 = anti-triplet, 2 = octet (gluon)
    pub color_type: i32,
    /// Mass parameters for the particle.
    /// `m0` is the nominal mass, `m_min`, and `m_max` define the allowed mass range for off-shell decays.     
    pub m0: f64,
    /// Total decay width (decay rate) in GeV (0 = stable/prompt)
    pub m_width: f64,
    /// Lower Breit-Wigner cutoff in GeV
    pub m_min: f64,
    /// Upper Breit-Wigner cutoff in GeV
    pub m_max: f64,
    ///`tau0` is the proper lifetime of the particle.
    pub tau0: f64,
    /// Decay channels (empty for stable particles), sum of all br_ratios should be <= 1.0
    pub channels: Vec<DecayChannel>,
}
