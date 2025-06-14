use crate::calc_error::{CalcError, CombineErrors};
use crate::sender::{Sender, Subscriber};
use itertools::izip;
use ndarray::{ArcArray2, Array2, ArrayView2};
use ndarray_linalg::Determinant;
use std::sync::Arc;

use super::helpers::{
    counts::CountsValue, total_dispercion::TDValue, wgs::WGSValue,
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
        wg: &Vec<Vec<Array2<f64>>>,
        td: &ArcArray2<f64>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<Vec<f64>, CalcError> {
        izip!(wg, counts)
            .map(|(w, c)| self.helper(w, &td.view(), c))
            .collect()
    }
    fn helper(
        &self,
        wg: &Vec<Array2<f64>>,
        td: &ArrayView2<f64>,
        counts: &Vec<usize>,
    ) -> Result<f64, CalcError> {
        let vec = izip!(wg, counts)
            .map(|(wg, nk)| {
                let det_wg = (wg / *nk as f64)
                    .det()
                    .map_err(|e| CalcError::from(format!("{e:?}")))?;
                Ok(det_wg.ln() * *nk as f64)
            })
            .collect::<Result<Vec<f64>, CalcError>>()?;
        let val = vec.iter().sum();

        Ok(val)
    }
}

pub struct Node<'a> {
    index: Index,
    counts: Option<Result<CountsValue, CalcError>>,
    wg: Option<Result<WGSValue, CalcError>>,
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
impl<'a> Subscriber<WGSValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGSValue, CalcError>) {
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
