use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

use crossbeam_channel::unbounded;
use rand::prelude::*;

// Write a program with the following workflow:
//
// Producer is a separate thread, which continuously generates square matrixes of random u8 elements and size 4096.
// Consumer is a separate thread, which takes a generated matrix, counts sum of all its elements and prints the sum to STDOUT.
// There are only 1 Producer and 2 Consumers.
// Counting sum of matrix elements should be parallelized.

#[derive(Clone)]
struct Matrix(Arc<[[u8; 4096]; 4096]>);

impl Matrix {
    fn new() -> Self {
        let matrix: [[u8; 4096]; 4096] =
            std::array::from_fn(|_| std::array::from_fn(|_| random::<u8>()));

        Self(Arc::new(matrix))
    }

    fn index(&self, index: ((usize, usize), (usize, usize))) -> Vec<u8> {
        let ((x1, y1), (x2, y2)) = index;

        self.0[x1..x2]
            .iter()
            .map(move |col| col[y1..y2].iter().map(|u| u.clone()))
            .flatten()
            .collect()
    }
}
fn main() {
    let (sender, reciever) = unbounded::<(Vec<u8>, Arc<AtomicU8>, Arc<AtomicU64>)>();
    const R: u8 = 2;

    let rec_closure = || {
        for (matrix, counter, sum) in reciever.clone() {
            sum.fetch_add(
                matrix.iter().fold(0u64, |acc, item| acc + (*item as u64)),
                Ordering::SeqCst,
            );
            counter.fetch_add(1, Ordering::SeqCst);

            if counter.load(Ordering::Relaxed) == R {
                println!("Sum of the matrix: {sum:?}");
            }
        }
    };

    std::thread::scope(|s| {
        s.spawn(move || {
            let matrixes = (0..3)
                .into_iter()
                .map(|_| Matrix::new())
                .collect::<Vec<_>>();
            for m in matrixes.into_iter() {
                let counter = Arc::new(AtomicU8::new(0));
                let sum = Arc::new(AtomicU64::new(0));
                let div = m.0.len() / (R as usize);
                for i in 1..((R as usize) + 1) {
                    sender
                        .send((
                            m.index((((i - 1) * div, 0), (i * div, 4096))),
                            Arc::clone(&counter),
                            Arc::clone(&sum),
                        ))
                        .unwrap();
                }
            }
        });

        s.spawn(rec_closure);
        s.spawn(rec_closure);
    });
}
