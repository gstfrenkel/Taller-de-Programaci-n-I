#[cfg(test)]
mod integration_tests {
    use std::fs;
    use tp0::constantes::*;
    use tp0::coordenada_struct::Coordenada;
    use tp0::funciones::*;
    use tp0::pieza_struct::Pieza;

    #[test]
    fn cargar_tablero_vacio() {
        let contenido = match fs::read_to_string("tests/ejemplo1.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_err());
    }

    #[test]
    fn cargar_tablero_valido() {
        let contenido = match fs::read_to_string("tests/ejemplo2.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_ok());
        assert_eq!(negra.posicion.x, 6);
        assert_eq!(negra.posicion.y, 2);
        assert_eq!(blanca.posicion.x, 3);
        assert_eq!(blanca.posicion.y, 4);
    }

    #[test]
    fn cargar_tablero_corto() {
        let contenido = match fs::read_to_string("tests/ejemplo3.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_err());
    }

    #[test]
    fn cargar_tablero_largo() {
        let contenido = match fs::read_to_string("tests/ejemplo4.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_err());
    }

    #[test]
    fn cargar_tablero_mal_copiado() {
        let contenido = match fs::read_to_string("tests/ejemplo5.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_err());
    }

    #[test]
    fn cargar_tablero_con_pocas_piezas() {
        let contenido = match fs::read_to_string("tests/ejemplo6.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_err());
        assert_eq!(blanca.posicion.x, 3);
        assert_eq!(blanca.posicion.y, 4);
        assert_eq!(negra.posicion.x, 0);
        assert_eq!(negra.posicion.y, 0);
    }

    #[test]
    fn cargar_tablero_con_muchas_piezas() {
        let contenido = match fs::read_to_string("tests/ejemplo7.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_err());
    }

    #[test]
    fn captura_blanca() {
        let contenido = match fs::read_to_string("tests/ejemplo8.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_ok());
        assert_eq!(jugar_partida(&blanca, &negra), GANA_BLANCA);
    }

    #[test]
    fn captura_negra() {
        let contenido = match fs::read_to_string("tests/ejemplo9.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_ok());
        assert_eq!(jugar_partida(&blanca, &negra), GANA_NEGRA);
    }

    #[test]
    fn hay_empate() {
        let contenido = match fs::read_to_string("tests/ejemplo10.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_ok());
        assert_eq!(jugar_partida(&blanca, &negra), EMPATE);
    }

    #[test]
    fn hay_empate_peon() {
        let contenido = match fs::read_to_string("tests/ejemplo11.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_ok());
        assert_eq!(jugar_partida(&blanca, &negra), EMPATE);
    }

    #[test]
    fn no_se_capturan_peon() {
        let contenido = match fs::read_to_string("tests/ejemplo12.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_ok());
        assert_eq!(jugar_partida(&blanca, &negra), NADIE_GANA);
    }

    #[test]
    fn no_se_capturan() {
        let contenido = match fs::read_to_string("tests/ejemplo13.txt") {
            Ok(tablero) => tablero,
            Err(_) => {
                assert_eq!(true, false);
                return;
            }
        };

        let mut blanca = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };
        let mut negra = Pieza {
            pieza: ' ',
            posicion: Coordenada { x: 0, y: 0 },
        };

        let cargado = cargar_tablero(&contenido, &mut blanca, &mut negra);

        assert!(cargado.is_ok());
        assert_eq!(jugar_partida(&blanca, &negra), NADIE_GANA);
    }
}
