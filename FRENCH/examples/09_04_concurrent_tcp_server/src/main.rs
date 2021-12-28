// ANCHOR: main_func
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    ecouteur
        .incoming()
        .for_each_concurrent(/* limite */ None, |flux_tcp| async move {
            let flux_tcp = flux_tcp.unwrap();
            gestion_connexion(flux_tcp).await;
        })
        .await;
}
// ANCHOR_END: main_func

const reponse: &'static str = "salut";
// ANCHOR: handle_connection
use async_std::prelude::*;

async fn gestion_connexion(mut flux: TcpStream) {
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).await.unwrap();

    //<-- partie masquée ici -->
    flux.write(reponse.as_bytes()).await.unwrap();
    flux.flush().await.unwrap();
}
// ANCHOR_END: handle_connection
