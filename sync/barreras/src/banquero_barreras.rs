
extern crate rand;

use std::sync::{Arc, RwLock, Barrier};
use std::thread;
use std::time::Duration;
use rand::Rng;

/*
Han habido algunos problemas de organización, y los hijos del señor banquero, si
bien desean que los inversores sigan trabajando de forma autónoma, deben
tomar el dinero de la cuenta y devolverlo al final de la semana, análogamente a
como se hacía cuando el señor banquero se encontraba vivo. Recordamos: cada
inversor debe tomar exáctamente el mismo dinero.
*/
const INVERSORES: i32 = 5;
const MONTO_INICIAL: f64 = 100000.0;
fn main() {
    
    let saldo = Arc::new(RwLock::new(MONTO_INICIAL));
    let everyone_finishes_investing = Arc::new(Barrier::new(INVERSORES as usize));
    let everyone_started = Arc::new(Barrier::new(INVERSORES as usize));

    let mut inversores = vec![];

    for id in 0..INVERSORES{
        let saldo_clone = saldo.clone();
        let everyone_finishes_investing_clone = everyone_finishes_investing.clone();
        let everyone_started_clone = everyone_started.clone();
        inversores.push(thread::spawn(move || invest(id, saldo_clone, everyone_finishes_investing_clone, everyone_started_clone)));
    }

    for friend in inversores {
        friend.join().unwrap();
    }
    
}


fn invest(id: i32, saldo: Arc<RwLock<f64>>, everyone_finishes_investing: Arc<Barrier>, everyone_started: Arc<Barrier>){

    let mut week = 0;
    while *saldo.read().unwrap() > 1.0{
        
    
        let load = *saldo.read().unwrap() / INVERSORES as f64;
        println!("inversor {} inicio semana {} plata {}", id, week, load);
         // espero a que todos lean lo mismo a invertir
        everyone_started.wait();

        // Tomo el dinero
        if let Ok(mut money_to_invest) = saldo.write() {
            *money_to_invest -= load;
        }

        week += 1;
        let mut rng = rand::thread_rng();
        let random_result: f64 = rng.gen();
        thread::sleep(Duration::from_millis((2000 as f64 * random_result) as u64));
        let earn = load * (random_result + 0.5);
        println!("inversor {} voy a devolver {}", id, earn);

        // guardo el dinero investido 

        if let Ok(mut money_to_invest) = saldo.write() {
            *money_to_invest += earn;
        }
    
        //espero a que todos terminen su semana
        everyone_finishes_investing.wait();
    }
   

}
