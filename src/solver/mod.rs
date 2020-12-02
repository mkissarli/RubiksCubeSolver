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
//! Module that deals with the solving of the rubiks cube. This is done in two
//! parts, each focusing on a different mathematical group to solve. Both phases
//! use the same implamentation of IDA*, with different depths and goals.

use prunning;
use std::cmp;
use crate::cubes::coord_cube::{ CoordCube, Moves, MOVE_LIST, PHASE_TWO_MOVE_LIST };
use crate::cubes::face_cube::FaceCube;
use crate::cubes::cubie_cube::CubieCube;

use std::thread;
pub fn solve(cc: CubieCube, max_length: usize){
    let mut children = vec![];
    
    //solve_thread(CoordCube::from_cubie_cube(cc), max_length.clone(), 0);
    
    //for rot in 0..3{
       // let mut cc_rot = cc.clone();
        
        
    for invert in 0..2 {
        let mut cc_alt = cc.clone();
        if invert == 1 {
            cc_alt = cc_alt.inverse_cubiecube();
        }
        let ml = max_length.clone();
        children.push(
            thread::spawn( move || {
                solve_thread(CoordCube::from_cubie_cube(cc_alt),  max_length.clone(), invert.clone()); 
            })
        );
    }
}

pub fn solve_thread(cc: CoordCube, max_length: usize, invert: usize){
    for depth in 0..max_length {
        println!("Depth is: {} for invert:{}", depth, invert);
        let (cc_1, dd, t) = phase_one_search(cc.clone(), depth, max_length);
        if t {
            println!("We succeeded! Moves are:");
            break;
        }
    }
}

pub fn phase_one_search(cc: CoordCube, depth: usize, max_length: usize) -> (CoordCube, usize, bool){
    if depth == 0 {
        if phase_one_subgoal(cc.clone()){
            //println!("Phase Two Achieved");
            if cc.last_move.len() == 0 {
                return phase_two_init(cc.clone(), depth, max_length);
            }
            else if [Moves::R1 as usize,
                     Moves::F1 as usize,
                     Moves::L1 as usize,
                     Moves::B1 as usize,
                     Moves::R2 as usize,
                     Moves::F2 as usize,
                     Moves::L2 as usize,
                    Moves::B2 as usize].contains(&cc.last_move[cc.last_move.len() - 1]){
                
                return phase_two_init(cc.clone(), depth, max_length);
            }
        }
    }
    else if depth > 0 {
        //println!("Trying different Moves?");
        if phase_one_cost(cc.clone()) <= depth {
            for i in 0..18 {
                let mut cc_1 = cc.clone();
                cc_1.movement(i);
                //if i == Moves::R3 as usize && depth == 0{
                //    println!("#### Phase One Coordinates:\nFlip: {}, \nTwist: {},\nUDSlice: {}\n", cc_1.flip, cc_1.twist, cc_1.udslice);
                //    println!("#### Phase Two Coordinates: \nEdge4: {},\nEdge8: {},\nCorner: {}", cc_1.edge4, cc_1.edge8, cc_1.corner);
                //}
                let (cc_1, dd, b) = phase_one_search(cc_1, depth - 1, max_length);
                if b {
                    return (cc_1, dd, b);
                }
            }
        }
    }

    (cc, depth, false)
}

pub fn phase_two_init(cc: CoordCube, p1_depth: usize, max_length: usize) -> (CoordCube, usize, bool){
    for depth in 0..(max_length - p1_depth){
        let (cc_1, dd, t) = phase_two_search(cc.clone(), p1_depth, max_length);
        if t {
            return (cc_1, dd, t)
        }
    }

    (cc.clone(), p1_depth, false)
}

pub fn phase_two_search(cc: CoordCube, depth: usize, max_length: usize) -> (CoordCube, usize, bool){
    if depth == 0 {
        if phase_two_subgoal(cc.clone()){
            // Success!!
            return (cc.clone(), depth, true);
        }
    }
    else if depth > 0 {
        if phase_two_cost(cc.clone()) <= depth {
            for i in PHASE_TWO_MOVE_LIST.iter() {
                let mut cc_1 = cc.clone();
                cc_1.movement(*i as usize);
                let (cc_1, dd, b) =  phase_two_search(cc_1, depth - 1, max_length);
                if b {
                    return (cc_1, dd, b);
                }
            }
        }
    }

    (cc, depth, false)
}

pub fn phase_one_subgoal(cc: CoordCube) -> bool {
    cc.udslice == 0 && cc.twist == 0 && cc.flip == 0
}

pub fn phase_two_subgoal(cc: CoordCube) -> bool {
    cc.edge4 == 0 && cc.edge8 == 0 && cc.corner == 0
}

