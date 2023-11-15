
use std::fs::{File};
use std::io::{BufReader};
use std::io::prelude::*;
use std::thread;


fn get_archivos() -> Vec<File>{
    let result = std::fs::read_dir("/Users/agust/concu-2023/concu/fork-join/src/data").unwrap(); // leo los archivos de un directorio
    let mut files = vec![];
    for r in result{
        let path = r.unwrap().path();
        let file = File::open(path).expect("Unable to open file");
        files.push(file);
    }
    return files;
}

fn count_lines(file: File){
    let mut file_aux = BufReader::new(file);
    let mut cnt = 0;
    for _ in file_aux.by_ref().lines() {
        cnt = cnt + 1;
    }
    println!("Cantidad leida de este archivo {:?}", cnt);
}

fn main(){

    let files = get_archivos();
    let mut clientes = vec![];
    for file in files {
        clientes.push(thread::spawn(move || count_lines(file)));
    }

    for client in  clientes{
        let _ = client.join();
    }

}