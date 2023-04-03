use std::fs;

use tp0::coordenada_struct::Coordenada;
use tp0::pieza_struct::Pieza;
use tp0::constantes::*;
use tp0::funciones::*;

#[test]
fn cargar_tablero_vacio() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo1.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));

    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    assert_eq!(cargar_tablero(&contenido, &mut blanca, &mut negra), ERROR);
}

#[test]
fn cargar_tablero_valido() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo2.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    assert_eq!(cargar_tablero(&contenido, &mut blanca, &mut negra), EXITO);
    assert_eq!(negra.posicion.x, 6);
    assert_eq!(negra.posicion.y, 2);
    assert_eq!(blanca.posicion.x, 3);
    assert_eq!(blanca.posicion.y, 4);
}

#[test]
fn cargar_tablero_corto() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo3.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    assert_eq!(cargar_tablero(&contenido, &mut blanca, &mut negra), ERROR);
}

#[test]
fn cargar_tablero_largo() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo4.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    assert_eq!(cargar_tablero(&contenido, &mut blanca, &mut negra), ERROR);
}

#[test]
fn cargar_tablero_mal_copiado() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo5.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    assert_eq!(cargar_tablero(&contenido, &mut blanca, &mut negra), ERROR);
}

#[test]
fn cargar_tablero_con_pocas_piezas() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo6.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    assert_eq!(cargar_tablero(&contenido, &mut blanca, &mut negra), ERROR);
    assert_eq!(blanca.posicion.x, 3);
    assert_eq!(blanca.posicion.y, 4);
    assert_eq!(negra.posicion.x, 0);
    assert_eq!(negra.posicion.y, 0);
}

#[test]
fn cargar_tablero_con_muchas_piezas() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo7.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    assert_eq!(cargar_tablero(&contenido, &mut blanca, &mut negra), ERROR);
}

#[test]
fn captura_blanca() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo8.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    cargar_tablero(&contenido, &mut blanca, &mut negra);
    assert_eq!(jugar_partida(&blanca, &negra), GANA_BLANCA);
}

#[test]
fn captura_negra() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo9.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    cargar_tablero(&contenido, &mut blanca, &mut negra);
    assert_eq!(jugar_partida(&blanca, &negra), GANA_NEGRA);
}

#[test]
fn hay_empate() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo10.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    cargar_tablero(&contenido, &mut blanca, &mut negra);
    assert_eq!(jugar_partida(&blanca, &negra), EMPATE);
}

#[test]
fn hay_empate_peon() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo11.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    cargar_tablero(&contenido, &mut blanca, &mut negra);
    assert_eq!(jugar_partida(&blanca, &negra), EMPATE);
}

#[test]
fn no_se_capturan_peon() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo12.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    cargar_tablero(&contenido, &mut blanca, &mut negra);
    assert_eq!(jugar_partida(&blanca, &negra), NADIE_GANA);
}

#[test]
fn no_se_capturan() {
    let contenido = fs::read_to_string(
        "/home/gst-frenkel/Desktop/Taller de Programacion 1/tp0/tests/ejemplo13.txt",
    )
    .expect(&format!(
        "{}No se pudo abrir el archivo especificado.",
        ERROR_MSJ
    ));
    let mut blanca = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };
    let mut negra = Pieza {
        pieza: ' ',
        posicion: Coordenada { x: 0, y: 0 },
    };

    cargar_tablero(&contenido, &mut blanca, &mut negra);
    assert_eq!(jugar_partida(&blanca, &negra), NADIE_GANA);
}