pub fn phase_one_cost(cc: CoordCube) -> usize{
    std::cmp::max(
        cc.tables.udslice_twist_prune.get(
            cc.udslice,
            cc.twist),
        cc.tables.udslice_flip_prune.get(
            cc.udslice,
            cc.flip)
    ) as usize
}

pub fn phase_two_cost(cc: CoordCube) -> usize{
    std::cmp::max(
        cc.tables.edge4_corner_prune.get(
            cc.edge4,
            cc.corner),
        cc.tables.edge4_edge8_prune.get(
            cc.edge4,
            cc.edge8)
    ) as usize
}








































#[derive(Clone)]
pub struct Solver1 {
    original_cc: CoordCube,
    cc: CoordCube,
    max_depth: isize,
    tables: prunning::Tables,

    axis: Vec<isize>,
    power: Vec<isize>,
    twist: Vec<isize>,
    flip: Vec<isize>,
    udslice: Vec<isize>,
    corner: Vec<isize>,
    edge4: Vec<isize>,
    edge8: Vec<isize>,

    min_dist_1: Vec<isize>,
    min_dist_2: Vec<isize>
}

impl Solver1{
    pub fn new(cc: CoordCube, max_depth: usize) -> Solver1{
        let mut sol = Solver1{
            original_cc: cc.clone(),
            cc: cc.clone(),
            max_depth: max_depth as isize,
            tables: cc.tables.clone(),

            axis: vec![0; max_depth],
            power: vec![0; max_depth],
            twist: vec![0; max_depth],
            flip: vec![0; max_depth],
            udslice: vec![0; max_depth],
            corner: vec![0; max_depth],
            edge4: vec![0; max_depth],
            edge8: vec![0; max_depth],

            min_dist_1: vec![0; max_depth],
            min_dist_2: vec![0; max_depth],
        };

        sol.twist[0] = cc.twist as isize;
        sol.flip[0] = cc.flip as isize;
        sol.udslice[0] = cc.udslice as isize;
        sol.corner[0] = cc.corner as isize;
        sol.edge4[0] = cc.edge4 as isize;
        sol.edge8[0] = cc.edge8 as isize;
        sol.min_dist_1[0] = sol.clone().phase_one_cost(0);

        sol
    }
    
    pub fn solve(&mut self) -> String{
        let mut s: String = "No Solutions found in depth range.".to_string();
        println!("START");
        println!("#### Phase One Coordinates:\nFlip: {}, \nTwist: {},\nUDSlice: {}\n", self.flip[0], self.twist[0], self.udslice[0]);
        println!("#### Phase Two Coordinates: \nEdge4: {},\nEdge8: {},\nCorner: {}", self.edge4[0], self.edge8[0], self.corner[0]);
        
        for depth in 0..self.max_depth {
            println!("Currently at depth: {}\n", depth);
            let n = self.phase_one_search(0, depth);
            if n >= 0 {
                println!("Solved: {}", n);
                s = self.clone().solution_to_string(n as usize);
                println!("Move set: {}", s);
                return s;
            }
        }

        s
    }

    pub fn phase_one_search(&mut self, n: isize, depth: isize) -> isize{
        //println!("Phase 1 search starting\n");
        if n == 0 && depth == 0 {
            println!("n = 0 and depth = 0 \n");
        }
        if self.min_dist_1[(n as usize)] == 0
            || (self.twist[n as usize] == 0
                && self.flip[n as usize] ==0
                && self.udslice[n as usize] == 0){
            println!("Phase two started.");
            return self.phase_two_init(n);
        }
        else if self.min_dist_1[(n as usize)] <= depth{
            //println!("Trying moves.");
            for i in 0..6{
                // Don't do consecutive or opposite moves.
                if n > 0 && [i, i + 3].contains(&self.axis[(n as usize) - 1]){
                    continue;
                }
                
                for j in 1..4{
                    if i == 1 && j == 3 {
                        println!("We do a R' at {}, {}", n, depth);
                        println!("#### Phase One Coordinates:\nFlip: {}, \nTwist: {},\nUDSlice: {}\n", self.flip[n as usize], self.twist[n as usize], self.udslice[n as usize]);
                        println!("#### Phase Two Coordinates: \nEdge4: {},\nEdge8: {},\nCorner: {}", self.edge4[n as usize], self.edge8[n as usize], self.corner[n as usize]);
                    }
                    self.axis[(n as usize)] = i;
                    self.power[(n as usize)] = j;
                    let mv = 3 * i + j - 1;
                    self.twist[(n as usize) + 1] =
                        self.tables.twist_move[self.twist[(n as usize)] as usize][mv as usize];
                    self.flip[(n as usize) + 1] =
                        self.tables.flip_move[self.flip[(n as usize)] as usize][mv as usize];
                    self.udslice[(n as usize) + 1] =
                        self.tables.udslice_move[self.udslice[(n as usize)] as usize][mv as usize];
                    self.min_dist_1[(n as usize) + 1] =
                        self.clone().phase_one_cost(n + 1);
                    let m = self.phase_one_search(n + 1, depth - 1);
                    if m >= 0{
                        return m;
                    }
                }
            }
        }
       -1
    }

