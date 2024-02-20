
/*
        Un ejercicio practico de actores en Rust. Se consultaba una lista de 
        pasajes que tienen diferentes aerolíneas y se debía de devolver el 
        resultado ordenado por precio
 */
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct ConsultarPasajesEnAerolineas;

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct ConsultarPasajes {
    sender: Recipient<RespuestaPasajes>,
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct RespuestaPasajes {
    aerolinea: String,
    pasajes: Vec<(String, u32)>,
}

struct CoordinadorAerolineas {
    aerolineas: Vec<Addr<Aerolinea>>,
    remaining_aerolineas: u32,
    pasajes_por_aerolinea: HashMap<String, Vec<(String, u32)>>,
}

struct Aerolinea {
    nombre: String,
    pasajes: Vec<(String, u32)>,
}

impl Actor for CoordinadorAerolineas {
    type Context = Context<Self>;
}

impl Actor for Aerolinea {
    type Context = Context<Self>;
}

impl Handler<ConsultarPasajesEnAerolineas> for CoordinadorAerolineas {
    type Result = Result<(), ()>;
    fn handle(
        &mut self,
        _msg: ConsultarPasajesEnAerolineas,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        for aerolinea in &self.aerolineas {
            let _ = aerolinea.do_send(ConsultarPasajes {
                sender: _ctx.address().recipient(),
            });
        }
        Ok(())
    }
}

impl Handler<ConsultarPasajes> for Aerolinea {
    type Result = Result<(), ()>;
    fn handle(&mut self, msg: ConsultarPasajes, _ctx: &mut Context<Self>) -> Self::Result {
        let mut pasajes_ordenados = self.pasajes.clone();
        pasajes_ordenados.sort_by(|a, b| b.1.cmp(&a.1));
        let _ = msg.sender.do_send(RespuestaPasajes {
            aerolinea: self.nombre.clone(),
            pasajes: pasajes_ordenados,
        });
        Ok(())
    }
}

impl Handler<RespuestaPasajes> for CoordinadorAerolineas {
    type Result = Result<(), ()>;
    fn handle(&mut self, msg: RespuestaPasajes, _: &mut Context<Self>) -> Self::Result {
        self.remaining_aerolineas -= 1;
        self.pasajes_por_aerolinea.insert(msg.aerolinea, msg.pasajes);
        if self.remaining_aerolineas == 0 {
            for (aerolinea, pasajes) in &self.pasajes_por_aerolinea {
                println!("Aerolinea {:?} posee los siguientes pasajes {:?}", aerolinea, pasajes);
            }
        }
        Ok(())
    }
}

fn main() {
    let system = System::new();
    system.block_on(async {
        let aerolinea1 =
            Aerolinea {
                nombre: "ARG".to_string(),
                pasajes: vec![
                    ("rio_iguazu".to_string(), 500),
                    ("bs_aires".to_string(), 200),
                    ("salta".to_string(), 1000),
                ],
            }
            .start();
        let aerolinea2 =
            Aerolinea {
                nombre: "AMERICAN".to_string(),
                pasajes: vec![
                    ("rio_iguazu".to_string(), 700),
                    ("bs_aires".to_string(), 1000),
                    ("usa".to_string(), 5000),
                ],
            }
            .start();

        let coordinator = CoordinadorAerolineas {
            aerolineas: vec![aerolinea1, aerolinea2],
            remaining_aerolineas: 2,
            pasajes_por_aerolinea: HashMap::new(),
        }
        .start();

        coordinator.do_send(ConsultarPasajesEnAerolineas {});
    });
    system.run().unwrap();
}
