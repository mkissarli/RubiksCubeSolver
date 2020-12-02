//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************
//!
//! Module for coord level cube.

use crate::prunning::Tables;
use super::cubie_cube::CubieCube;
use cubes::{TWIST, FLIP, UDSLICE, EDGE4, EDGE8, CORNER};

/// Coordinate Representation of a cube. Updates coordinates using a pre-computed
/// move table.
#[derive(Clone)]
pub struct CoordCube {
    // Phase One
    pub twist:   isize,
    pub flip:    isize,
    pub udslice: isize,

    // Phase Two
    pub edge4:   isize,
    pub edge8:   isize,
    pub corner:  isize,
    pub tables:  Tables,

    pub last_move: Vec<usize>,
}

impl CoordCube{

    /// Create a CoordCube from a CubieCube struct.
    pub fn from_cubie_cube(cc: CubieCube) -> CoordCube {
        CoordCube{
            twist:   cc.twist()   as isize,
            flip:    cc.flip()    as isize,
            udslice: cc.udslice() as isize,
            edge4:   cc.edge4()   as isize,
            edge8:   cc.edge8()   as isize,
            corner:  cc.corner()  as isize,
            tables:  Tables::load_tables(),
            last_move: Vec::new(),
        }
    }

    /// A Move method to update the coordinates in the cubie cube from tables.
    pub fn movement(&mut self, m_as_u: usize){
        //let m_as_u = m as usize;

        if self.twist < 0 {
            self.twist = TWIST as isize + self.twist;
        }
        if self.flip < 0 {
            self.flip = FLIP as isize + self.flip;
        }
        if self.udslice < 0 {
            self.udslice = UDSLICE as isize + self.udslice;
        }
        if self.edge4 < 0 {
            //println!("Edge4 is under 0 again: {}", self.edge4);
            self.edge4 = EDGE4 as isize + self.edge4;
        }
        if self.edge8 < 0 {
            self.edge8 = EDGE8 as isize + self.edge8;
        }
        if self.corner < 0 {
            self.corner = CORNER as isize + self.corner;
        }
        
        self.twist   = self.tables.twist_move  [self.twist   as usize][m_as_u];
        self.flip    = self.tables.flip_move   [self.flip    as usize][m_as_u];
        self.udslice = self.tables.udslice_move[self.udslice as usize][m_as_u];
        self.edge4   = self.tables.edge4_move  [self.edge4   as usize][m_as_u];
        self.edge8   = self.tables.edge8_move  [self.edge8   as usize][m_as_u];
        self.corner  = self.tables.corner_move [self.corner  as usize][m_as_u];

        self.last_move.push(m_as_u);
    }
}

#[derive(Copy, Clone)]
pub enum Moves {
    U1 = 0,
    U2, U3,
    R1, R2, R3,
    F1, F2, F3,
    D1, D2, D3,
    L1, L2, L3,
    B1, B2, B3
}

pub const MOVE_LIST: [Moves; 18] = [
    Moves::U1, Moves::U2, Moves::U3,
    Moves::R1, Moves::R2, Moves::R3,
    Moves::F1, Moves::F2, Moves::F3,
    Moves::D1, Moves::D2, Moves::D3,
    Moves::L1, Moves::L2, Moves::L3,
    Moves::B1, Moves::B2, Moves::B3
];


pub const PHASE_TWO_MOVE_LIST: [usize; 10] = [
    Moves::U1 as usize, Moves::U2 as usize, Moves::U3 as usize,
    Moves::R2 as usize,
    Moves::F2 as usize,
    Moves::D1 as usize, Moves::D2 as usize, Moves::D3 as usize,
    Moves::L2 as usize,
    Moves::B2 as usize,
];
