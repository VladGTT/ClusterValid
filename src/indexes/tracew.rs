use std::sync::Arc;

use super::helpers::within_group_dispercion::WGDValue;
use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::Array2;

#[derive(Clone, Debug)]
pub struct TracewIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    fn compute(&self, wg: &Vec<Array2<f64>>) -> Result<Vec<f64>, CalcError> {
        wg.into_iter().map(|w| self.helper(w)).collect()
    }
    fn helper(&self, wg: &Array2<f64>) -> Result<f64, CalcError> {
        Ok(wg.diag().sum())
    }
}
pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, TracewIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, TracewIndexValue>) -> Self {
        Self {
            index: Index,
            sender,
        }
    }
}

impl<'a> Subscriber<WGDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGDValue, CalcError>) {
        let res = match data {
            Ok(wg) => self
                .index
                .compute(&wg.val)
                .map(|val| TracewIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err),
        };
        self.sender.send_to_subscribers(res);
    }
}
