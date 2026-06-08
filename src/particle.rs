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
    pub particle_name: String,
    /// Antiparticle name, or None if the particle is its own antiparticle (or self-conjugate)
    pub antiparticle_name: Option<String>,
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

impl ParticleEntry {
    /// Name for given PDG id
    pub fn name(&self, id: i32) -> &str {
        if id >= 0 {
            &self.particle_name
        } else {
            match &self.antiparticle_name {
                Some(name_exists) => name_exists, // distinct antiparticle
                None => &self.particle_name,      // self-conjugate particle
            }
        }
    }
    /// Electric charge in units of e (e.g. -1.0 for electron, +2/3 for up quark)
    pub fn charge(&self) -> f64 {
        self.charge_type as f64 / 3.0
    }

    /// True if particle has a distinct antiparticle
    pub fn has_anti(&self) -> bool {
        self.antiparticle_name.is_some()
    }

    /// Whether to use Breit-Wigner mass distribution for this particle (true if width > 0)
    pub fn use_breit_wigner(&self) -> bool {
        self.m_width > 0.0
    }

    /// Whether this particle can decay
    pub fn can_decay(&self) -> bool {
        !self.channels.is_empty() || self.m_width > 0.0
    }

    pub fn is_quark(&self) -> bool {
        self.id >= 1 && self.id <= 8
    }
    pub fn is_lepton(&self) -> bool {
        self.id >= 11 && self.id <= 18
    }
    pub fn is_gluon(&self) -> bool {
        self.id == 21
    }
}
