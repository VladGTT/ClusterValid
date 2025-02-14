use crate::calc_error::CalcError;
use ndarray::{Array1, ArrayView1};
use std::iter::zip;

use crate::sender::{Sender, Subscriber};
use std::sync::Arc;

use super::helpers::pairs_and_distances::PairsAndDistancesValue;

#[derive(Clone, Debug)]
pub struct McclainIndexValue {
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
        let nb = nt - nw;
        let sw = zip(pairs_in_the_same_cluster, distances)
            .filter(|(p, _)| **p == 1)
            .map(|(_, d)| *d)
            .sum::<f64>();
        let sb = zip(pairs_in_the_same_cluster, distances)
            .filter(|(p, _)| **p == 0)
            .map(|(_, d)| *d)
            .sum::<f64>();
        Ok((sw / nw) / (sb / nb))

        // let (mut num_pairs_the_same_clust, mut num_pairs_dif_clust): (f64, f64) = (0., 0.);
        // let (mut sum_dist_same_clust, mut sum_dist_dif_clust): (f64, f64) = (0.0, 0.0);
        // for (i, (row1, clust1)) in zip(x.rows(), y).enumerate() {
        //     for (j, (row2, clust2)) in zip(x.rows(), y).enumerate() {
        //         if i < j {
        //             let dist = (&row2 - &row1).pow2().sum().sqrt();
        //             if clust1 == clust2 {
        //                 sum_dist_same_clust += dist;
        //                 num_pairs_the_same_clust += 1.;
        //             } else {
        //                 sum_dist_dif_clust += dist;
        //                 num_pairs_dif_clust += 1.;
        //             }
        //         }
        //     }
        // }
        // let value = (sum_dist_same_clust / num_pairs_the_same_clust)
        //     / (sum_dist_dif_clust / num_pairs_dif_clust);
        // Ok(value)
    }
}
pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, McclainIndexValue>,
}
impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, McclainIndexValue>) -> Self {
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
                .map(|val| McclainIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
