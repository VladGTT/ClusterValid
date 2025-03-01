use std::{iter::zip, sync::Arc};

use crate::{
    calc_error::CalcError,
    sender::{Sender, Subscriber},
};
use ndarray::{ArcArray1, Array1, Array2, ArrayView1, ArrayView2, Axis};

use super::raw_data::RawDataValue;
#[derive(Debug, Clone)]
pub struct ScatValue {
    pub val: Arc<Vec<f64>>,
    pub clusters_vars: Arc<Vec<Array1<f64>>>,
    pub var: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    pub fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<usize>,
    ) -> Result<(Vec<f64>, Vec<Array1<f64>>, Vec<f64>), CalcError> {
        y.columns()
            .into_iter()
            .map(|c| self.helper(x, &c))
            .collect::<Result<Vec<(f64, Array1<f64>, f64)>, CalcError>>()
            .map(|v| {
                let (first, rest): (Vec<_>, Vec<_>) =
                    v.into_iter().map(|(x, y, z)| (x, (y, z))).unzip();
                let (second, third): (Vec<_>, Vec<_>) = rest.into_iter().unzip();
                (first, second, third)
            })
    }
    fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<usize>,
    ) -> Result<(f64, Array1<f64>, f64), CalcError> {
        let var = x.var_axis(Axis(0), 0.);
        let q = *y.iter().max().ok_or("Cant get max cluster index")? as usize + 1;
        let mut clusters_vars: Array2<f64> = Array2::zeros((q, x.ncols()));
        for i in 0..q {
            let vec = zip(x.rows(), y)
                .filter(|(_, c)| **c as usize == i)
                .map(|(v, _)| v.into_iter())
                .flatten()
                .map(|v| *v)
                .collect::<Vec<f64>>();
            let arr = Array2::from_shape_vec((vec.len() / x.ncols(), x.ncols()), vec)
                .map_err(|e| e.to_string())?;
            let var = arr.var_axis(Axis(0), 0.);
            clusters_vars.row_mut(i).assign(&var);
        }
        let var = var.pow2().sum().sqrt();
        let clusters_vars = clusters_vars.dot(&clusters_vars.t()).diag().sqrt();
        let clusters_vars_mean = clusters_vars.mean().ok_or("Cant calc mean")?;
        let val = clusters_vars_mean / var;
        Ok((val, clusters_vars, var))
    }
}
pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, ScatValue>,
}
impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, ScatValue>) -> Self {
        Self {
            index: Index,
            sender,
        }
    }
}

impl<'a> Subscriber<RawDataValue<'a>> for Node<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        let res = match data {
            Ok(rd) => self
                .index
                .compute(&rd.x, &rd.y)
                .map(|(val, clusters_vars, var)| ScatValue {
                    val: Arc::new(val),
                    clusters_vars: Arc::new(clusters_vars),
                    var: Arc::new(var),
                }),
            Err(err) => Err(err),
        };
        self.sender.send_to_subscribers(res);
    }
}
