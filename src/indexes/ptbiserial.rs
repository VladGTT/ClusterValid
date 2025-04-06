use std::iter::zip;
use super::helpers::pairs_and_distances::PairsAndDistancesValue;
use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::Array1;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PtbiserialIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    fn compute(
        &self,
        pairs_in_the_same_cluster: &Vec<Array1<i8>>,
        distances: &Vec<Array1<f64>>,
    ) -> Result<Vec<f64>, CalcError> {
        zip(pairs_in_the_same_cluster, distances)
            .map(|(p, d)| self.helper(p, d))
            .collect()
    }
    fn helper(
        &self,
        pairs_in_the_same_cluster: &Array1<i8>,
        distances: &Array1<f64>,
    ) -> Result<f64, CalcError> {
        let nt = pairs_in_the_same_cluster.len() as f64;
        let nw = pairs_in_the_same_cluster
            .iter()
            .filter(|i| **i == 1)
            .count() as f64;
        let sw = zip(pairs_in_the_same_cluster, distances)
            .filter(|(p, _)| **p == 1)
            .map(|(_, d)| *d)
            .sum::<f64>();
        let nb = pairs_in_the_same_cluster
            .iter()
            .filter(|i| **i == 0)
            .count() as f64;
        let sb = zip(pairs_in_the_same_cluster, distances)
            .filter(|(p, _)| **p == 0)
            .map(|(_, d)| *d)
            .sum::<f64>();

        let std_d = distances.std(0.);
        // let val = ((sb / nb - sw / nw) * (nw * nb / (nt * nt)).sqrt()) / std_d;
        let val = ((sw / nw - sb / nb) * (nw * nb).sqrt()) / nt / std_d;
        Ok(val)
    }
}
pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, PtbiserialIndexValue>,
}
impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, PtbiserialIndexValue>) -> Self {
        Self {
            index: Index,
            sender,
        }
    }
}
impl<'a> Subscriber<PairsAndDistancesValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<PairsAndDistancesValue, CalcError>) {
        let res = match data.as_ref() {
            Ok(pd) => self
                .index
                .compute(&pd.pairs, &pd.distances)
                .map(|val| PtbiserialIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
