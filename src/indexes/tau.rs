use super::helpers::pairs_and_distances::PairsAndDistancesValue;
use super::helpers::s_plus_and_minus::SPlusAndMinusValue;
use crate::calc_error::{CalcError, CombineErrors};
use crate::sender::{Sender, Subscriber};
use core::f64;
use ndarray::Array1;
use std::iter::zip;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TauIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    pub fn compute(
        &self,
        pairs_in_the_same_cluster: &Vec<Array1<i8>>,
        s_plus: &Vec<isize>,
        s_minus: &Vec<isize>,
        ties: &Vec<isize>,
    ) -> Result<Vec<f64>, CalcError> {
        zip(zip(pairs_in_the_same_cluster, ties), zip(s_plus, s_minus))
            .into_iter()
            .map(|((p, t), (sp, sm))| self.helper(p, *sp, *sm, *t))
            .collect()
    }
    fn helper(
        &self,
        pairs_in_the_same_cluster: &Array1<i8>,
        s_plus: isize,
        s_minus: isize,
        ties: isize,
    ) -> Result<f64, CalcError> {
        let nt = pairs_in_the_same_cluster.len() as f64;
        let temp = nt * (nt - 1.) / 2.;

        let nw = pairs_in_the_same_cluster
            .iter()
            .filter(|i| **i == 1)
            .count() as f64;
        let nb = nt - nw;
        let value = (s_plus - s_minus) as f64 / (temp * nw * nb).sqrt();
        Ok(value)
    }
}

pub struct Node<'a> {
    index: Index,
    s_plus_and_minus: Option<Result<SPlusAndMinusValue, CalcError>>,
    pairs_and_distances: Option<Result<PairsAndDistancesValue, CalcError>>,
    sender: Sender<'a, TauIndexValue>,
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
                    .compute(&pd.pairs, &spm.s_plus, &spm.s_minus, &spm.ties)
                    .map(|val| TauIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.s_plus_and_minus = None;
            self.pairs_and_distances = None;
        }
    }
    pub fn new(sender: Sender<'a, TauIndexValue>) -> Self {
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
