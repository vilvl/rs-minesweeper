
use crate::common::*;

pub struct OpenCellsRq {
    crds: Vec<Coords>,
}

pub struct OpenCellsRs {
    crds: Vec<Cell>,
}
