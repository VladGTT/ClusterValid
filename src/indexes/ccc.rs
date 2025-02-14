use std::sync::Arc;

use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::{s, Array2, ArrayView1, ArrayView2};
use ndarray_linalg::{Eig, Inverse, Scalar};

use super::helpers::raw_data::RawDataValue;

#[derive(Clone, Debug)]
pub struct CCCIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    pub fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<usize>,
    ) -> Result<Vec<f64>, CalcError> {
        y.columns()
            .into_iter()
            .map(|c| self.helper(x, &c))
            .collect()
    }
    pub fn helper(&self, x: &ArrayView2<f64>, y: &ArrayView1<usize>) -> Result<f64, CalcError> {
        let n = x.nrows();
        let p = x.ncols();
        let q = *y.iter().max().ok_or("Cant find max")? as usize + 1;
        let xtx = x.t().dot(x);
        let m = &xtx / (x.len() as f64 - 1.);
        let (eigvals_m, _) = m.eig().map_err(|v| v.to_string())?;
        let s = eigvals_m.map(|v| v.re()).sqrt();

        let p_star = s
            .iter()
            .enumerate()
            .filter(|(_, v)| **v as usize >= 1 && (**v as usize) < q)
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(v, _)| v)
            .ok_or("Cant find p star")?;
        let v_star = s.slice(s![0..p_star]).product();
        let c = (v_star / q as f64).powf(1. / p_star as f64);
        let u = s / c;

        let temp = {
            let a = (0..p_star).map(|i| 1. / (n as f64 + u[i])).sum::<f64>();
            let b = (p_star + 1..p)
                .map(|i| u[i].powi(2) / (n as f64 + u[i]))
                .sum::<f64>();
            let c = (0..p).map(|i| u[i].powi(2)).sum::<f64>();
            (a + b) / c
        };
        let er_squared = 1. - temp * ((n - q).pow(2) / n) as f64 * (1 + 4 / n) as f64;
        let mut z: Array2<f64> = Array2::zeros((n, q));
        for (i, c) in y.iter().enumerate() {
            *z.get_mut((i, *c as usize)).ok_or("Cant get value")? = 1.;
        }

        let r_squared = {
            let ztz = z.t().dot(&z);
            let ztz_inv = Inverse::inv(&ztz).map_err(|e| e.to_string())?;
            let x_ = ztz_inv.dot(&z.t()).dot(x);

            1. - (&xtx - x_.t().dot(&z.t()).dot(&z).dot(&x_)).diag().sum() / xtx.diag().sum()
        };
        let ccc_p1 = ((1. - er_squared) / (1. - r_squared)).ln();
        let ccc_p2 = ((n * p_star) as f64 / 2.).sqrt() / (0.001 + er_squared).powf(1.2);
        let value = ccc_p1 - ccc_p2;

        Ok(value)
    }
}

pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, CCCIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, CCCIndexValue>) -> Self {
        Self {
            index: Index,
            sender,
        }
    }
}

impl<'a> Subscriber<RawDataValue<'a>> for Node<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        let res = match data.as_ref() {
            Ok(rd) => self
                .index
                .compute(&rd.x, &rd.y)
                .map(|val| CCCIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
