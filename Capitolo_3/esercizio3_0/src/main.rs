/* Il programma riprende l'esercizio 2.1 implementando le tre funzioni come 
funzione template  */

use std::fs::File;
use std::io::{ BufReader, BufRead, Result };
use std::str::FromStr;
use std::fmt::Debug;
use num_traits::{ Float, FromPrimitive }; // FromPrimitive per NumCast

use std::ops::Sub;

fn read_from_file<T>(file_path: &str) -> Result<Vec<T>>
    where
        T: FromStr,
        T::Err: Debug // Questo vincolo è necessario per usare unwrap o per gestire l'errore di parsing
{
    let file = File::open(file_path)?; //propago gli errori con (?)
    /* ? Operatore:
    -Se il risultato di File::open(file_path) è Ok(file), il file viene
    assegnato alla variabile file.
    -Se il risultato è Err(error), l'operatore ? propaga immediatamente l'errore, 
    facendo terminare la funzione e restituendo l'errore. */

    let reader = BufReader::new(file);
    let numbers: Vec<T> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Errore nella lettura della riga");
            line.parse::<T>().expect("Errore nel parsing della riga")
        })
        .collect();

    Ok(numbers)
}

fn evaluate_median<T>(data: &mut Vec<T>) -> Result<T>
    where T: PartialOrd + Copy + std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<f32>
{
    // Ordina il vettore in-place
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = data.len() / 2;

    // Se la lunghezza è dispari, restituisce l'elemento centrale
    // Se la lunghezza è pari, restituisce la media dei due elementi centrali
    if data.len() % 2 == 0 {
        // Calcolo della media: usiamo `From<f32>` per conversioni se necessario
        let lower = data[mid - 1];
        let upper = data[mid];
        let median = (lower + upper) / T::from(2.0); // Conversione da f32 a T
        Ok(median)
    } else {
        Ok(data[mid])
    }
}

/* fn evaluate_std<T>(data: &Vec<T>) -> Result<T>
where
    T: PartialOrd + Copy + std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<f32> + Float + FromPrimitive, Option<T>: Sub,
{
    let mean = evaluate_mean(data);
    let mut sum = T::from(0.0 as f32); // Inizializza la somma a zero
    let len = T::from(data.len() as f32); // Converti la lunghezza in T

    // Calcola la somma dei quadrati delle differenze rispetto alla media
    for &value in data.iter() {
        let diff = value - mean;
        sum = sum + diff * diff;
    }

    // Deviazione standard: prendi la radice quadrata della somma divisa per (n-1)
    let len_minus_one = len - T::from(1.0 as f32);
    let dev_std = (sum / len_minus_one).sqrt();

    Ok(dev_std)
} */

fn evaluate_mean<T>(data: &Vec<T>) -> Result<T>
    where T: PartialOrd + Copy + std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<f32>
{
    let len = T::from(data.len() as f32); // Converti la lunghezza in T
    let mut sum = T::from(0.0); // Inizializza la somma come 0.0, convertito in T

    for &item in data.iter() {
        sum = sum + item;
    }

    let mean = sum / len; // Calcola la media
    Ok(mean)
}

fn main() {
    let file = String::from("./src/data.dat");
    let mut _datas = match read_from_file::<f32>(&file) {
        Ok(data) => data, // Qui estraiamo il Vec<f32> dall'Ok
        Err(e) => {
            eprintln!("Errore nella lettura del file: {}", e);
            return; // Uscita dal programma in caso di errore
        }
    };
    println!("Vettore contenente i dati {:?}", _datas);
    match evaluate_mean(&_datas) {
        Ok(mean) => println!("La media è = {}", mean),
        Err(e) => eprintln!("Errore: {}", e),
    }
    // Calcola la mediana
    match evaluate_median(&mut _datas) {
        Ok(median) => println!("La mediana è: {}", median),
        Err(e) => eprintln!("Errore: {}", e),
    }
/*     match evaluate_std(&mut _datas) {
        Ok(dev_Std) => println!("La deviazione standard è: {}", dev_Std),
        Err(e) => eprintln!("Errore: {}", e),
    } */
}
