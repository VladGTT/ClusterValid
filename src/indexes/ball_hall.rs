use std::iter::zip;
use std::sync::Arc;

use crate::calc_error::{CalcError, CombineErrors};
use crate::sender::{Sender, Subscriber};
use ndarray::Array2;

use super::helpers::counts::CountsValue;
use super::helpers::within_group_dispercion::WGDValue;
#[derive(Clone, Debug)]
pub struct BallHallIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    fn compute(
        &self,
        wg: &Vec<Array2<f64>>,
        cnts: &Vec<Vec<usize>>,
    ) -> Result<Vec<f64>, CalcError> {
        zip(wg, cnts)
            .into_iter()
            .map(|(wg, cnts)| self.helper(wg, cnts))
            .collect()
    }
    fn helper(&self, wg: &Array2<f64>, cnts: &Vec<usize>) -> Result<f64, CalcError> {
        let trace_wg = wg.diag().sum();
        let q = cnts.len();
        Ok(trace_wg / q as f64)
    }
}

pub struct Node<'a> {
    index: Index,
    wg: Option<Result<WGDValue, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
    sender: Sender<'a, BallHallIndexValue>,
}

impl<'a> Node<'a> {
    fn process_when_ready(&mut self) {
        if let (Some(wg), Some(counts)) = (self.wg.as_ref(), self.counts.as_ref()) {
            let res = match wg.combine(counts) {
                Ok((wg, cnts)) => self
                    .index
                    .compute(&wg.val, &cnts.val)
                    .map(|val| BallHallIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
            self.counts = None;
        }
    }
    pub fn new(sender: Sender<'a, BallHallIndexValue>) -> Self {
        Self {
            index: Index,
            wg: None,
            counts: None,
            sender,
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
