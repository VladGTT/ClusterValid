use crate::calc_error::{CalcError, CombineErrors};
use ndarray::Array2;
use ndarray_linalg::Determinant;

use crate::sender::{Sender, Subscriber};

use super::helpers::{counts::CountsValue, within_group_dispercion::WGDValue};
use std::{iter::zip, sync::Arc};
#[derive(Clone, Debug)]
pub struct HartiganIndexValue {
    pub val: Arc<Vec<f64>>,
}

#[derive(Default)]
pub struct Index;
impl Index {
    pub fn compute(
        &self,
        counts: &Vec<Vec<usize>>,
        wg: &Vec<Array2<f64>>,
    ) -> Result<Vec<f64>, CalcError> {
        zip(zip(counts, counts.iter().next()), zip(wg, wg.iter().next()))
            .into_iter()
            .map(|((counts, counts_next), (wg, wg_next))| {
                let tracewg_next = wg_next.diag().sum();
                let tracewg = wg.diag().sum();
                let n = counts_next.iter().sum::<usize>() as f64;
                let q = counts_next.len() as f64;
                let val = (tracewg_next / tracewg - 1.) * (n - q - 1.);
                Ok(val)
            })
            .collect()
    }
}

pub struct Node<'a> {
    index: Index,
    counts: Option<Result<CountsValue, CalcError>>,
    wg: Option<Result<WGDValue, CalcError>>,
    sender: Sender<'a, HartiganIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, HartiganIndexValue>) -> Self {
        Self {
            index: Index,
            counts: None,
            wg: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(counts), Some(wg)) = (self.counts.as_ref(), self.wg.as_ref()) {
            let res = match wg.combine(counts) {
                Ok((wg, cnts)) => self
                    .index
                    .compute(&cnts.val, &wg.val)
                    .map(|val| HartiganIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
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
