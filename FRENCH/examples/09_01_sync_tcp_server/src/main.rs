use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Ecoute les connexions TCP entrantes sur localhost, port 7878.
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Bloque pour toujours, gérant chaque requête qui arrive
    // sur cette adresse IP.
    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();

        gestion_connexion(flux);
    }
}

fn gestion_connexion(mut flux: TcpStream) {
    // Lit les 1024 premiers octets de données présents dans le flux
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // Répond avec l'accueil ou une erreur 404,
    // en fonction des données présentes dans la requête
    let (ligne_statut, nom_fichier) = if tampon.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contenu = fs::read_to_string(nom_fichier).unwrap();

    // Ecrit la réponse dans le flux, et purge le flux pour s'assurer
    // que la réponse est bien renvoyée au client
    let reponse = format!("{}{}", ligne_statut, contenu);
    flux.write(reponse.as_bytes()).unwrap();
    flux.flush().unwrap();
}
