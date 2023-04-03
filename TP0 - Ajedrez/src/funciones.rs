use crate::constantes::*;
use crate::pieza_struct::Pieza;

fn es_formato_valido(casillero: char, ult_casillero: char) -> bool {
    if casillero == ult_casillero {
        println!("{}Formato de tablero inválido.", ERROR_MSJ);
        return false;
    }
    return true;
}

fn es_pieza_valida(casillero: char) -> bool {
    return matches!(casillero, REY | DAMA | ALFIL | CABALLO | TORRE | PEON);
}

fn es_caracter_valido(casillero: char) -> bool {
    return casillero == ESPACIO
        || casillero == VACIO
        || casillero == SALTO
        || es_pieza_valida(casillero.to_ascii_uppercase());
}

fn es_pieza_blanca(casillero: char) -> bool {
    return es_pieza_valida(casillero.to_ascii_uppercase()) && casillero.is_ascii_lowercase();
}

fn es_pieza_negra(casillero: char) -> bool {
    return es_pieza_valida(casillero.to_ascii_uppercase()) && casillero.is_ascii_uppercase();
}

///Devuelve false e imprime un mensaje de error en caso de que el nuevo casillero sea una pieza jugable (Rey, Dama, Alfil, Torre, Peón o Caballo) y ya se había encontrado una del mismo color con anterioridad. En cualquier otro caso devuelve true.
fn es_cantidad_piezas_correcta(casillero: char, p_blancas: &mut u8, p_negras: &mut u8) -> bool {
    if es_pieza_blanca(casillero) {
        if *p_blancas > 0 {
            println!(
                "{}Se ha encontrado más de una pieza blanca en el tablero.",
                ERROR_MSJ
            );
            return false;
        }

        *p_blancas += 1;
    } else if es_pieza_negra(casillero) {
        if *p_negras > 0 {
            println!(
                "{}Se ha encontrado más de una pieza negra en el tablero.",
                ERROR_MSJ
            );
            return false;
        }

        *p_negras += 1;
    }
    return true;
}

///Devuelve false e imprime un mensaje de error en caso de que casillero no corresponda a uno de los caracteres esperados (R, D, A, C, T, O, _, \n, o ' '), en caso de que se se haya superado el límite de una pieza negra y blanca en el tablero, o en caso de que haya dos caracteres del mismo tipo de forma consecutiva. En cualquier otro caso devolverá true.
fn es_casillero_valido(
    casillero: char,
    ult_casillero: char,
    p_blancas: &mut u8,
    p_negras: &mut u8,
) -> bool {
    if !es_caracter_valido(casillero) {
        println!(
            "{}Se ha encontrado una pieza desconocida en el tablero.",
            ERROR_MSJ
        );
        return false;
    }

    return es_formato_valido(casillero, ult_casillero)
        && es_cantidad_piezas_correcta(casillero, p_blancas, p_negras);
}

///Devuelve false e imprime un mensaje de error en caso de que, al encontrarse un salto de línea como casillero actual, aún no se ha alcanzado el límite de 8 casilleros recorridos del tablero. En cualquiero otro caso devuelve true.
fn es_dimension_correcta(casillero: char, j: u8) -> bool {
    if casillero == SALTO && j != DIMENSION_AJEDREZ {
        println!(
            "{}La dimensión del tablero no coincide con uno de 8x8.",
            ERROR_MSJ
        );
        return false;
    }
    return true;
}

fn actualizar_pieza(pieza: &mut Pieza, casillero: char, i: u8, j: u8) {
    pieza.pieza = casillero;
    pieza.posicion.x = j;
    pieza.posicion.y = i;
}

///Actualiza las variables de recorrido del tablero, y guarda información del tipo de casillero y su ubicación en caso de que se trae de una pieza jugable (Rey, Dama, Alfil, Torre, Peón o Caballo).
fn leer_casillero(casillero: char, blanca: &mut Pieza, negra: &mut Pieza, i: &mut u8, j: &mut u8) {
    if casillero == SALTO {
        *i += 1;
        *j = 0;
    } else if casillero != ESPACIO {
        if es_pieza_valida(casillero.to_ascii_uppercase()) {
            if es_pieza_blanca(casillero) {
                actualizar_pieza(blanca, casillero, *i, *j);
            } else if es_pieza_negra(casillero) {
                actualizar_pieza(negra, casillero, *i, *j);
            }
        }
        *j += 1;
    }
}

