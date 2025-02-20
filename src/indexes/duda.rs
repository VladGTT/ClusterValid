use super::helpers::wgs::WGSValue;
use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::{Array2, Axis};
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct DudaIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    pub fn compute(&self, wgs: &Vec<Vec<Array2<f64>>>) -> Result<Vec<f64>, CalcError> {}
    fn helper(
        &self,
        level: usize,
        index_m: usize,
        index_k: usize,
        index_l: usize,
        wgs: &Vec<Vec<Array2<f64>>>,
    ) -> Result<f64, CalcError> {
        Ok(
            (wgs[level][index_k].diag().sum() + wgs[level][index_l].diag().sum())
                / wgs[level + 1][index_m].diag().sum(),
        )
    }
}
pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, DudaIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, DudaIndexValue>) -> Self {
        Self {
            index: Index,
            sender,
        }
    }
}

impl<'a> Subscriber<WGSValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGSValue, CalcError>) {
        let res = match data {
            Ok(wg) => self
                .index
                .compute(&wg.val)
                .map(|val| DudaIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err),
        };
        self.sender.send_to_subscribers(res);
    }
}
