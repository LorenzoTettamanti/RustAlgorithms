/* Implementazione di vettori su rust 
con qualche semplice operazione (somma,sottrazione,prodotto,divisione) */

use std::vec;

use rand::Rng;

fn main() {
    let mut v: Vec<i8> = Vec::new();
    /* push singoli elementi */
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Inizializzazione vettore v {:?}", v);
    println!("-----------------------------------------------");

    let mut k = vec![1, 2, 3, 4];
    println!("inizializzazione vettore k tramite macro {:?}", k);
    println!("-----------------------------------------------");
    k.push(10);
    println!("aggiunta del valore 10 in coda al vettore k");
    println!("{:?}", k);
    k.remove(3);
    println!("rimozione del valore 4 dal vettore k ");
    println!("{:?}", k);
    k.pop();
    println!("rimozione del valore 10 in coda dal vettore k");
    println!("{:?}", k);
    println!("-----------------------------------------------");
    println!("push di componenti randomiche nel vettore k");
    let mut rng = rand::thread_rng();
    for _j in 0..k.len() {
        let num: i8 = rng.gen(); //i8 ---> non servono numeri troppo grandi
        k.push(num);
    }
    k.sort();
    println!("Sorting del vettore {:?}", k);
    println!("-----------------------------------------------");
    let mut u = Vec::new();

    /* WIKI: zip prende 2 iteratori e li combina in un singolo iteratore
    per iterare coppie di elementi*/
    println!("Rimozione di 3 componenti dal vettore k");
    k.pop();
    k.pop();
    k.pop();

    println!("Vettore k dopo la rimozione {:?}", k);
    for (a, b) in v.iter().zip(k.iter()) {
        u.push(a + b);
    }
    println!("-----------------------------------------------");
    println!("------------ O P E R A Z I O N I --------------");
    println!("-----------------------------------------------");
    println!("Somma di vettori v e k {:?} ", u);

    let mut l = Vec::new();
    for (a, b) in v.iter().zip(k.iter()) {
        l.push(a - b);
    }
    println!("Sottrazione di vettori v e k {:?} ", l);

    let mut m = Vec::new();
    for (a, b) in v.iter().zip(k.iter()) {
        m.push(a * b);
    }
    println!("Moltiplicazione di vettori v e k {:?} ", m);

    let mut n: Vec<i8> = Vec::new();
    for (a, b) in v.iter().zip(k.iter()) {
        n.push(a / b);
    }
    println!("Divisione di vettori v e k {:?} ", n);
    println!("\n");
    println!("La divisione è sbagliata perchè il tipo non lo permette");
    println!("i8 non permette di immagazzinare valori con decimale. Bisogna usare tipi float f32 o f64")
}
