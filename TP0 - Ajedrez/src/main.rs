use std::env;
use std::fs;

use tp0::coordenada_struct::Coordenada;
use tp0::pieza_struct::Pieza;
use tp0::constantes::*;
use tp0::funciones::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    if args.len() == 1 {
        println!("{}Se requiere una ruta de acceso al archivo .txt con las posiciones de las piezas en el tablero.", ERROR_MSJ);
        return;
    }

    let contenido = fs::read_to_string(&args[1]).expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));

    if cargar_tablero(&contenido, &mut blanca, &mut negra) == ERROR {
        return;
    }

    println!("{}", jugar_partida(&blanca, &negra));
}
