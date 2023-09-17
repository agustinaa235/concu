extern crate rand;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
use rand::{Rng, thread_rng};
use std::collections::HashMap;

/* Realizar un pequeño idle game
    ● Algunas entidades "extraerán" (generarán aleatoriamente) oro
    ● Algunas entidades podrán convertir oro en recursos (a gusto)
    ● Otras entidades podrán convertir combinaciones de recursos en + oro
    ● Otras entidades podrán solamente consumir recursos
    ● Periódicamente se reporta por pantalla el nivel de recursos y oro
*/


const ENTIDADES: u32 = 10;


#[derive(Debug)]
struct Game {
    oro: f64,
    recursos: HashMap<String,i32>,
}

fn main() {
   
    let mut recursos = HashMap::new();
    recursos.insert(String::from("Hamburguesa"), 5);
    recursos.insert(String::from("Papas fritas"), 10);
    recursos.insert(String::from("Coca Cola"), 7);

    let game = Arc::new(RwLock::new(Game{oro: 100.0, recursos: recursos}));
    let mut entidades: Vec<JoinHandle<()>> =Vec::new();
    for id in 1..=ENTIDADES {
        let game_local = game.clone();
        let entidad =  thread::spawn(move || play_game(id, game_local));
        entidades.push(entidad);
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Estado del juego {:?}", game.read());
    }
     
    
    
}

fn play_game(id:u32, game_local:Arc<RwLock<Game>>){

    // id del rpdoucto y valor del producto
    let recursos = [ String::from("Hamburguesa"),  String::from("Papas fritas"),  String::from("Coca Cola")];
    let mut round = 0;
    while round < 5 {

        if let Ok(mut game) = game_local.write(){
            if  id == 2 || id == 5 {
                // extraen oro
                thread::sleep(Duration::from_millis(1000));
                let extraigo_oro = game.oro*thread_rng().gen_range(0.1, 0.5);
                game.oro -= extraigo_oro;
                println!("[Entidad {}] extraigo oro {}", id, extraigo_oro);
            } else if id == 1 || id == 3{
                // producen oro
                thread::sleep(Duration::from_millis(1000));
                let agrego_oro = game.oro*thread_rng().gen_range(0.1, 0.5);
                game.oro += agrego_oro;
                println!("[Entidad {}] agrego oro {}", id, agrego_oro);
            } else if id == 8 || id == 7 {
                // solo produce de a 1 recurso
                let recurso_a_producir = &recursos[thread_rng().gen_range(1,3)];
                let cant_a_producir_recurso = game.recursos[recurso_a_producir] + 1;
                game.recursos.insert(recurso_a_producir.to_string(), cant_a_producir_recurso);
                println!("[Entidad {}] produzco recurso {} en cantidad {}",id,  &recurso_a_producir, cant_a_producir_recurso);
            } else {
                // consumo recurso 
                let recurso_a_producir = &recursos[thread_rng().gen_range(1,3)];
                let cant_a_producir_recurso = game.recursos[recurso_a_producir] - 1;
                
                game.recursos.insert(recurso_a_producir.to_string(), cant_a_producir_recurso);
                println!("[Entidad {}] consumo recurso {} en cantidad {}",id,  &recurso_a_producir, cant_a_producir_recurso);
            }
        }
        round +=1;
       
        println!("{:?}", game_local.read());
    }
}
