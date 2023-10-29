extern crate rand;
extern crate std_semaphore;


use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use rand::{thread_rng};
use rand::seq::SliceRandom;
use std_semaphore::Semaphore;


const N:usize = 3;


const FUMADORES: i32 = 3;
fn main(){

    let agente_semaforo = Arc::new(Semaphore::new(1));
    let par_ingredientes_sem = Arc::new(Semaphore::new(0));
    let agente_semaforo_clone = agente_semaforo.clone();
    let par_ingredientes_sem_clone = par_ingredientes_sem.clone();
    /*
    let ingredientes_agentes = ingredientes.clone();
    let agent = thread::spawn(move || procesar_agente(agente_semaforo_clone, par_ingredientes_sem_clone,  ingredientes));

    let mut fumadores = vec![];

    for fumador_id in 0..FUMADORES {
        let par_ingredientes_fumador = par_ingredientes_sem.clone();
        let agente_sem_fumador = agente_semaforo.clone();
        let ingredientes_fumadores = ingredientes.clone();
        fumadores.push(thread::spawn(move || procesar_fumador(fumador_id, agente_sem_fumador, par_ingredientes_fumador,  ingredientes)));
    }


    for fumador in fumadores {
        fumador.join().unwrap();
    }

    agent.join().unwrap();
    */
}
/*
fn procesar_agente(agente_semaforo: Arc<Semaphore>, par_ingredientes_sem: Arc<Semaphore>, ingredientes_agentes:  Arc<RwLock<[]>>){
    loop {
        agente_semaforo.acquire();
        let mut ings = [String::from("TABACO"), String::from("PAPEL"), String::from("FUEGO")];
        ings.shuffle(&mut thread_rng());
        let selected_ings = &ings[0..N-1];
        let ingredientes = &ings[0..N-1];
        par_ingredientes_sem.release();
    }
}

fn procesar_fumador(fumador_id: i32, agente_semaforo: Arc<Semaphore>, par_ingredientes_sem: Arc<Semaphore>,ingredientes_agentes: Arc<RwLock<Vec<str>>>){
    let  ings = [String::from("TABACO"), String::from("PAPEL"), String::from("FUEGO")];
    let ingrediete_fumador = &ings[fumador_id as usize];
    println!("[Fumador {:?}]", ingrediete_fumador);
    if !ingredientes_agentes.contains(&ingrediete_fumador){
        par_ingredientes_sem.acquire();
        println!("[Fumador {:?}] Obtuve {:?}", ingrediete_fumador, ingredientes_agentes);
    }
    println!("[Fumador {:?}] Fumando", ingrediete_fumador);
    thread::sleep(Duration::from_secs(2));
    agente_semaforo.release();
    println!("[Fumador {:?}] Terminé", ingrediete_fumador);
}
*/

