use pdg_rs::{AngularMomentum, Charge, ParticleSearchQuery, ParticleType, Pdg};

fn charge_type(charge: Charge) -> i32 {
    match charge {
        Charge::PlusPlus => 6,
        Charge::Plus => 3,
        Charge::PlusTwoThirds => 2,
        Charge::PlusOneThird => 1,
        Charge::Neutral => 0,
        Charge::MinusOneThird => -1,
        Charge::MinusTwoThirds => -2,
        Charge::Minus => -3,
        Charge::MinusMinus => -6,
    }
}

fn total_angular_momentum_type(ang_mom: Option<AngularMomentum>) -> i32 {
    match ang_mom {
        Some(AngularMomentum::J0) => 1,
        Some(AngularMomentum::J1) => 2,
        Some(AngularMomentum::J2) => 3,
        Some(AngularMomentum::J3) => 4,
        Some(AngularMomentum::J4) => 5,
        Some(AngularMomentum::J5) => 6,
        Some(AngularMomentum::J6) => 7,
        Some(AngularMomentum::J7) => 8,
        Some(AngularMomentum::J8) => 9,
        Some(AngularMomentum::J9) => 10,
        Some(AngularMomentum::J10) => 11,
        Some(AngularMomentum::J11) => 12,
        Some(AngularMomentum::J12) => 13,
        Some(AngularMomentum::J13) => 14,
        Some(AngularMomentum::J14) => 15,
        Some(AngularMomentum::J15) => 16,
        _ => 0,
    }
}

fn main() {
    // 1. Get particles from PDG database
    let db = Pdg::open().expect("failed to open PDG database");
    let mut particles = Vec::new();
    for particle_type in [ParticleType::Particle, ParticleType::SelfConjugate] {
        let query = ParticleSearchQuery::new().particle_type(particle_type);
        let results = db.search_particles(query).expect("search failed");
        particles.extend(results);
    }
    particles.retain(|p| p.mcid.is_some()); // keep particles with a valid MC ID only
    println!("Found {} particles", particles.len());
}
