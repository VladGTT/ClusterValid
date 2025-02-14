use crate::calc_error::{CalcError, CombineErrors};
use ndarray::{ArcArray2, Array2, ArrayView2};
use ndarray_linalg::Determinant;
use std::{iter::zip, sync::Arc};

use crate::sender::{Sender, Subscriber};

use super::helpers::{
    counts::CountsValue, total_dispercion::TDValue, within_group_dispercion::WGDValue,
};

#[derive(Clone, Debug)]
pub struct ScottIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    fn compute(
        &self,
        wg: &Vec<Array2<f64>>,
        td: &ArcArray2<f64>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<Vec<f64>, CalcError> {
        zip(wg, counts)
            .map(|(w, c)| self.helper(w, &td.view(), c))
            .collect()
    }
    fn helper(
        &self,
        wg: &Array2<f64>,
        td: &ArrayView2<f64>,
        counts: &Vec<usize>,
    ) -> Result<f64, CalcError> {
        let n = counts.iter().sum::<usize>() as f64;
        let det_t = td.det().map_err(|e| CalcError::from(format!("{e:?}")))?;
        let det_wg = wg.det().map_err(|e| CalcError::from(format!("{e:?}")))?;
        let val = (det_t / det_wg).ln();
        let val = val * n;
        // let x_mean = x.mean_axis(Axis(0)).ok_or("Cant calc mean")?;
        // let mut diffs1: Array2<f64> = Array2::zeros(x.dim());
        // let mut diffs2: Array2<f64> = Array2::zeros(x.dim());
        // for (i, (x, y)) in zip(x.rows(), y).enumerate() {
        //     diffs1.row_mut(i).assign(&(&x - &clusters_centroids[y]));
        //     diffs2.row_mut(i).assign(&(&x - &x_mean));
        // }
        //
        // let w_q = diffs1.t().dot(&diffs1);
        // let t = diffs2.t().dot(&diffs2);
        // let det_t = Determinant::det(&t).map_err(|e| CalcError::from(format!("{e:?}")))?;
        // let det_w_q = Determinant::det(&w_q).map_err(|e| CalcError::from(format!("{e:?}")))?;
        //
        // let val = x.nrows() as f64 * (det_t / det_w_q).ln();
        //
        Ok(val)
    }
}

pub struct Node<'a> {
    index: Index,
    counts: Option<Result<CountsValue, CalcError>>,
    wg: Option<Result<WGDValue, CalcError>>,
    td: Option<Result<TDValue, CalcError>>,
    sender: Sender<'a, ScottIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, ScottIndexValue>) -> Self {
        Self {
            index: Index,
            wg: None,
            td: None,
            counts: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(wg), Some(td), Some(counts)) =
            (self.wg.as_ref(), self.td.as_ref(), self.counts.as_ref())
        {
            let res = match wg.combine(td).combine(counts) {
                Ok(((wg, td), cnts)) => self
                    .index
                    .compute(&wg.val, &td.val, &cnts.val)
                    .map(|val| ScottIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
            self.td = None;
            self.counts = None;
        }
    }
}
impl<'a> Subscriber<CountsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<CountsValue, CalcError>) {
        self.counts = Some(data);
        self.process_when_ready();
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
