use crate::calc_error::CalcError;
use ndarray::{ArcArray1, Array1, ArrayView1};

use crate::sender::{Sender, Subscriber};
use std::iter::zip;
use std::sync::Arc;

use super::helpers::pairs_and_distances::PairsAndDistancesValue;
#[derive(Clone, Debug)]
pub struct DunnIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    pub fn compute(
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
        let max_intracluster = zip(pairs_in_the_same_cluster, distances)
            .filter(|(i, _)| **i == 1)
            .map(|(_, d)| *d)
            .max_by(|a, b| a.total_cmp(b))
            .ok_or("Can't find max intracluster distance")?;
        let min_intercluster = zip(pairs_in_the_same_cluster, distances)
            .filter(|(i, _)| **i == 0)
            .map(|(_, d)| *d)
            .min_by(|a, b| a.total_cmp(b))
            .ok_or("Can't find min intercluster distance")?;
        let val = min_intercluster / max_intracluster;
        Ok(val)
    }
}

pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, DunnIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, DunnIndexValue>) -> Self {
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
                .map(|val| DunnIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
