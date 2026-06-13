use pdg_rs::{Charge, Pdg};

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
