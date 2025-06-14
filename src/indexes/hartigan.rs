use crate::calc_error::{CalcError, CombineErrors};
use ndarray::{ Array2};

use crate::sender::{Sender, Subscriber};

use super::helpers::{counts::CountsValue, within_group_dispercion::WGDValue};
use std::sync::Arc;
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
        wg: &[Array2<f64>],
    ) -> Result<Vec<f64>, CalcError> {
        let mut retval = vec![f64::NAN;counts.len()];
        for i in 0..counts.len()-1{
            let val = Self::helper(&counts[i], &wg[i], &wg[i+1])?;
            retval[i]=val;
        }
        Ok(retval)
    }
    fn helper(counts:&[usize], w: &Array2<f64>, w_plus_one: &Array2<f64>)->Result<f64,CalcError>{
                let tracewg_plus_one = w_plus_one.diag().sum();
                let tracewg = w.diag().sum();
                let n = counts.iter().sum::<usize>() as f64;
                let q = counts.len() as f64;
                let val = (tracewg / tracewg_plus_one - 1.) * (n - q - 1.);
                Ok(val)
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