///Recorre el tablero en busca de la ubicación de las piezas jugables (Rey, Dama, Alfil, Torre, Peón o Caballo). Devuelve ERROR en caso de que el tablero no se ajuste a la dimensión de 8x8, o en caso de que posea un formato inválido. En cualquier otro caso devuelve EXITO.
pub fn cargar_tablero(contenido: &str, blanca: &mut Pieza, negra: &mut Pieza) -> i8 {
    let (mut piezas_blancas, mut piezas_negras): (u8, u8) = (0, 0);
    let (mut i, mut j): (u8, u8) = (0, 0);
    let mut ult_casillero: char = ESPACIO;

    for casillero in contenido.chars() {
        if !es_casillero_valido(
            casillero,
            ult_casillero,
            &mut piezas_blancas,
            &mut piezas_negras,
        ) {
            return ERROR;
        }

        if !es_dimension_correcta(casillero, j) {
            return ERROR;
        }

        leer_casillero(casillero, blanca, negra, &mut i, &mut j);
        ult_casillero = casillero;
    }

    if piezas_blancas != PIEZAS_POR_COLOR || piezas_negras != PIEZAS_POR_COLOR {
        println!(
            "{}No se han encontrado las suficientes piezas por cada equipo como para jugar.",
            ERROR_MSJ
        );
        return ERROR;
    }

    return EXITO;
}

///De acuerdo a las reglas del ajedrez, devuelve true si la Pieza jugador puede capturar a la Pieza rival.
fn puede_ganar(jugador: &Pieza, rival: &Pieza) -> bool {
    let dif_x: i8 = (jugador.posicion.x as i8 - rival.posicion.x as i8).abs();
    let dif_y: i8 = (jugador.posicion.y as i8 - rival.posicion.y as i8).abs();
    let pieza: char = jugador.pieza.to_ascii_uppercase();

    return pieza == REY && (dif_x <= 1 && dif_y <= 1)
        || (pieza == DAMA && (dif_x == 0 || dif_y == 0 || dif_x == dif_y))
        || (pieza == ALFIL && dif_x == dif_y)
        || (pieza == CABALLO && ((dif_x == 2 && dif_y == 1) || (dif_x == 1 && dif_y == 2)))
        || (pieza == TORRE && (dif_x == 0 || dif_y == 0))
        || (jugador.pieza == PEON
            && (dif_x == 1 && dif_y == 1 && jugador.posicion.y < rival.posicion.y))
        || (jugador.pieza == PEON.to_ascii_lowercase()
            && (dif_x == 1 && dif_y == 1 && jugador.posicion.y > rival.posicion.y));
}

///Devuelve EMPATE en caso de que ambas piezas puedan capturarse, GANA_BLANCA en caso de que solo la blanca pueda capturar, GANA_NEGRA en caso de que solo la negra pueda capturar, o NADIE_GANA si ninguna de las dos piezas puede capturar a la otra, de acuerdo a las reglas del ajedrez.
pub fn jugar_partida(blanca: &Pieza, negra: &Pieza) -> char {
    let gana_blanca: bool = puede_ganar(blanca, negra);
    let gana_negra: bool = puede_ganar(negra, blanca);

    match (gana_blanca, gana_negra) {
        (true, true) => EMPATE,
        (true, false) => GANA_BLANCA,
        (false, true) => GANA_NEGRA,
        _ => NADIE_GANA,
    }
}

#[cfg(test)]
mod tests {
    use crate::coordenada_struct::Coordenada;
    use super::*;

    #[test]
    fn leer_casillero_pieza_negra() {
        let contenido = PEON;
        let mut i: u8 = 1;
        let mut j: u8 = 1;
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        leer_casillero(contenido, &mut blanca, &mut negra, &mut i, &mut j);

        assert_eq!(j, 2);
        assert_eq!(i, 1);
        assert_eq!(negra.posicion.x, 1);
        assert_eq!(negra.posicion.y, 1);
    }

    #[test]
    fn leer_casillero_pieza_blanca() {
        let contenido = PEON.to_ascii_lowercase();
        let mut i: u8 = 1;
        let mut j: u8 = 1;
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        leer_casillero(contenido, &mut blanca, &mut negra, &mut i, &mut j);

        assert_eq!(j, 2);
        assert_eq!(i, 1);
        assert_eq!(blanca.posicion.x, 1);
        assert_eq!(blanca.posicion.y, 1);
    }

