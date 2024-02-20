/*
    Escribir un programa en rust que utilice un modelo de 
    actores para implementar un programa que permita contar 
    la cantidad de lineas de cada uno de los archivos de un 
    directorio de file system y con esas cantidades generar el 
    valor totalizador
*/
extern crate actix;
use actix::{Actor, Context, Handler, System, Message, Addr};
use std::fs::{File};
use std::io::{BufReader};
use std::io::prelude::*;
use std::io::Read;

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

struct Totalizator{
    total_lines: u32,
    remaining_files: usize
}

struct FileCounter {
    totalizator : Addr<Totalizator>
}

#[derive(Message)]
#[rtype( result = "Result<(),()>")]
struct CountLines{
    file : File
}

#[derive(Message)]
#[rtype( result = "Result<(),()>")]
struct SumLines{
    amount_lines : u32
}

impl Actor for Totalizator{
    type Context = Context<Self>;
}

impl Actor for FileCounter{
    type Context = Context<Self>;
}

impl Handler<CountLines> for FileCounter{
    type Result = Result<(),()>;
    fn handle(&mut self, _msg: CountLines, _cnx: &mut Context<Self>) -> Self::Result {
        let mut file_aux = BufReader::new(_msg.file);
        let mut cnt = 0;
        for _ in file_aux.by_ref().lines() {
            cnt = cnt + 1;
        }
        println!("Cantidad leida de este archivo {:?}", cnt);
        self.totalizator.do_send(SumLines{amount_lines: cnt});
        Ok(())
    }
}  

impl Handler<SumLines> for Totalizator{
    type Result = Result<(),()>;
    fn handle(&mut self, _msg: SumLines, _cnx: &mut Context<Self>)-> Self::Result {
        let amount = _msg.amount_lines;
        self.total_lines += amount;
        self.remaining_files -= 1;
        if self.remaining_files == 0{
            println!("[Totalizador] cantidad total de lineas es {:?}", self.total_lines);
        }
        Ok(())
    }
}


fn main(){
    let system = System::new();
    let files = get_archivos();
    system.block_on(async {
        let totalizador = Totalizator{total_lines: 0, remaining_files: files.len()}.start();

        for file in files {
            let totalizador_clone = totalizador.clone();
            FileCounter{totalizator: totalizador_clone}.start().do_send(CountLines{file});
        }
    });
    system.run().unwrap();
}