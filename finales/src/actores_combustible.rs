/*
    Armar con actores un sistema de consulta de 
    combustible a dar al motor del auto según posición 
    del acelerador, pendiente del camino y presión del aceite. 
    El combustible tiene que ser un actor.
 */
extern crate actix;
use actix::{Actor, Context, Handler, System, Message};

#[derive(Message)]
#[rtype(result = "Result<f32,  &'static str>")]
struct GetCombustible{
    acelerador: f32, 
    pendiente : f32, 
    aceite : f32
} 

 struct Combustible {
    combustible: f32
 }

impl Actor for Combustible {
    type Context = Context<Self>;
}

impl Handler<GetCombustible> for Combustible{
    type Result = Result<f32,  &'static str>;

    fn handle(&mut self, _msg: GetCombustible, _ctx: &mut Context<Self>) -> Self::Result {
        let acelerador = _msg.acelerador;
        let pendiente = _msg.pendiente;
        let aceite = _msg.aceite;
        let amount = self.combustible - acelerador * (1.0 - pendiente)* (1.0/aceite);
        
        if amount > 0.0 {
            Ok(amount)
        } else {
            Err("Error: el auto no podria estar andando ya que no hay combustible")
        }

    }
}

fn main(){
    let system = System::new();
    system.block_on(async { 
        let combustible = Combustible{combustible: 200.0}.start();
        let result = combustible.send(GetCombustible{acelerador: 40.0, pendiente: 0.3, aceite: 1.0}).await;
        match result {
            Ok(cantidad) => println!("Cantidad de combustible: {:?} litros", cantidad),
            Err(error) => eprintln!("Error: {:?}", error) 
        }
        let result2 = combustible.send(GetCombustible{acelerador: 50.0, pendiente: 0.3, aceite: 2.0}).await;
        match result2 {
            Ok(cantidad) => println!("Cantidad de combustible: {:?} litros", cantidad),
            Err(error) => eprintln!("Error: {:?}", error) 
        }
    });
    system.run().unwrap();
}