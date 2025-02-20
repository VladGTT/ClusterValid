use super::helpers::counts::CountsValue;
use crate::calc_error::CombineErrors;

use super::helpers::wgs::WGSValue;
use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::Array2;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct Pseudot2IndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    pub fn compute(
        &self,
        wgs: &Vec<Vec<Array2<f64>>>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<Vec<f64>, CalcError> {
    }
    fn helper(
        &self,
        level: usize,
        index_m: usize,
        index_k: usize,
        index_l: usize,

        wgs: &Vec<Vec<Array2<f64>>>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<f64, CalcError> {
        let wgs_k = wgs[level][index_k].diag().sum();
        let wgs_l = wgs[level][index_l].diag().sum();
        let wgs_m = wgs[level + 1][index_m].diag().sum();
        let val = (wgs_m - wgs_l - wgs_k) / (wgs_k + wgs_l)
            * (counts[level][index_k] + counts[level][index_l] - 2) as f64;
        Ok(val)
    }
}
pub struct Node<'a> {
    index: Index,
    counts: Option<Result<CountsValue, CalcError>>,
    wgs: Option<Result<WGSValue, CalcError>>,
    sender: Sender<'a, Pseudot2IndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, Pseudot2IndexValue>) -> Self {
        Self {
            index: Index,
            counts: None,
            wgs: None,
            sender,
        }
    }

    fn process_when_ready(&mut self) {
        if let (Some(counts), Some(wgs)) = (self.counts.as_ref(), self.wgs.as_ref()) {
            let res = match counts.combine(wgs) {
                Ok((counts, wgs)) => self
                    .index
                    .compute(&wgs.val, &counts.val)
                    .map(|val| Pseudot2IndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wgs = None;
            self.counts = None;
        }
    }
}

impl<'a> Subscriber<WGSValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGSValue, CalcError>) {
        self.wgs = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<CountsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<CountsValue, CalcError>) {
        self.counts = Some(data);
        self.process_when_ready();
    }
}