    #[test]
    fn leer_casillero_salto() {
        let contenido = SALTO;
        let mut i: u8 = 1;
        let mut j: u8 = 1;
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        leer_casillero(contenido, &mut blanca, &mut negra, &mut i, &mut j);

        assert_eq!(j, 0);
        assert_eq!(i, 2);
    }

    #[test]
    fn leer_casillero_espacio() {
        let contenido = ESPACIO;
        let mut i: u8 = 1;
        let mut j: u8 = 1;
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        leer_casillero(contenido, &mut blanca, &mut negra, &mut i, &mut j);

        assert_eq!(j, 1);
        assert_eq!(i, 1);
    }

    #[test]
    fn leer_casillero_vacio() {
        let contenido = VACIO;
        let mut i: u8 = 1;
        let mut j: u8 = 1;
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        leer_casillero(contenido, &mut blanca, &mut negra, &mut i, &mut j);

        assert_eq!(j, 2);
        assert_eq!(i, 1);
    }

    #[test]
    fn piezas_correctas() {
        let casillero: char = 'p';
        let mut blanca: u8 = 0;
        let mut negra: u8 = 0;

        assert_eq!(
            es_cantidad_piezas_correcta(casillero, &mut blanca, &mut negra),
            true
        );
    }

    #[test]
    fn piezas_correctas2() {
        let casillero: char = 'p';
        let mut blanca: u8 = 0;
        let mut negra: u8 = 1;

        assert_eq!(
            es_cantidad_piezas_correcta(casillero, &mut blanca, &mut negra),
            true
        );
    }

    #[test]
    fn piezas_de_mas() {
        let casillero: char = 'P';
        let mut blanca: u8 = 1;
        let mut negra: u8 = 1;

        assert_eq!(
            es_cantidad_piezas_correcta(casillero, &mut blanca, &mut negra),
            false
        );
    }

