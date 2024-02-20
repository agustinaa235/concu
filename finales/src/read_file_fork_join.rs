
/*
2da fecha diciembre 2022
    Hacer un programa en Rust (pseudo código) que cuente las 
    líneas de todos los archivos de un directorio usando modelo 
    fork-join (sin usar rayon).
 */

 use std::fs::{File};
 use std::io::{BufReader};
 use std::io::prelude::*;
 use std::io::Read;
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

 fn count_lines(file: File) -> i32 {
    let mut file_aux = BufReader::new(file);
    let mut cnt = 0;
    for _ in file_aux.by_ref().lines() {
        cnt = cnt + 1;
    }
    println!("Cantidad leida de este archivo {:?}", cnt);
    cnt
}

fn main() {
    let archivos = get_archivos();
    let mut procesos = vec![];
    let mut final_lines :i32 = 0;
    for archivo in archivos {
        procesos.push(thread::spawn(move || count_lines(archivo)));
    }

    for process in procesos{
        match process.join() {
            Ok(lines) => final_lines += lines,
            Err(err) => eprintln!("Error joining thread: {:?}", err),
        }
    }

    println!("Cantidad de lineas totales es {:?}", final_lines);

}
