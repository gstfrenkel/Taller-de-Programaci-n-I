use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::{InvalidData, InvalidInput};

use crate::constantes::*;
use crate::pieza_struct::Pieza;

/// Determina si dos casilleros contiguos del tablero son iguales.
///
/// # Parámetros
///
/// * `casillero` - Un caracter representando un casillero específico del tablero.
/// * `ult_casillero` - Un caracter representando un casillero específico del tablero, anterior a casillero.
///
/// # Retorna
///
/// En caso de que no sean iguales, se retorna un `Result` con un valor booleano indicando que el casillero no se repite.
/// En caso de que sean iguales, se retorna un `Err` con con un mensaje de error indicando que el formato del tablero es inválido.
///
fn se_repite_casillero(casillero: char, ult_casillero: char) -> Result<bool, Error> {
    if casillero == ult_casillero {
        return Err(Error::new(InvalidInput, "Formato de tablero inválido."));
    }
    Ok(false)
}

/// Determina si en un casillero se encuentra una pieza de ajedrez jugable.
///
/// # Parámetros
///
/// * `casillero` - Un caracter representando un casillero específico del tablero.
///
/// # Retorna
///
/// Devuelve un booleano indicando si el casillero representa una de seis piezas jugables.
///
fn es_pieza(casillero: char) -> bool {
    matches!(casillero, REY | DAMA | ALFIL | CABALLO | TORRE | PEON)
}

/// Determina si un casillero es válido en función del caracter que lo representa.
///
/// # Parámetros
///
/// * `casillero` - Un caracter representando un casillero específico del tablero.
///
/// # Retorna
///
/// Devuelve un booleano indicando si el casillero representa una de seis piezas jugables, o alguno de los caracteres que componen el tablero.
///
fn es_caracter_valido(casillero: char) -> bool {
    casillero == ESPACIO
        || casillero == VACIO
        || casillero == SALTO
        || es_pieza(casillero.to_ascii_uppercase())
}

/// Determina si un `casillero` es una pieza blanca, validando si se trata de una de las seis piezas jugables, y si se encuentra representada en minúscula.
///
/// # Parámetros
///
/// * `casillero` - Un caracter representando un casillero específico del tablero.
///
/// # Retorna
///
/// Devuelve un booleano indicando si el casillero representa una de seis piezas jugables y de color blanco.
///
fn es_pieza_blanca(casillero: char) -> bool {
    es_pieza(casillero.to_ascii_uppercase()) && casillero.is_ascii_lowercase()
}

/// Determina si un `casillero` es una pieza negra, validando si se trata de una de las seis piezas jugables, y si se encuentra representada en mayúscula.
///
/// # Parámetros
///
/// * `casillero` - Un caracter representando un casillero específico del tablero.
///
/// # Retorna
///
/// Devuelve un booleano indicando si el casillero representa una de seis piezas jugables y de color negro.
///
fn es_pieza_negra(casillero: char) -> bool {
    es_pieza(casillero.to_ascii_uppercase()) && casillero.is_ascii_uppercase()
}

/// Verifica que, dado un nuevo `casillero` el cual puede representar una pieza jugable o no, no se supere el límite de una pieza de ajedrez jugable por
/// cada equipo.
///
/// La función recibe un `casillero`, representado por un caracter, y dos variables mutables que sirven como contador para la cantidad de piezas negras
/// y blancas que ya se han encontrado con anterioridad en el tablero. Si el casillero está ocupado por una pieza blanca y ya se había encontrado una
/// pieza blanca con anterioridad, se imprimirá por pantalla un mensaje de error y se retornará `false`. Caso contrario, el contador se incrementará en
/// uno y se retornará `true`. La misma lógica aplica en caso de que se encuentre una pieza negra. Por otro lado, si en el casillero no se encontrase
/// ninguna pieza jugable, los contadores no variarán y se retornará true.
///
/// # Parámetros
///
/// * `casillero`: Un caracter representando un casillero del tablero.
/// * `p_blancas`: Un contador mutable para mantener registro del número de piezas negras encontradas hasta el momento.
/// * `p_negras`: Un contador mutable para mantener registro del número de piezas blancas encontradas hasta el momento.
///
/// # Retorna
///
/// Retorna un `Result` con un booleano indicando que el nuevo casillero no provoca un exceso en la cantidad de piezas permitidas por equipo, o un `Err` con un mensaje
/// de error indicando que se la ha superado.
///
fn es_cantidad_piezas_correcta(
    casillero: char,
    p_blancas: &mut u8,
    p_negras: &mut u8,
) -> Result<bool, Error> {
    if es_pieza_blanca(casillero) {
        if *p_blancas > 0 {
            return Err(Error::new(
                InvalidInput,
                "Se ha encontrado más de una pieza blanca en el tablero.",
            ));
        }

        *p_blancas += 1;
    } else if es_pieza_negra(casillero) {
        if *p_negras > 0 {
            return Err(Error::new(
                InvalidInput,
                "Se ha encontrado más de una pieza negra en el tablero.",
            ));
        }

        *p_negras += 1;
    }
    Ok(true)
}

