/* Implementazione di della struttura Posizione che imlementa 
la posizione di una particella in coordinate cartesiane,polari e sferiche */


struct Posizione {
    x: f32,
    y: f32,
    z: f32,
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
    fn get_theta(&mut self) -> f32 {
        let theta = (self.z / self.get_r()).acos();
        theta
    }
    fn get_phi(&mut self) -> f32 {
        let phi = self.y.atan2(self.x);
        phi
    }

    fn distanza(&mut self, q: &mut Posizione) -> f32 {
        let x_diff = self.x - q.get_x();
        let y_diff = self.y - q.get_y();
        let z_diff = self.z - q.get_z();

        let distanza = (x_diff.powi(2)+y_diff.powi(2)+z_diff.powi(2)).sqrt();
        distanza
    }
}
fn main() {
    println!("Hello World!");
    let mut particella = Posizione::new(10.0, 1.3, 3.5);
    println!("coordinata x della particella {}", particella.get_x());
    println!("coordinata y della particella {}", particella.get_y());
    println!("coordinata z della particella {}", particella.get_z());

    println!("-----------------------------------------------------");
    println!("Coordinate sferiche");
    println!("coordinata r della particella {}", particella.get_r());
    println!("coordinata theta della particella {}", particella.get_theta());
    println!("coordinata phi della particella {}", particella.get_phi());

    println!("-----------------------------------------------------");
    println!("Coordinate cilindriche");
    println!("coordinata rho della particella {}", particella.get_r());
    println!("coordinata theta della particella {}", particella.get_theta());

    let mut particella2 = Posizione::new(1.,1.,1.);
    println!("coordinata x della particella {}", particella2.get_x());
    println!("coordinata y della particella {}", particella2.get_y());
    println!("coordinata z della particella {}", particella2.get_z());
    println!("Distanza tra le due particelle {}",particella.distanza(&mut particella2))
}