    #[test]
    fn casillero_desconocido() {
        let casillero: char = 'x';
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 0;
        let mut negra: u8 = 0;

        assert_eq!(
            es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra),
            false
        );
    }

    #[test]
    fn casillero_valido() {
        let casillero: char = PEON.to_ascii_lowercase();
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 0;
        let mut negra: u8 = 0;

        assert_eq!(
            es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra),
            true
        );
    }

    #[test]
    fn casillero_repetido() {
        let casillero: char = VACIO;
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 0;
        let mut negra: u8 = 0;

        assert_eq!(
            es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra),
            false
        );
    }

    #[test]
    fn pieza_blanca_repetida() {
        let casillero: char = PEON.to_ascii_lowercase();
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 1;
        let mut negra: u8 = 0;

        assert_eq!(
            es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra),
            false
        );
    }

    #[test]
    fn pieza_negra_repetida() {
        let casillero: char = PEON;
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 0;
        let mut negra: u8 = 1;

        assert_eq!(
            es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra),
            false
        );
    }

    #[test]
    fn cargar_tablero_vacio() {
        let contenido = "";
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(cargar_tablero(contenido, &mut blanca, &mut negra), ERROR);
    }

    #[test]
    fn cargar_tablero_valido() {
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ p _ _ _ _\n_ _ P _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n";
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(cargar_tablero(contenido, &mut blanca, &mut negra), EXITO);
        assert_eq!(blanca.posicion.x, 3);
        assert_eq!(blanca.posicion.y, 2);
        assert_eq!(negra.posicion.x, 2);
        assert_eq!(negra.posicion.y, 3);
    }

    #[test]
    fn cargar_tablero_corto() {
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ p _ _ _ _\n_ _ P _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _\n";
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(cargar_tablero(contenido, &mut blanca, &mut negra), ERROR);
    }

    #[test]
    fn cargar_tablero_largo() {
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ p _ _ _ _\n_ _ P _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _ _\n";
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(cargar_tablero(contenido, &mut blanca, &mut negra), ERROR);
    }

    #[test]
    fn cargar_tablero_mal_copiado() {
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ p _ _ _ _\n_ _ P _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ __  _ _\n";
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(cargar_tablero(contenido, &mut blanca, &mut negra), ERROR);
    }

    #[test]
    fn cargar_tablero_con_pocas_piezas() {
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ p _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n";
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(cargar_tablero(contenido, &mut blanca, &mut negra), ERROR);
        assert_eq!(blanca.posicion.x, 3);
        assert_eq!(blanca.posicion.y, 2);
        assert_eq!(negra.posicion.x, 0);
        assert_eq!(negra.posicion.y, 0);
    }

    #[test]
    fn cargar_tablero_con_muchas_piezas() {
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ p _ _ _ _\n_ _ r _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ d _ _ _\n_ _ _ D _ _ _ _\n_ _ _ _ _ _ _ _\n";
        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(cargar_tablero(contenido, &mut blanca, &mut negra), ERROR);
    }

    #[test]
    fn peon_no_captura() {
        let blanca = Pieza {
            pieza: 'p',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let negra = Pieza {
            pieza: 'P',
            posicion: Coordenada { x: 1, y: 1 },
        };

        assert_eq!(puede_ganar(&blanca, &negra), false);
        assert_eq!(puede_ganar(&negra, &blanca), false);
        assert_eq!(jugar_partida(&blanca, &negra), NADIE_GANA);
    }

    #[test]
    fn peon_captura() {
        let blanca = Pieza {
            pieza: 'p',
            posicion: Coordenada { x: 1, y: 1 },
        };
        let negra = Pieza {
            pieza: 'P',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(puede_ganar(&blanca, &negra), true);
        assert_eq!(puede_ganar(&negra, &blanca), true);
        assert_eq!(jugar_partida(&blanca, &negra), EMPATE);
    }

    #[test]
    fn rey_captura() {
        let blanca = Pieza {
            pieza: 'r',
            posicion: Coordenada { x: 1, y: 1 },
        };
        let negra = Pieza {
            pieza: 'R',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(puede_ganar(&blanca, &negra), true);
        assert_eq!(puede_ganar(&negra, &blanca), true);
        assert_eq!(jugar_partida(&blanca, &negra), EMPATE);
    }

    #[test]
    fn rey_no_captura() {
        let blanca = Pieza {
            pieza: 'r',
            posicion: Coordenada { x: 1, y: 1 },
        };
        let negra = Pieza {
            pieza: 'R',
            posicion: Coordenada { x: 3, y: 6 },
        };

        assert_eq!(puede_ganar(&blanca, &negra), false);
        assert_eq!(puede_ganar(&negra, &blanca), false);
        assert_eq!(jugar_partida(&blanca, &negra), NADIE_GANA);
    }

    #[test]
    fn reina_captura() {
        let blanca = Pieza {
            pieza: 'd',
            posicion: Coordenada { x: 0, y: 6 },
        };
        let negra = Pieza {
            pieza: 'R',
            posicion: Coordenada { x: 0, y: 0 },
        };

        assert_eq!(puede_ganar(&blanca, &negra), true);
        assert_eq!(puede_ganar(&negra, &blanca), false);
        assert_eq!(jugar_partida(&blanca, &negra), GANA_BLANCA);
    }

    #[test]
    fn caballo_captura() {
        let blanca = Pieza {
            pieza: 'd',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let negra = Pieza {
            pieza: 'C',
            posicion: Coordenada { x: 2, y: 1 },
        };

        assert_eq!(puede_ganar(&blanca, &negra), false);
        assert_eq!(puede_ganar(&negra, &blanca), true);
        assert_eq!(jugar_partida(&blanca, &negra), GANA_NEGRA);
    }

    #[test]
    fn torre_captura() {
        let blanca = Pieza {
            pieza: 't',
            posicion: Coordenada { x: 6, y: 1 },
        };
        let negra = Pieza {
            pieza: 'C',
            posicion: Coordenada { x: 2, y: 1 },
        };

        assert_eq!(puede_ganar(&blanca, &negra), true);
        assert_eq!(puede_ganar(&negra, &blanca), false);
        assert_eq!(jugar_partida(&blanca, &negra), GANA_BLANCA);
    }

    #[test]
    fn alfil_captura() {
        let blanca = Pieza {
            pieza: 'd',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let negra = Pieza {
            pieza: 'A',
            posicion: Coordenada { x: 7, y: 7 },
        };

        assert_eq!(puede_ganar(&blanca, &negra), true);
        assert_eq!(puede_ganar(&negra, &blanca), true);
        assert_eq!(jugar_partida(&blanca, &negra), EMPATE);
    }
}
