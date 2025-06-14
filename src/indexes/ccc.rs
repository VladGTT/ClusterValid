use std::f64;
use std::sync::Arc;

use crate::calc_error::{CalcError, CombineErrors};
use crate::sender::{Sender, Subscriber};
use itertools::izip;
use ndarray::{ ArcArray2, Array1, Array2, ArrayView1, ArrayView2};
use ndarray_linalg::{Eig, Inverse, Scalar};

use super::helpers::raw_data::RawDataValue;
use super::helpers::total_dispercion::TDValue;
use super::helpers::within_group_dispercion::WGDValue;

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
        y: &ArrayView2<u32>,
        wg: &Vec<Array2<f64>>,
        td: &ArcArray2<f64>,
    ) -> Result<Vec<f64>, CalcError> {
        izip!(y.columns(), wg)
            .map(|(c, w)| self.helper(x, &c, &w.view(), &td.view()))
            .collect()
    }
    pub fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<u32>,
        wg: &ArrayView2<f64>,
        td: &ArrayView2<f64>,
    ) -> Result<f64, CalcError> {
        let n = x.nrows();
        let p = x.ncols();
        let q = (*y.iter().max().ok_or("Cant find max")? + 1) as usize;
        let xtx = x.t().dot(x);
        let m = &xtx / (n as f64 - 1.);
        let (eigvals_m, _) = m.eig().map_err(|v| v.to_string())?;
        let s = eigvals_m.map(|v| v.re()).sqrt();
        let s_view = s.view();

        let z = {
            let mut retval: Array2<f64> = Array2::zeros((n, q));
            for (i, c) in y.iter().enumerate() {
                *retval.get_mut((i, *c as usize)).ok_or("Cant get value")? = 1.;
            }
            retval
        };

        let r_squared = {
            let ztz = z.t().dot(&z);
            let ztz_inv = Inverse::inv(&ztz).map_err(|e| e.to_string())?;
            let x_ = ztz_inv.dot(&z.t()).dot(x);
            1. - (&xtx - x_.t().dot(&z.t()).dot(&z).dot(&x_)).diag().sum() / xtx.diag().sum()
        };
        let (p_star, u) = {
            let vv = s_view.product();
            let c = (vv as f64 /q as f64).powf(1./p as f64);
            let u = &s_view / &(Array1::from_elem(s_view.len(),c)).view();
            let k1 = u.iter().filter(|i|**i>=1.).count();
            let p_star = k1.min(q-1);

            let mut v1 = 1.;
            for i in 0..p_star{
                v1=v1*s_view[i]; 
            }
            let c = (v1/q as f64).powf(1./p_star as f64);
            let u = &s_view / &(Array1::from_elem(s_view.len(),c)).view(); 
            (p_star,u)
        };
        // return Err(CalcError::from(format!("{p} {u}")));
        let temp = {
            let a = (0..p_star).map(|i| 1. / (n as f64 + u[i])).sum::<f64>();
            let b = (p_star..p) // p*+1..p
                .map(|i| u[i].powi(2) / (n as f64 + u[i]))
                .sum::<f64>();
            let c = (0..p).map(|i| u[i].powi(2)).sum::<f64>();
            (a + b) / c
        };
        let er_squared = 1. - temp * ((n - q).pow(2) as f64 / n as f64) * (1. + 4. / n as f64);
        let ccc_p1 = ((1. - er_squared) / (1. - r_squared)).ln();
        let ccc_p2 = ((n * p_star) as f64 / 2.).sqrt() / (0.001 + er_squared).powf(1.2);
        let value = ccc_p1 * ccc_p2;
        Ok(value)
    }
}

pub struct Node<'a> {
    index: Index,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    wg: Option<Result<WGDValue, CalcError>>,
    td: Option<Result<TDValue, CalcError>>,
    sender: Sender<'a, CCCIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, CCCIndexValue>) -> Self {
        Self {
            index: Index,
            raw_data: None,
            wg: None,
            td: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(raw_data), Some(wg), Some(td)) =
            (self.raw_data.as_ref(), self.wg.as_ref(), self.td.as_ref())
        {
            let res = match raw_data.combine(wg).combine(td) {
                Ok(((rd, wg), td)) => self
                    .index
                    .compute(&rd.x, &rd.y, &wg.val, &td.val)
                    .map(|val| CCCIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
            self.td = None;
        }
    }
}

impl<'a> Subscriber<WGDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGDValue, CalcError>) {
        self.wg = Some(data);
        self.process_when_ready();
    }
}

impl<'a> Subscriber<TDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<TDValue, CalcError>) {
        self.td = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<RawDataValue<'a>> for Node<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        self.raw_data = Some(data);
        self.process_when_ready();
    }
}
