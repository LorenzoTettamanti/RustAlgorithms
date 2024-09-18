/* Semplice struttura per rappresentare un punto nel piano cartesiano
con implementazione di un semplice algoritmo per il calcolo 
della distanza euclidea tra due punti */

/* la struttura implementa le coordinate del punto con float32.
Successivamente vengono create funzioni che utilizzano la struttura */

struct Point {
    x: f32,
    y: f32,
}

fn get_coords(p: &Point) -> (f32, f32) {
    (p.x, p.y) /* restituisce una tupla contenente le coordinate del punto */
}

fn set_coords(p: &mut Point, tuple: (f32, f32)) {
    let (a, b) = tuple;
    p.x = a;
    p.y = b;
}

fn eval_distance(p: &Point, q: &Point) -> f32 {
    let x_diff = p.x - q.x;
    let y_diff = p.y - q.y;
    let dist = (x_diff * x_diff + y_diff * y_diff).sqrt();
    dist /* restituisce un float32 che indica la distanza */
}

fn main() {
    let mut point: Point = Point { x: 10.0, y: 2.0 };
    let mut _coordinate1 = get_coords(&point);
    println!("coordinate del primo punto : {:?}", _coordinate1);
    println!("Cambio di coordinate del primo punto in '(2.1,2.1)'");
    set_coords(&mut point, (2.1, 2.1));
    _coordinate1 = get_coords(&point);
    println!("coordinate del primo punto dopo modifica : {:?}", _coordinate1);
    println!("\nGenerazione di un nuovo punto");
    let mut point2: Point = Point { x: 2.0, y: 1.0 };
    let mut _coordinate2 = get_coords(&point2);
    println!("coordinate del secondo punto : {:?}", _coordinate2);
    let distanza = eval_distance(&mut point, &mut point2);
    println!("Distanza {}", distanza);
}
