
use std::fs::{File};
use std::io::{BufReader};
use std::io::prelude::*;
use std::thread;



fn main(){

    let file = File::open("/Users/agust/concu-2023/concu/fork-join/src/data/example1.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let file_lines = reader.lines();
    let mut hilos = vec![];
    for line in file_lines{
        hilos.push(thread::spawn(move || {
            let aux = line.unwrap().chars().count();
            println!("Cantidad caracteres: {:?}", aux);
            aux
        }));
    }
    let mut cantidad_caracteres = 0;
    for hilo in hilos{
        cantidad_caracteres += hilo.join().unwrap();
    }
    println!("Cantidad total de caracteres: {:?}", cantidad_caracteres);
}