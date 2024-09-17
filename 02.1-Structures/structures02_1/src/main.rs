/* Il programma implementa delle semplici operazioni sui vettori
come calcoli di media, mediana,deviazione standard*/

use std::{ fs::File, io::{ BufRead, BufReader } };

fn read_from_file(file_path: &str) -> Vec<f32> {
    let file = File::open(file_path).expect("impossibile aprire il file");
    let reader = BufReader::new(file);

    let _numbers: Vec<f32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<f32>().unwrap())
        .collect();

    _numbers
}

fn evaluate_median(data: &mut Vec<f32>) -> f32 {
    // Ordina il vettore in-place
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = data.len() / 2;

    // Se la lunghezza è dispari, restituisce l'elemento centrale
    // Se la lunghezza è pari, restituisce la media dei due elementi centrali
    if data.len() % 2 == 0 {
        (data[mid - 1] + data[mid]) / 2.0
    } else {
        data[mid]
    }
}

fn evaluate_std(data: &Vec<f32>) -> f32 {
    let dev_std: f32;
    let mean: f32 = evaluate_mean(data);
    let mut sum: f32 = 0.0;
    let arr_len: usize = data.len();
    for i in 0..arr_len {
        sum += (data[i as usize] - mean) * (data[i as usize] - mean);
    }
    dev_std = (sum / ((arr_len as f32) - (1 as f32))).sqrt() as f32;
    dev_std
}

fn evaluate_mean(data: &Vec<f32>) -> f32 {
    let mut _mean: f32 = 0.0;
    let len: usize = data.len();

    let mut sum: f32 = 0.0;
    for i in 0..len {
        sum +=
            data
                [
                    i as usize
                ]; /* il tipo usize è un unsigned int. Lo utilizzo per indicizzare gli elementi del vettore */
    }
    _mean = ((sum as usize) / len) as f32;
    _mean
}

fn main() {
    let file = String::from("./src/data.dat");
    let mut _datas = read_from_file(&file);
    println!("Vettore contenente i dati {:?}", _datas);
    let _data_mean = evaluate_mean(&_datas);
    println!("Media dei valori = {:?}", _data_mean);
    let _median = evaluate_median(&mut _datas);
    println!("Mediana dei valori = {:?}", _median);
    let _dev_std: f32 = evaluate_std(&_datas);
    println!("Deviazione standard dei valori = {:?}", _dev_std);
}
