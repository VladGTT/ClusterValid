use std::iter::zip;

use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use itertools::Itertools;
use ndarray::{Array1, ArrayView1};
use std::sync::Arc;

use super::helpers::pairs_and_distances::PairsAndDistancesValue;

#[derive(Clone, Debug)]
pub struct CIndexValue {
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
        let nw = pairs_in_the_same_cluster
            .iter()
            .filter(|i| **i == 1)
            .count();
        let sorted_distances = distances
            .iter()
            .sorted_unstable_by(|a, b| a.total_cmp(b))
            .map(|e| *e)
            .collect::<Vec<f64>>();

        //calculating sum of Nw minimum and maximum distances
        let mut sum_of_minimum_distances = 0.0;
        let mut sum_of_maximum_distances = 0.0;
        for i in 0..nw {
            sum_of_minimum_distances += sorted_distances[i];
            sum_of_maximum_distances += sorted_distances[(sorted_distances.len() - 1) - i];
        }

        let sw = zip(pairs_in_the_same_cluster, distances)
            .filter(|(p, _)| **p == 1)
            .map(|(_, d)| *d)
            .sum::<f64>();

        Ok((sw - sum_of_minimum_distances) / (sum_of_maximum_distances - sum_of_minimum_distances))
    }
}

pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, CIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, CIndexValue>) -> Self {
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
                .map(|val| CIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
