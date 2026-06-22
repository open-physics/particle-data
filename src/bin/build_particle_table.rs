use pdg_rs::{AngularMomentum, Charge, Pdg};

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
    let db = Pdg::open().expect("failed to open PDG database");
    let electron = db.mcid(11).expect("db error").expect("electron not found");
    println!("Electron name: {:?}", electron.name);
    println!("mcid: {:?}", electron.mcid);
    println!("charge: {:?}", electron.charge);

    if let Ok(Some(mass)) = electron.mass() {
        println!("mass: {:?} GeV", mass.value);
    }
    if let Ok(bfs) = electron.branching_fractions() {
        println!("branching fractions: {}", bfs.len());
    }
}