/// Verifica si un `casillero` del tablero es válido, en función del tipo de caracter que lo representa, del tipo de caracter que representa al
/// casillero anterior y de la cantidad de piezas jugables blancas y negras encontradas con anterioridad.
///
/// # Parámetros
///
/// * `casillero`: Un caracter representando un casillero dentro del tablero.
/// * `ult_casillero`: Un caracter representando el casillero anterior al actual, dentro del tablero.
/// * `p_blancas`: Un contador mutable para mantener registro del número de piezas negras encontradas hasta el momento.
/// * `p_negras`: Un contador mutable para mantener registro del número de piezas blancas encontradas hasta el momento.
///
/// # Retorna
///
/// Retorna un `Result` con un booleano indicando que se cumplen las condiciones de tipo de caracter del `casillero`, o un `Err` con su respectivo mensaje de error.
///
fn es_casillero_valido(
    casillero: char,
    ult_casillero: char,
    p_blancas: &mut u8,
    p_negras: &mut u8,
) -> Result<bool, Error> {
    if !es_caracter_valido(casillero) {
        return Err(Error::new(
            InvalidData,
            "Se ha encontrado una pieza desconocida en el tablero.",
        ));
    }

    let se_repite_casillero: bool = match se_repite_casillero(casillero, ult_casillero) {
        Ok(se_repite) => se_repite,
        Err(error) => return Err(error),
    };

    let cantidad_piezas_correcta: bool =
        match es_cantidad_piezas_correcta(casillero, p_blancas, p_negras) {
            Ok(cantidad_correcta) => cantidad_correcta,
            Err(error) => return Err(error),
        };

    Ok(!se_repite_casillero && cantidad_piezas_correcta)
}

/// Determina si el tablero respeta la dimensión de 8 columnas establecida por el juego.
///
/// # Parámetros
///
/// * `casillero`: Un caracter representando un casillero dentro del tablero.
/// * `j`: Un entero que representa la columna en la que se encuentra el casillero.
///
/// # Retorna
///
/// Retorna un `Result` con un booleano indicando que se cumplen las especificaciones de dimensión del tablero, al validar que el salto de línea se realiza al
/// recorrer 8 casilleros. Caso contrario, se retorna un `Err` con un mensaje de error indicando que no se cumple con las dimensiones.
///
fn es_dimension_correcta(casillero: char, j: u8) -> Result<bool, Error> {
    if casillero == SALTO && j != DIMENSION_AJEDREZ {
        return Err(Error::new(
            InvalidData,
            "La dimensión del tablero no coincide con uno de 8x8.",
        ));
    }
    Ok(true)
}

/// Actualiza los campos de la `pieza` de ajedrez con el caracter del `casillero` y con la ubicación en dos dimensiones, determinada por (`j`, `i`).
///
/// # Parámetros
///
/// * `pieza`: Referencia mutable a un struct de 'Pieza', la cual representa una pieza de ajedrez con su tipo y su ubicación en el tablero.
/// * `casillero`: Un caracter representando una pieza jugable dentro del tablero.
/// * `i`: Un entero que representa la fila (coordenada y) en la que se encuentra el casillero.
/// * `j`: Un entero que representa la columna (coordenada x) en la que se encuentra el casillero.
///
fn actualizar_pieza(pieza: &mut Pieza, casillero: char, i: u8, j: u8) {
    pieza.pieza = casillero;
    pieza.posicion.x = j;
    pieza.posicion.y = i;
}

/// Lee un caracter de un tablero de ajedrez y actualiza las piezas `blancas` o `negras` y la posición actual en la que se está leyendo.
///
/// # Parámetros
///
/// * `casillero`: Un caracter que representa un casillero del tablero de ajedrez.
/// * `blanca`: Una referencia mutable a una pieza blanca.
/// * `negra`: Una referencia mutable a una pieza negra.
/// * `i`: Un entero que representa la fila (coordenada y) en la que se encuentra el casillero.
/// * `j`: Un entero que representa la columna (coordenada x) en la que se encuentra el casillero.
///
fn leer_casillero(casillero: char, blanca: &mut Pieza, negra: &mut Pieza, i: &mut u8, j: &mut u8) {
    if casillero == SALTO {
        *i += 1;
        *j = 0;
    } else if casillero != ESPACIO {
        if es_pieza(casillero.to_ascii_uppercase()) {
            if es_pieza_blanca(casillero) {
                actualizar_pieza(blanca, casillero, *i, *j);
            } else if es_pieza_negra(casillero) {
                actualizar_pieza(negra, casillero, *i, *j);
            }
        }
        *j += 1;
    }
}

