/*
    Implementar en un programa con pseudo-codigo rust (utilizando el modelo de 
        actores) un mecanismo para consultar en tiempo real el resultado parcial 
        de una elección donde las mesas van emitiendo sus votos propios de manera 
        concurrente.
*/

extern crate actix;
use std::collections::HashMap;
use actix::prelude::*;


struct Mesa {
    numero: u32, 
    votaciones: Vec<(String, u32)> 
}

struct CoordinadorMesas {
    mesas: Vec<Addr<Mesa>>,
    cant_mesas_escrutadas : u32,
    resultados_finales: HashMap<String, u32>
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct Votar{
    elegido: String,
    amount: u32
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct ResultadosParciales{
    sender: Recipient<RespuestaResultados>
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct RespuestaResultados{
    resultados: HashMap<String,u32>,
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct ContarVotos{}

impl Actor for Mesa{
    type Context = Context<Self>;
}

impl Actor for CoordinadorMesas{
    type Context = Context<Self>;
}

impl Handler<Votar> for Mesa{
    type Result = Result<(),()>;
    fn handle(&mut self, _msg: Votar, _ctx: &mut Context<Self>) -> Self::Result {
        println!("[MESA {:?}] recibiendo votacion", self.numero);
        self.votaciones.push((_msg.elegido,  _msg.amount));
        Ok(())
    }
}

impl Handler<ResultadosParciales> for Mesa{
    type Result = Result<(),()>;
    fn handle(&mut self, msg: ResultadosParciales, _ctx: &mut Context<Self>) -> Self::Result {
        
        let mut votaciones_parciales : HashMap<String, u32>= HashMap::new();
        for voto in &self.votaciones{
            *votaciones_parciales.entry(voto.0.clone()).or_insert(0) += voto.1;
        }
        println!("Mesa {:?} tiene estos resultados {:?}", self.numero, votaciones_parciales);
        let _ = msg.sender.do_send(RespuestaResultados{resultados: votaciones_parciales});
        Ok(())
        
    }
}
impl Handler<ContarVotos> for CoordinadorMesas {
    type Result = Result<(),()>;
    fn handle(&mut self, _msg: ContarVotos, _ctx: &mut Context<Self>)-> Self::Result {
        for mesa in &self.mesas{
            mesa.do_send(ResultadosParciales{sender: _ctx.address().recipient()}); 
        }
        Ok(())
    }
}

impl Handler<RespuestaResultados> for CoordinadorMesas {
    type Result = Result<(),()>;
    fn handle(&mut self, msg: RespuestaResultados, _ctx: &mut Context<Self>)-> Self::Result {
        self.cant_mesas_escrutadas -= 1;
        for (elegido, votos) in msg.resultados {
            *self.resultados_finales.entry(elegido).or_insert_with(|| 0) += votos;
        }
        if self.cant_mesas_escrutadas == 0 {
            for (key, value) in &self.resultados_finales{
                println!("Para el elegido [{:?}] tuvo una cantidad de votos {:?}", key, value);
            }
        }
        Ok(())
    }
}

fn main(){
    let system = System::new();
    system.block_on(async {
        let mesa1 = Mesa{numero:1, votaciones: vec![]}.start();
        let mesa2 = Mesa{numero:2, votaciones: vec![]}.start();

        mesa1.do_send(Votar{elegido: "Agus".to_string(), amount: 1});
        mesa1.do_send(Votar{elegido: "Agus".to_string(), amount: 1});
        mesa1.do_send(Votar{elegido: "Marcos".to_string(), amount: 1});

        mesa2.do_send(Votar{elegido: "Agus".to_string(), amount: 1});
        mesa2.do_send(Votar{elegido: "Agus".to_string(), amount: 1});
        mesa2.do_send(Votar{elegido: "Agus".to_string(), amount: 1});
        mesa2.do_send(Votar{elegido: "Marcos".to_string(), amount: 1});

        let coordinator = CoordinadorMesas{mesas: vec![mesa1, mesa2], cant_mesas_escrutadas: 2, resultados_finales: HashMap::new()}.start();

        coordinator.do_send(ContarVotos{})
    });
    system.run().unwrap();

}

