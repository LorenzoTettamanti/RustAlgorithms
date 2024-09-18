/* Il programma implementa delle semplici operazioni sui vettori
come calcoli di media, mediana,deviazione standard.
Implementa un grafico con plotters

*/

use plotters::prelude::*;
use std::fs::File;
use std::io::{ BufRead, BufReader };

const OUT_FILE_NAME: &str = "./histogram.png";
const NBINS: usize = 30;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = String::from("./src/data.dat");
    let mut _datas = read_from_file(&file);
    println!("Vettore contenente i dati {:?}", _datas);

    let _data_mean = evaluate_mean(&_datas);
    println!("Media dei valori = {:?}", _data_mean);

    let _median = evaluate_median(&mut _datas);
    println!("Mediana dei valori = {:?}", _median);

    let _dev_std: f32 = evaluate_std(&_datas);
    println!("Deviazione standard dei valori = {:?}", _dev_std);

    /* VISUALIZZAZIONE CON ISTOGRAMMA */
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Calcolo del range dei dati
    let min_data = *_datas
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);
    let max_data = *_datas
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&1.0);
    println!("Statistiche: ");
    println!("min data : {} ", min_data);
    println!("max data : {} ", max_data);
    println!("#bins : {} ", NBINS);
    // Creazione del grafico
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d(
            (0..NBINS).into_segmented(), // Intervallo discreto per i bin
            0u32.._datas.len() as u32 // Intervallo per l'asse y
        )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bin")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    // Creazione dei bin per l'istogramma
    let bin_width = (max_data - min_data) / (NBINS as f32);
    println!("bin width: {}", bin_width);
    let mut histogram = vec![0u32; NBINS];

    // Itera sui dati per popolare l'istogramma
    for &value in &_datas {
        let mut bin_index = ((value - min_data) / bin_width) as usize;

        // Assegna i valori al bin corretto
        if bin_index >= NBINS {
            bin_index = NBINS - 1; // Se il valore è al massimo, mettilo nell'ultimo bin
        }

        histogram[bin_index] += 1;
    }

    println!("istogramma: {:?}", histogram);

    // Disegno dell'istogramma
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(
                histogram
                    .iter()
                    .enumerate()
                    .map(|(i, &count)| {
                        (i, count) // Usa bin index come x-coordinate
                    })
            )
    )?;

    println!("Istogramma salvato in histogram.png");

    Ok(())
}
