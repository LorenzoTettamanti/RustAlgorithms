/* il programma implementa una struttura per la preparazione di una simulazione con
la la particella elettrone. la struttura ospita carica e massa di un elettrone */

struct Elettrone {
    massa: f32,
    carica: f32,
}
const MASS_E: f32 = 9.11e-31;
const CHARGE_E: f32 = 1.6e-19;

impl Elettrone {
    fn new(massa: f32, carica: f32) -> Self {
        Elettrone { massa, carica }
    }
    fn get_massa(&self) -> f32 {
        let massa_elettrone = self.massa;
        massa_elettrone
    }
    fn get_carica(&self) -> f32 {
        let carica_elettrone = self.carica;
        carica_elettrone
    }
    fn print_massa(&self) -> f32 {
        let mass = self.get_massa();
        println!("Massa dell'elettrone = {}", mass);
        mass
    }
    fn print_carica(&self) -> f32 {
        let charge = self.get_carica();
        println!("Carica dell'elettrone = {}", charge);
        charge
    }
}

fn main() {
    let elettrone = Elettrone::new(MASS_E, CHARGE_E);
    elettrone.print_carica();
    elettrone.print_massa();
}
