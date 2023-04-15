use crate::coordenada_struct::Coordenada;

/// Pieza de ajedrez, representada por un caracter el cual indica su color y el tipo de pieza, y su posición en un tablero de dos dimensiones.
pub struct Pieza {
    pub pieza: char,
    pub posicion: Coordenada,
}
