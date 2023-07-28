#[derive(Debug, PartialEq)]
pub enum NodeTypeError {
    Setting,
    HandShake,
    HeaderDownload,
    BlockDownload,
}

#[derive(Debug, PartialEq)]
pub struct NodeError {
    tipo: NodeTypeError,
    mensaje: String,
}

impl NodeError {
    pub fn new(tipo: NodeTypeError, mensaje: String) -> NodeError {
        NodeError { tipo, mensaje }
    }
}

impl From<std::io::Error> for NodeError {
    fn from(_: std::io::Error) -> NodeError {
        NodeError::new(NodeTypeError::FileNotFound, "File not found".to_string())
    }
}

impl From<ParseIntError> for NodeError {
    fn from(_: ParseIntError) -> NodeError {
        NodeError::new(
            NodeTypeError::FieldNotFound,
            "Field not found".to_string(),
        )
    }
}

impl From<AddrParseError> for NodeError {
    fn from(_: AddrParseError) -> NodeError {
        NodeError::new(
            NodeTypeError::FieldNotFound,
            "Field not found".to_string(),
        )
    }
}

impl From<ParseBoolError> for NodeError {
    fn from(_: ParseBoolError) -> NodeError {
        NodeError::new(
            NodeTypeError::FieldNotFound,
            "Field not found".to_string(),
        )
    }
}
