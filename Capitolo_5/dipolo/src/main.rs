/* il programma implementa una struttura per la preparazione di una simulazione con
la la particella elettrone. la struttura ospita carica e massa di un elettrone 
il programma segue implementazione per calcolare il campo elettrico prodotto da un dipolo
e la forza associata 
*/

/* inizializzazione costanti utilizzate nei calcoli */
const MASS_E: f32 = 9.11e-31;
const CHARGE_E: f32 = -1.6e-19;
const MASS_P: f32 = 1.67e-27;
const CHARGE_P: f32 = 1.6e-19;
const EPS0: f32 = 8.854e-10;

#[derive(Clone)]
struct Posizione {
    x: f32,
    y: f32,
    z: f32,
}

struct Elettrone {
    massa: f32,
    carica: f32,
    posizione: Posizione,
}
struct Protone {
    massa: f32,
    carica: f32,
    posizione: Posizione,
}

struct Dipolo {
    protone: Protone,
    elettrone: Elettrone,
}

impl Posizione {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Posizione { x, y, z }
    }
    fn get_x(&mut self) -> f32 {
        self.x
    }
    fn get_y(&mut self) -> f32 {
        self.y
    }
    fn get_z(&mut self) -> f32 {
        self.z
    }
    fn get_r(&mut self) -> f32 {
        let r = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        r
    }

    fn distanza(&mut self, q: &mut Posizione) -> f32 {
        let x_diff = self.x - q.get_x();
        let y_diff = self.y - q.get_y();
        let z_diff = self.z - q.get_z();

        let distanza = (x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)).sqrt();
        distanza
    }
}

impl Elettrone {
    fn new(massa: f32, carica: f32, posizione: Posizione) -> Self {
        Elettrone { massa, carica, posizione }
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
    fn get_posizione(&mut self) -> &mut Posizione {
        let mut pos = &mut self.posizione;
        pos
    }
    fn print_posizione(&mut self) {
        let posizione: &mut Posizione = self.get_posizione();
        println!(
            "Posizione dell'elettrone = ({}, {}, {})",
            posizione.get_x(),
            posizione.get_y(),
            posizione.get_z()
        );
    }
}

impl Protone {
    fn new(massa: f32, carica: f32, posizione: Posizione) -> Self {
        Protone { massa, carica, posizione }
    }
    fn get_massa(&self) -> f32 {
        let massa_protone = self.massa;
        massa_protone
    }
    fn get_carica(&self) -> f32 {
        let carica_protone = self.carica;
        carica_protone
    }
    fn print_massa(&self) -> f32 {
        let mass = self.get_massa();
        println!("Massa del protone = {}", mass);
        mass
    }
    fn print_carica(&self) -> f32 {
        let charge = self.get_carica();
        println!("Carica del protone = {}", charge);
        charge
    }
    fn get_posizione(&mut self) -> &mut Posizione {
        let mut pos = &mut self.posizione;
        pos
    }
    fn print_posizione(&mut self) {
        let posizione: &mut Posizione = self.get_posizione();
        println!(
            "Posizione del protone = ({}, {}, {})",
            posizione.get_x(),
            posizione.get_y(),
            posizione.get_z()
        );
    }
}

impl Dipolo {
    fn new(protone: Protone, elettrone: Elettrone) -> Self {
        Dipolo { protone, elettrone }
    }

    // Calcola la distanza tra elettrone e protone
    fn calcola_distanza(&mut self) -> f32 {
        self.protone.posizione.distanza(&mut self.elettrone.posizione)
    }

    // Calcola il momento di dipolo elettrico: p = q * r
    fn calcola_dipolo(&mut self) -> (f32, f32, f32) {
        let distanza = self.calcola_distanza();
        let q = self.elettrone.carica; // La carica dell'elettrone o del protone (uguale in modulo)

        let dx = self.elettrone.posizione.x - self.protone.posizione.x;
        let dy = self.elettrone.posizione.y - self.protone.posizione.y;
        let dz = self.elettrone.posizione.z - self.protone.posizione.z;

        // momento di dipolo per ciascun asse
        let p_x = q * dx;
        let p_y = q * dy;
        let p_z = q * dz;

        println!("Momento di dipolo: ({}, {}, {})", p_x, p_y, p_z);
        (p_x, p_y, p_z)
    }

    // Calcola la forza elettrostatica usando la legge di Coulomb: F = k * q1 * q2 / r^2
    fn calcola_forza(&mut self) -> f32 {
        let distanza = self.calcola_distanza();
        let forza =
            ((1.0 / (4.0 * std::f32::consts::PI * EPS0)) *
                (self.elettrone.carica * self.protone.carica)) /
            distanza.powi(2);

        println!("Forza elettrostatica tra elettrone e protone: {} N", forza);
        forza
    }
}

fn main() {
    let pos_e = Posizione::new(10.02, 1.4, 2.0);
    let pos_p = Posizione::new(1.5, 5.3, 2.4);
    let mut elettrone = Elettrone::new(MASS_E, CHARGE_E, pos_e);
    let mut protone = Protone::new(MASS_P, CHARGE_P, pos_p);

    elettrone.print_carica();
    elettrone.print_massa();
    elettrone.print_posizione();
    protone.print_carica();
    protone.print_massa();
    protone.print_posizione();

    let mut dipolo = Dipolo::new(protone, elettrone);

    dipolo.calcola_dipolo();
    dipolo.calcola_forza();
}