/// Recorre el tablero de ajedrez en formato de string, buscando la ubicación de la pieza `blanca` y la pieza `negra`, y validando la dimensión
/// del tablero, su formato, y la cantidad de piezas por equipo.
///
/// # Parámetros
///
/// * `contenido`: Un string que representa la secuencia de caracteres que compone al tablero de ajedrez.
/// * `blanca`: Una referencia mutable a una pieza blanca.
/// * `negra`: Una referencia mutable a una pieza negra.
///
/// # Retorna
///
/// Retorna un `Result` indicando si se pudo recorrer todo el tablero de forma exitosa, sin encontrar errores. Si se encuentra un
/// error, se retorna un `Err` con un mensaje detallando la causa del error.
///
pub fn cargar_tablero(contenido: &str, blanca: &mut Pieza, negra: &mut Pieza) -> Result<(), Error> {
    let (mut piezas_blancas, mut piezas_negras): (u8, u8) = (0, 0);
    let (mut i, mut j): (u8, u8) = (0, 0);
    let mut ult_casillero: char = ESPACIO;

    for casillero in contenido.chars() {
        match es_casillero_valido(
            casillero,
            ult_casillero,
            &mut piezas_blancas,
            &mut piezas_negras,
        ) {
            Ok(_) => {}
            Err(error) => return Err(error),
        };

        match es_dimension_correcta(casillero, j) {
            Ok(_) => {}
            Err(error) => return Err(error),
        };

        leer_casillero(casillero, blanca, negra, &mut i, &mut j);
        ult_casillero = casillero;
    }

    if piezas_blancas != PIEZAS_POR_COLOR || piezas_negras != PIEZAS_POR_COLOR {
        return Err(Error::new(
            InvalidData,
            "No se han encontrado las suficientes piezas por cada equipo como para jugar.",
        ));
    }

    Ok(())
}

/// Determina si la pieza `jugador` puede vencer a la pieza `rival` en un solo movimiento, considerando las reglas tradicionales del ajedrez.
///
/// Para determinar esto, se tiene en cuenta el tipo de pieza del `jugador` y la ubicación de cada una de las piezas. Adicionalmente, se tiene
/// en cuenta la orientación en caso de que la pieza del jugador sea un `PEON`, ya que estos solo pueden comer hacia adelante.
///
/// # Parámetros
///
/// * `jugador`: Una referencia a una pieza la cual representa al jugador que debe realizar el movimiento.
/// * `rival`: Una referencia a una pieza la cual representa al jugador contrario.
///
/// # Retorna
///
/// Retorna `true` en caso de que el jugador pueda vencer al rival en un solo movimiento. Caso contrario, devolverá `false`.
///
fn puede_ganar(jugador: &Pieza, rival: &Pieza) -> bool {
    let dif_x: i8 = (jugador.posicion.x as i8 - rival.posicion.x as i8).abs();
    let dif_y: i8 = (jugador.posicion.y as i8 - rival.posicion.y as i8).abs();
    let pieza: char = jugador.pieza.to_ascii_uppercase();

    pieza == REY && (dif_x <= 1 && dif_y <= 1)
        || (pieza == DAMA && (dif_x == 0 || dif_y == 0 || dif_x == dif_y))
        || (pieza == ALFIL && dif_x == dif_y)
        || (pieza == CABALLO && ((dif_x == 2 && dif_y == 1) || (dif_x == 1 && dif_y == 2)))
        || (pieza == TORRE && (dif_x == 0 || dif_y == 0))
        || (jugador.pieza == PEON
            && (dif_x == 1 && dif_y == 1 && jugador.posicion.y < rival.posicion.y))
        || (jugador.pieza == PEON.to_ascii_lowercase()
            && (dif_x == 1 && dif_y == 1 && jugador.posicion.y > rival.posicion.y))
}

/// Determina y muestra por pantalla la posibilidad de que dos piezas de ajedrez puedan comerse mutuamente en un solo movimiento.
///
/// Para determinar esto, se tiene en cuenta el tipo de pieza de cada color y la ubicación de cada una. Adicionalmente, se tiene en cuenta
/// la orientación en caso de que la pieza del jugador sea un `PEON`, ya que estos solo pueden comer hacia adelante.
///
/// # Parámetros
///
/// * `blanca`: Una referencia a una pieza la cual representa la pieza blanca del tablero.
/// * `negra`: Una referencia a una pieza la cual representa la pieza negra del tablero.
///
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

