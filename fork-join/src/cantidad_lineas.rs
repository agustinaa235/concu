
use std::fs::{File};
use rayon::prelude::*;
use std::io::{BufReader};
use std::io::prelude::*;


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

fn process_lines(file: &File){
    let mut file_aux = BufReader::new(file);
    let mut cnt = 0;
    for _ in file_aux.by_ref().lines() {
        cnt = cnt + 1;
    }
    println!("Cantidad leida de este archivo {:?}", cnt);


}


fn main() {
    let archivos = get_archivos();
    println!("{:?}",archivos);
    let _aux = archivos.par_iter().for_each(|file| process_lines(file));
 
}