    pub fn phase_two_init(&mut self, p: isize) -> isize{
        if(p < 0){
            panic!("p is negative: {}", p);
        }
        
        let n = p as usize;

        println!("#### Phase One Coordinates:\nFlip: {}, \nTwist: {},\nUDSlice: {}\n", self.flip[n], self.twist[n], self.udslice[n]);
        
        let mut cc = self.original_cc.clone();
        for i in 0..n {
            for _j in 0..self.power[i]{
                if self.axis[i] < 0 {
                    panic!("self.axis is lower than 0 in init: {}", self.axis[i]);
                }
                cc.movement(self.axis[i] as usize)
            }
        }
        self.edge4[n] = cc.edge4;
        self.edge8[n] = cc.edge8;
        self.corner[n] = cc.corner;
        let val = self.clone().phase_two_cost(p);
        std::mem::replace(&mut self.min_dist_2[n], val);

        println!("#### Phase Two Coordinates: \nEdge4: {},\nEdge8: {},\nCorner: {}", self.edge4[n], self.edge8[n], self.corner[n]);
        
        for depth in 0..(self.max_depth - p){
            //println!("Phase two p: {}", p);
            let m = self.phase_two_search(p, depth);
            if m >= 0{
                return m;
            }
        }
        // fail
        return -1;
    }

    pub fn phase_two_search(&mut self, p: isize, depth: isize) -> isize{
        if p < 0 {
            panic!("What is p less than 0 in phase two search?? {}", p);
        }
        let n: usize = p as usize;

        if self.min_dist_2[n] == 0
            || self.phase_two_subgoal(n){
                return p;
        }
        else if self.min_dist_2[n] <= depth{
            for i in 0..6{
                if n > 0
                    && self.axis[n - 1] != i
                    && self.axis[n - 1] != i + 3{
                        continue;
                    }
                for j in 1..4 {
                    if (i == 1 || i == 2 || i == 4 || i == 5) && j != 2 {
                        continue;
                    }
                    self.axis[n] = i;
                    self.power[n] = j;
                    let mut mv = (3 * i + j - 1) as usize;
                    self.edge4[n + 1] = self.tables.edge4_move[self.edge4[n] as usize][mv];
                    self.edge8[n + 1] = self.tables.edge8_move[self.edge8[n] as usize][mv];
                    self.corner[n + 1] = self.tables.corner_move[self.corner[n] as usize][mv];
                    self.min_dist_2[n + 1] = self.clone().phase_two_cost(p + 1);
                    let m = self.phase_two_search(p  + 1, depth - 1);
                    if m >= 0 {
                        return m;
                    }
                }
            }
        }
        -1
    }

    pub fn phase_one_subgoal(&self, n: usize) -> bool {
        self.udslice[n] == 0 && self.twist[n] == 0 && self.flip[n] == 0
    }

    pub fn phase_two_subgoal(&self, n: usize) -> bool {
        self.edge4[n] == 0 && self.edge8[n] == 0 && self.corner[n] == 0
    }
    
    pub fn phase_one_cost(self, n: isize) -> isize{
        std::cmp::max(
            self.cc.tables.udslice_twist_prune.get(
                self.udslice[(n as usize)],
                self.twist[(n as usize)]),
            self.cc.tables.udslice_flip_prune.get(
                self.udslice[(n as usize)],
                self.flip[(n as usize)])
        )
    }

    pub fn phase_two_cost(self, n: isize) -> isize{
        std::cmp::max(
            self.cc.tables.edge4_corner_prune.get(
                self.edge4[(n as usize)],
                self.corner[(n as usize)]),
            self.cc.tables.edge4_edge8_prune.get(
                self.edge4[(n as usize)],
                self.edge8[(n as usize)])
        )
    }

    pub fn solution_to_string(self, length: usize) -> String{
        let mut moves: Vec<String> = Vec::new();
        for i in 0..length {
            let s1 = match self.axis[i] {
                0 => 'U',
                1 => 'R',
                2 => 'F',
                3 => 'D',
                4 => 'L',
                5 => 'B',
                _ => panic!("There shouldn't be a number higher than 5 in axis?: {}", self.axis[i])
            };

            let s2 = match self.power[i] {
                //0 => "", // <- Not sure why we  have zeros but its probs an error
                1 => "",
                2 => "2",
                3 => "'",
                _ => panic!("Unknown value in power?: {}", self.power[i])
            };

            moves.push(format!("{}{}", s1, s2));
        }

        let s = moves.join(" ");
        s
    }
}
