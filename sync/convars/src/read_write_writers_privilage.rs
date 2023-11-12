

use std::sync::{Arc, Mutex,Condvar};
use std::thread;
use std::time::Duration;

const READERS : usize = 10;
const WRITERS : usize = 6;






/*
Le doy prioridad a los escritores por sobre los lectores, es decir ahora el lector debe esperar a leer si es que hay escritores
problema: Si hay muchso escritores lo lectores no leen 

*/
#[derive(Debug)]
struct ReadWrite{
    writers: usize,
    writing: bool, 
    readers: usize,
}

fn main (){
    
    let read_write_pair = Arc::new((Mutex::new(ReadWrite{writers:0, writing: false, readers: 0}), Condvar::new()));
    let mut readers = vec![];
    let mut writers = vec![];

    for reader_id in 0..=READERS{
        let read_write_readers = read_write_pair.clone();
        readers.push(thread::spawn(move || process_readers(reader_id, read_write_readers)));
    }

    for writer_id in 0..=WRITERS{
        let read_write_writers = read_write_pair.clone();
        writers.push(thread::spawn(move || process_writers(writer_id, read_write_writers)));
    }

    for reader in readers{
        reader.join();
    }

    for writer in writers{
        writer.join();
    }
    
}

fn process_readers(id: usize, read_write: Arc<(Mutex<ReadWrite>, Condvar)>){
    loop{
        let (mutex, cvar) = &*read_write;
        {   
            let mut _guard = cvar.wait_while(mutex.lock().unwrap(), |rw|{
                rw.writing || rw.writers > 0
            }).unwrap();
            _guard.readers += 1;
        }
        println!("Proceso {:?} empieza  a leer", id);
        thread::sleep(Duration::from_millis(1000));
        println!("proceso {:?} termina de leer", id);
        mutex.lock().unwrap().readers -= 1; 
        cvar.notify_all();
    }
}

fn process_writers(id: usize, read_write: Arc<(Mutex<ReadWrite>, Condvar)>){
    loop{
        let (mutex, cvar) = &*read_write;
        mutex.lock().unwrap().writers += 1;
        
        {   
            let mut _guard = cvar.wait_while(mutex.lock().unwrap(), |rw|{
                rw.writing || rw.readers > 0  // esperan si hay escritores escribiendo y si hay lectores
            }).unwrap();
            _guard.writing = true;
        }
        
        println!("Proceso {:?} empieza  a escribir", id);
        thread::sleep(Duration::from_millis(3000));
        println!("proceso {:?} termina de escribir", id);
        mutex.lock().unwrap().writing = false;
        mutex.lock().unwrap().writers -=1;
            
        cvar.notify_all();
    }
}
