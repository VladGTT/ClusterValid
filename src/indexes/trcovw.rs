use super::helpers::within_group_dispercion::WGDValue;
use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::{Array2, Axis};
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct TrcovwIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    fn compute(&self, wg: &Vec<Array2<f64>>) -> Result<Vec<f64>, CalcError> {
        wg.into_iter().map(|w| self.helper(w)).collect()
    }
    fn helper(&self, wg: &Array2<f64>) -> Result<f64, CalcError> {
        let var = wg.var_axis(Axis(0), 0.);
        let val = var.sum();
        Ok(val * 2.)
    }
}
pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, TrcovwIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, TrcovwIndexValue>) -> Self {
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
                .map(|val| TrcovwIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err),
        };
        self.sender.send_to_subscribers(res);
    }
}
