use pdg_rs::Pdg;

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
