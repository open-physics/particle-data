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

    // loop over particles and print their properties
    for p in particles.iter().take(2) {
        println!(
            "PDG ID: {:?}, MC ID: {:?}, Name: {:?}, Charge: {:?}, Mass: {:?}, Width: {:?}, 
            Description: {:?},Particle Class: {:?}, Quantum Numbers: I={:?}, G={:?}, J={:?}, P={:?}, C={:?}, Summary={:?}",
            p.pdgid,
            p.mcid,
            p.name,
            p.charge,
            p.mass(),
            p.width(),
            p.description,
            p.particle_class,
            p.quantum_i,
            p.quantum_g,
            p.quantum_j,
            p.quantum_p,
            p.quantum_c,
            p.quantum_summary(),
        )
    }
    // received output
    /*
        Found 361 particles
    PDG ID: "B010", MC ID: Some(12214), Name: "Delta(1700)+", Charge: Plus,
    Mass: Ok(
            Some(
                DataEntry {
                    db: Pdg {
                        conn: Connection {
                            path: Some("/../org.pdg-rs.pdg-rs/pdgall-2025-v0.2.2.sqlite")
                        }
                    },
                    pdgid: "B010M",
                    edition: "2025",
                    value_type: Estimate,
                    in_summary_table: true,
                    confidence_level: None,
                    limit_type: Some(Range),
                    comment: None,
                    value: Some(1710.0),
                    value_text: Some("1690 to 1710 to 1730"),
                    error_positive: Some(20.0),
                    error_negative: Some(20.0),
                    scale_factor: None,
                    unit_text: "MeV",
                    display_value_text: "1690 to 1730 (~1710)",
                    display_power_of_ten: 0,
                    display_in_percent: false,
                    sort: Some(1)
                }
            )
        ),
    Width: Ok(
            Some(
                DataEntry {
                    db: Pdg {
                        conn: Connection {
                            path: Some("//org.pdg-rs.pdg-rs/pdgall-2025-v0.2.2.sqlite")
                        }
                    },
                    pdgid: "B010W",
                    edition: "2025",
                    value_type: Estimate,
                    in_summary_table: true,
                    confidence_level: None,
                    limit_type: Some(Range),
                    comment: None,
                    value: None,
                    value_text: Some("220 to 300 to 380"),
                    error_positive: None,
                    error_negative: None,
                    scale_factor: None,
                    unit_text: "MeV",
                    display_value_text: "220 to 380 (~300)",
                    display_power_of_ten: 0,
                    display_in_percent: false,
                    sort: Some(1)
                }
            )
        ),
    Description: "Delta(1700)",
    Particle Class: Baryon,
    Quantum Numbers: I=Some(I3), G=None, J=Some(J3), P=Some(Minus), C=None, Summary="Q=+1, I=3/2, J=3/2, P=-"
    */
}
