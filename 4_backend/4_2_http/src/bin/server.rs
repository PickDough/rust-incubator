use actix::prelude::*;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use client::Commands;
use step_4_2::repo::UserRolesRepository;
use tokio::runtime::{Builder, Handle, Runtime};
mod client;

async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    dotenv::dotenv().ok();

    ws::start(
        WebSocket::new(
            UserRolesRepository::new(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
        ),
        &req,
        stream,
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(|| {
        App::new()
            // WebSocket UI HTML file
            // websocket route
            .service(web::resource("/ws").route(web::get().to(ws)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

struct WebSocket {
    db: UserRolesRepository,
}
impl WebSocket {
    fn new(db: UserRolesRepository) -> Self {
        Self { db }
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Line {
    line: String,
}
impl Handler<Line> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Line, ctx: &mut Self::Context) {
        ctx.text(msg.line);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let commands = serde_json::from_str::<Commands>(&text);

                if let Ok(commands) = commands {
                    match commands {
                        Commands::User { command } => {
                            let recipient = ctx.address().recipient();
                            let db = self.db.clone();
                            let future = async move {
                                let res = command.apply(&db).await;
                                let res = match res {
                                    Ok(res) => res,
                                    Err(err) => format!("Error: {err}"),
                                };
                                recipient.do_send(Line { line: res });
                            };

                            future.into_actor(self).spawn(ctx);
                        }
                        Commands::Role { command } => {
                            let recipient = ctx.address().recipient();
                            let db = self.db.clone();
                            let future = async move {
                                let res = command.apply(&db).await;
                                let res = match res {
                                    Ok(res) => res,
                                    Err(err) => format!("Error: {err}"),
                                };
                                recipient.do_send(Line { line: res });
                            };

                            future.into_actor(self).spawn(ctx);
                        }
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
