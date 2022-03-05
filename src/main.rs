use std::fmt::format;
use crate::matrix_element::El;
use crate::tim::ScopedTimer;
use crossbeam::channel::{Sender, Receiver, unbounded};
use nalgebra::Matrix2;
use rayon::prelude::*;
use std::thread;

mod matrix_element;
mod tim;

pub type Int = i32;
pub type Fl = f32;
pub type Tup = (El, El, El, El);

pub const MIN_TOP: Int = -50;
pub const MAX_TOP: Int = -MIN_TOP;
pub const MIN_BTM: Int = MIN_TOP;
pub const MAX_BTM: Int = MAX_TOP;

fn main() {
    let (tx, rx): (Sender<Tup>, Receiver<Tup>) = unbounded();

    thread::spawn(move || {
        let mut added = vec![];
        loop {
            if let Ok(item) = rx.recv() {
                let a = (item.0.res, item.1.res, item.2.res, item.3.res);
                if !added.contains(&a) {
                    added.push(a);
                    println!("{:?}\t\t == {:?}", item, a);
                }
            }
        }
    });

    let identity = Matrix2::identity();
    (MIN_TOP..=MAX_TOP).into_par_iter().for_each_with(tx.clone(), |tx, tl_top| {
        let _st = ScopedTimer::new(format!("tl_top at {}", tl_top));

        (MIN_BTM..=MAX_BTM).into_par_iter().for_each_with(tx.clone(), |tx, tl_btm| {
            let tl = El::new(tl_top, tl_btm);

            if tl.res.is_normal() {
                (MIN_TOP..=MAX_TOP).into_par_iter().for_each_with(tx.clone(), |tx, tr_top| {
                    (MIN_BTM..=MAX_TOP).into_par_iter().for_each_with(tx.clone(), |tx, tr_btm| {
                        let tr = El::new(tr_top, tr_btm);

                        if tr.res == 0.0 || tr.res.is_normal() {
                            (MIN_TOP..=MAX_TOP).into_par_iter().for_each_with(tx.clone(), |tx, bl_top| {
                                (MIN_BTM..=MAX_TOP).into_par_iter().for_each_with(tx.clone(), |tx, bl_btm| {
                                    let bl = El::new(bl_top, bl_btm);

                                    if bl.res == 0.0 || tl.res.is_normal() {
                                        for br_top in MIN_TOP..=MAX_TOP {
                                            for br_btm in MIN_BTM..=MAX_TOP {
                                                let br = El::new(br_top, br_btm);

                                                if br.res == 0.0 || br.res.is_normal() {
                                                    let matrix = Matrix2::new(tl.res, tr.res, bl.res, br.res);
                                                    if matrix.pow(2) == identity || matrix.pow(4) == identity || matrix.pow(8) == identity {
                                                        tx.send((tl, tr, bl, br)).unwrap();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                });
                            });
                        }
                    });
                });
            }
        });
    });
}
