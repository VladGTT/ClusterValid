use crate::calc_error::{CalcError, CombineErrors};
use ndarray::Array1;
use std::{iter::zip, sync::Arc};

use crate::sender::{Sender, Subscriber};

use super::helpers::{
    pairs_and_distances::PairsAndDistancesValue, s_plus_and_minus::SPlusAndMinusValue,
};

#[derive(Clone, Debug)]
pub struct GplusIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    fn compute(
        &self,
        pairs_in_the_same_cluster: &Vec<Array1<i8>>,
        s_minus: &Vec<isize>,
    ) -> Result<Vec<f64>, CalcError> {
        zip(pairs_in_the_same_cluster, s_minus)
            .map(|(pd, sm)| self.helper(pd, *sm))
            .collect()
    }
    fn helper(
        &self,
        pairs_in_the_same_cluster: &Array1<i8>,
        s_minus: isize,
    ) -> Result<f64, CalcError> {
        let nt = pairs_in_the_same_cluster.len() as f64;
        let value = 2. * s_minus as f64 / (nt * (nt - 1.0));
        Ok(value)
    }
}

pub struct Node<'a> {
    index: Index,
    s_plus_and_minus: Option<Result<SPlusAndMinusValue, CalcError>>,
    pairs_and_distances: Option<Result<PairsAndDistancesValue, CalcError>>,
    sender: Sender<'a, GplusIndexValue>,
}

impl<'a> Node<'a> {
    fn process_when_ready(&mut self) {
        if let (Some(s_plus_and_minus), Some(pairs_and_distances)) = (
            self.s_plus_and_minus.as_ref(),
            self.pairs_and_distances.as_ref(),
        ) {
            let res = match s_plus_and_minus.combine(pairs_and_distances) {
                Ok((spm, pd)) => self
                    .index
                    .compute(&pd.pairs, &spm.s_minus)
                    .map(|val| GplusIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.s_plus_and_minus = None;
            self.pairs_and_distances = None;
        }
    }
    pub fn new(sender: Sender<'a, GplusIndexValue>) -> Self {
        Self {
            index: Index,
            s_plus_and_minus: None,
            pairs_and_distances: None,
            sender,
        }
    }
}

impl<'a> Subscriber<SPlusAndMinusValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<SPlusAndMinusValue, CalcError>) {
        self.s_plus_and_minus = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<PairsAndDistancesValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<PairsAndDistancesValue, CalcError>) {
        self.pairs_and_distances = Some(data);
        self.process_when_ready();
    }
}