/// Lee archivo de la ruta especificada como parámetro del programa, y lo devuelve en forma de string.
///
/// # Parámetros
///
/// Esta función no recibe parámetros.
///
/// # Retorno
///
/// Esta función devuelve un `Result` conteniendo un `String` en caso de éxito o un `Err` en caso de error.
///
pub fn leer_archivo() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Err(Error::new(InvalidInput, "Se requiere una ruta de acceso al archivo .txt con las posiciones de las piezas en el tablero."));
    }

    fs::read_to_string(&args[1])
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::coordenada_struct::Coordenada;

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

        match es_cantidad_piezas_correcta(casillero, &mut blanca, &mut negra) {
            Ok(_) => assert!(true),
            Err(_) => assert!(
                false,
                "La cantidad de piezas debería ser considerada como correcta"
            ),
        };
    }

    #[test]
    fn piezas_correctas2() {
        let casillero: char = 'p';
        let mut blanca: u8 = 0;
        let mut negra: u8 = 1;

        match es_cantidad_piezas_correcta(casillero, &mut blanca, &mut negra) {
            Ok(_) => assert!(true),
            Err(_) => assert!(
                false,
                "La cantidad de piezas debería ser considerada como correcta"
            ),
        };
    }

    #[test]
    fn piezas_de_mas() {
        let casillero: char = 'P';
        let mut blanca: u8 = 1;
        let mut negra: u8 = 1;

        match es_cantidad_piezas_correcta(casillero, &mut blanca, &mut negra) {
            Ok(_) => assert!(
                false,
                "La cantidad de piezas no debería ser considerada como correcta."
            ),
            Err(error) => assert_eq!(
                error.to_string(),
                "Se ha encontrado más de una pieza negra en el tablero."
            ),
        };
    }

    #[test]
    fn casillero_desconocido() {
        let casillero: char = 'x';
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 0;
        let mut negra: u8 = 0;

        match es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un casillero válido."),
            Err(error) => assert_eq!(
                error.to_string(),
                "Se ha encontrado una pieza desconocida en el tablero."
            ),
        };
    }

    #[test]
    fn casillero_valido() {
        let casillero: char = PEON.to_ascii_lowercase();
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 0;
        let mut negra: u8 = 0;

        match es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "El casillero debería ser considerado como válido"),
        };
    }

    #[test]
    fn casillero_repetido() {
        let casillero: char = VACIO;
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 0;
        let mut negra: u8 = 0;

        match es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un casillero válido."),
            Err(error) => assert_eq!(error.to_string(), "Formato de tablero inválido."),
        };
    }

    #[test]
    fn pieza_blanca_repetida() {
        let casillero: char = PEON.to_ascii_lowercase();
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 1;
        let mut negra: u8 = 0;

        match es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un casillero válido."),
            Err(error) => assert_eq!(
                error.to_string(),
                "Se ha encontrado más de una pieza blanca en el tablero."
            ),
        };
    }

    #[test]
    fn pieza_negra_repetida() {
        let casillero: char = PEON;
        let ult_casillero: char = VACIO;
        let mut blanca: u8 = 0;
        let mut negra: u8 = 1;

        match es_casillero_valido(casillero, ult_casillero, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un casillero válido."),
            Err(error) => assert_eq!(
                error.to_string(),
                "Se ha encontrado más de una pieza negra en el tablero."
            ),
        };
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

        match cargar_tablero(contenido, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un tablero válido."),
            Err(error) => assert_eq!(
                error.to_string(),
                "No se han encontrado las suficientes piezas por cada equipo como para jugar."
            ),
        };
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

        match cargar_tablero(contenido, &mut blanca, &mut negra) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "El tablero debería ser considerado como válido."),
        };

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

        match cargar_tablero(contenido, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un tablero válido."),
            Err(error) => assert_eq!(
                error.to_string(),
                "La dimensión del tablero no coincide con uno de 8x8."
            ),
        };
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

        match cargar_tablero(contenido, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un tablero válido."),
            Err(error) => assert_eq!(
                error.to_string(),
                "La dimensión del tablero no coincide con uno de 8x8."
            ),
        };
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

        match cargar_tablero(contenido, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un tablero válido."),
            Err(error) => assert_eq!(error.to_string(), "Formato de tablero inválido."),
        };
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

        match cargar_tablero(contenido, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un tablero válido."),
            Err(error) => assert_eq!(
                error.to_string(),
                "No se han encontrado las suficientes piezas por cada equipo como para jugar."
            ),
        };
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

        match cargar_tablero(contenido, &mut blanca, &mut negra) {
            Ok(_) => assert!(false, "No debería ser considerado un tablero válido."),
            Err(error) => assert_eq!(
                error.to_string(),
                "Se ha encontrado más de una pieza blanca en el tablero."
            ),
        };
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
    }
}
