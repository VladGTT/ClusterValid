use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::{Array1, ArrayView1, ArrayView2};
use std::collections::HashMap;
use std::iter::zip;
use std::sync::Arc;

use super::helpers::raw_data::RawDataValue;
#[derive(Clone, Debug)]
pub struct SilhouetteIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    pub fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<u32>,
    ) -> Result<Vec<f64>, CalcError> {
        y.columns()
            .into_iter()
            .map(|c| self.helper(x, &c))
            .collect()
    }
    fn helper(&self, x: &ArrayView2<f64>, y: &ArrayView1<u32>) -> Result<f64, CalcError> {
        let q = *y.iter().max().ok_or("Cant get numb of clusters")? + 1;
        let s = zip(x.rows(), y)
            .map(|(row, c)| {
                let wcvec: Vec<f64> = zip(x.rows(), y)
                    .filter(|(_, cluster)| *cluster == c)
                    .map(|(row2, _)| (&row2 - &row).pow2().sum().sqrt())
                    .collect();
                let a = wcvec.iter().sum::<f64>() / (wcvec.len() - 1) as f64;

                let mut bcvec: HashMap<usize, Vec<f64>> = HashMap::default();
                for (row2, c2) in zip(x.rows(), y) {
                    if c2 != c {
                        let dist = (&row2 - &row).pow2().sum().sqrt();
                        if bcvec.contains_key(&(*c2 as usize)) {
                            bcvec.get_mut(&(*c2 as usize)).ok_or("Cant get b(i)")?.push(dist);
                        } else {
                            bcvec.insert(*c2 as usize, vec![dist]);
                        }
                    }
                }
                let b = bcvec
                    .into_iter()
                    .map(|(_, vec)| Array1::from_vec(vec).mean())
                    .collect::<Option<Vec<f64>>>()
                    .ok_or("Cant calc means in b(i)")?
                    .into_iter()
                    .min_by(|a, b| a.total_cmp(b))
                    .ok_or("Cant find min in b(i)")?;

                let value = (b - a) / a.max(b);
                Ok(value)
            })
            .collect::<Result<Vec<f64>, CalcError>>()?;
        let val = Array1::from_vec(s)
            .mean()
            .ok_or("Cant calc res mean".into());
        val
        // let q = *y.iter().max().ok_or("Cant get numb of clusters")? + 1;
        // let mut s: Vec<Vec<f64>> = vec![Vec::default(); q];
        // for (row1, c1) in zip(x.rows(), y) {
        //     let mut d: Vec<Vec<f64>> = vec![Vec::default(); q];
        //     for (row2, c2) in zip(x.rows(), y) {
        //         let dist = (&row2 - &row1).pow2().sum().sqrt();
        //         if row1 != row2 {
        //             d.get_mut(*c2).ok_or("Cant get val")?.push(dist);
        //         }
        //     }
        //     let mut d = d
        //         .into_iter()
        //         .enumerate()
        //         .map(|(i, v)| {
        //             if i == *c1 {
        //                 Array1::from_vec(v).mean()
        //             } else {
        //                 Some(v.iter().sum::<f64>() / (v.len() - 1) as f64)
        //             }
        //         })
        //         .collect::<Option<Vec<f64>>>()
        //         .ok_or("Cant calc mean")?;
        //     let a = d.remove(*c1);
        //     let b = d
        //         .into_iter()
        //         .min_by(|a, b| a.total_cmp(b))
        //         .ok_or("Cant calc min")?;
        //     s.get_mut(*c1)
        //         .ok_or("Cant get val")?
        //         .push((b - a) / b.max(a));
        // }
        // let s = s
        //     .into_iter()
        //     .map(|v| Array1::from_vec(v).mean())
        //     .collect::<Option<Vec<f64>>>()
        //     .ok_or("Cant calc mean")?;
        // Array1::from_vec(s)
        //     .mean()
        //     .ok_or("Cant calc res mean".into())
    }
}

pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, SilhouetteIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, SilhouetteIndexValue>) -> Self {
        Self {
            index: Index::default(),
            sender,
        }
    }
}
impl<'a> Subscriber<RawDataValue<'a>> for Node<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue, CalcError>) {
        let res = match data.as_ref() {
            Ok(rd) => self
                .index
                .compute(&rd.x, &rd.y)
                .map(|val| SilhouetteIndexValue { val: Arc::new(val) }),

            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
