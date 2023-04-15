use tp0::coordenada_struct::Coordenada;
use tp0::funciones::*;
use tp0::pieza_struct::Pieza;

fn main() {
    let tablero: String = match leer_archivo() {
        Ok(resultado) => resultado,
        Err(_) => return,
    };

    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    match cargar_tablero(&tablero, &mut blanca, &mut negra) {
        Ok(_) => {}
        Err(_) => return,
    };

    println!("{}", jugar_partida(&blanca, &negra));
}
