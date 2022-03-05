use crate::matrix_element::El;
use crate::tim::ScopedTimer;
use crossbeam::channel::{unbounded, Receiver, Sender};
use nalgebra::Matrix2;
use rayon::prelude::*;
use std::thread;

mod matrix_element;
mod tim;

pub type Int = i32;
pub type Fl = f32;
pub type Tup = ((El, El, El, El), Matrix2<Fl>);

pub const MIN_TOP: Int = -15;
pub const MAX_TOP: Int = -MIN_TOP;
pub const MIN_BTM: Int = MIN_TOP;
pub const MAX_BTM: Int = MAX_TOP;

fn main() {
    let (tx, rx): (Sender<Tup>, Receiver<Tup>) = unbounded();
    let (finished_tx, finished_rx) = unbounded();

    thread::spawn(move || {
        let st = ScopedTimer::new("ALL OF THE THINGS".into());

        let mut added = vec![];
        let mut dupes: u128 = 0;
        loop {
            if let Ok((item, matrix)) = rx.recv() {
                let normalised = matrix.normalize();
                if !added.contains(&normalised) {
                    added.push(normalised);
                    println!(
                        "{:?}\t\t == {:?}",
                        item,
                        (item.0.res, item.1.res, item.2.res, item.3.res)
                    );
                } else {
                    dupes += 1;
                }
            }

            if !finished_rx.is_empty() {
                let sf = st.so_far();
                let len = added.len() as f64;

                println!("\n\n");
                println!("Removed duplicates: {}", dupes);
                println!("Overall time: {:?}", sf);
                println!("Average ops/ms: {}", (
                    (MIN_TOP..MAX_TOP).into_iter().len().pow(8) as f64 /
                        sf.as_millis() as f64
                    ));
                println!("Percentage of useful matricies: {}%", (
                    len /
                    (len + dupes as f64)
                    ) * 100.0);
                return;
            }
        }
    });

    let identity = Matrix2::identity();
    (MIN_TOP..=MAX_TOP)
        .into_par_iter()
        .for_each_with(tx, |tx, tl_top| {
            let _st = ScopedTimer::new(format!("tl_top at {}", tl_top));
            (MIN_BTM..=MAX_BTM)
                .into_par_iter()
                .for_each_with(tx.clone(), |tx, tl_btm| {
                    if tl_btm != 0 {
                        let tl = El::new(tl_top, tl_btm);

                        if tl.res.is_normal() {
                            for tr_top in MIN_TOP..=MAX_TOP {
                                for tr_btm in MIN_BTM..=MAX_BTM {
                                    if tr_btm == 0 {
                                        continue;
                                    }

                                    let tr = El::new(tr_top, tr_btm);
                                    if tr.res == 0.0 || tr.res.is_normal() {
                                        for bl_top in MIN_TOP..=MAX_TOP {
                                            for bl_btm in MIN_BTM..=MAX_BTM {
                                                if bl_btm == 0 {
                                                    continue;
                                                }

                                                let bl = El::new(bl_top, bl_btm);

                                                if bl.res == 0.0 || tl.res.is_normal() {
                                                    for br_top in MIN_TOP..=MAX_TOP {
                                                        for br_btm in MIN_BTM..=MAX_TOP {
                                                            if br_btm == 0 {
                                                                continue;
                                                            }

                                                            let br = El::new(br_top, br_btm);

                                                            if br.res == 0.0 || br.res.is_normal() {
                                                                let matrix = Matrix2::new(
                                                                    tl.res, tr.res, bl.res, br.res,
                                                                );
                                                                if matrix.pow(2) == identity
                                                                    || matrix.pow(4) == identity
                                                                    || matrix.pow(8) == identity
                                                                {
                                                                    tx.send((
                                                                        (tl, tr, bl, br),
                                                                        matrix,
                                                                    ))
                                                                    .unwrap_or_else(|err| {
                                                                        eprintln!("Error sending to tx: {}", err);
                                                                        println!(
                                                                            "{:?}\t\t == {:?}",
                                                                            (tl, tr, bl, br),
                                                                            (tl.res, tr.res, bl.res, br.res)
                                                                        );
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
        });

    finished_tx.send(()).unwrap();
}
