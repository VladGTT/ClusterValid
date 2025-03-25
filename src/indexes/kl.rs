use crate::{calc_error::{CalcError, CombineErrors}, indexes::helpers::counts};
use ndarray::Array2;

use crate::sender::{Sender, Subscriber};

use super::helpers::{counts::CountsValue, within_group_dispercion::WGDValue};
use itertools::izip;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct KLIndexValue {
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
        izip!(counts, counts[1..].iter(),counts[2..].iter(), wg, wg[1..].iter(), wg[2..].iter())
            .map(|(counts, counts_next,counts_next_next, wg, wg_next,wg_next_next)| {
                let diff = self.helper(counts_next_next, wg_next_next, counts_next, wg_next)?;
                let diff_plus_one = self.helper(counts_next, wg_next, counts, wg)?;
                Ok((diff/diff_plus_one).abs())
            })
            .collect::<Result<Vec<f64>, CalcError>>()
    }
    fn helper(
        &self,
        counts_next: &Vec<usize>,
        wg_next: &Array2<f64>,
        counts: &Vec<usize>,
        wg: &Array2<f64>,
    ) -> Result<f64, CalcError> {
        let q_next = counts_next.len() as f64;
        let q = counts.len() as f64;
        let p = wg.ncols() as f64;
        let trace_wg_next = wg_next.diag().sum();
        let trace_wg = wg.diag().sum();

        let val = q_next.powf(2. / p) * trace_wg_next - q.powf(2. / p) * trace_wg;
        Ok(val)
    }
}

pub struct Node<'a> {
    index: Index,
    counts: Option<Result<CountsValue, CalcError>>,
    wg: Option<Result<WGDValue, CalcError>>,
    sender: Sender<'a, KLIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, KLIndexValue>) -> Self {
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
                    .map(|val| KLIndexValue { val: Arc::new(val) }),
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
