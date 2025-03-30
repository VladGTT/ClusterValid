use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::{Array1, ArrayView1, ArrayView2, Axis};
use std::sync::Arc;

use super::raw_data::RawDataValue;
#[derive(Debug, Clone)]
pub struct PairsAndDistancesValue {
    pub pairs: Arc<Vec<Array1<i8>>>,
    pub distances: Arc<Vec<Array1<f64>>>,
}
#[derive(Default)]
pub struct PairsAndDistances;
impl PairsAndDistances {
    fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<u32>,
    ) -> Result<(Vec<Array1<i8>>, Vec<Array1<f64>>), CalcError> {
        y.columns()
            .into_iter()
            .map(|c| self.helper(x, &c))
            .collect::<Result<Vec<(Array1<i8>, Array1<f64>)>, CalcError>>()
            .map(|v| v.into_iter().map(|(a, b)| (a, b)).unzip())
    }
    fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<u32>,
    ) -> Result<(Array1<i8>, Array1<f64>), CalcError> {
        let n = y.len() * (y.len() - 1) / 2;
        let mut distances: Vec<f64> = Vec::with_capacity(n);
        let mut pairs_in_the_same_cluster: Vec<i8> = Vec::with_capacity(n);

        //calculating distances beetween pair of points and does they belong to the same cluster
        for (i, (row1, cluster1)) in x.axis_iter(Axis(0)).zip(y).enumerate() {
            for (j, (row2, cluster2)) in x.axis_iter(Axis(0)).zip(y).enumerate() {
                if i < j {
                    pairs_in_the_same_cluster.push((cluster1 == cluster2) as i8); // the same cluster =producer 1, different = 0
                    distances.push((&row2 - &row1).pow2().sum().sqrt());
                }
            }
        }
        let pairs_in_the_same_cluster = Array1::from_vec(pairs_in_the_same_cluster);
        let distances = Array1::from_vec(distances);
        Ok((pairs_in_the_same_cluster, distances))
    }
}

pub struct PairsAndDistancesNode<'a> {
    index: PairsAndDistances,
    sender: Sender<'a, PairsAndDistancesValue>,
}
impl<'a> PairsAndDistancesNode<'a> {
    pub fn new(sender: Sender<'a, PairsAndDistancesValue>) -> Self {
        Self {
            index: PairsAndDistances,
            sender,
        }
    }
}
impl<'a> Subscriber<RawDataValue<'a>> for PairsAndDistancesNode<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        let res = match data.as_ref() {
            Ok(rd) => self
                .index
                .compute(&rd.x, &rd.y)
                .map(|(p, d)| PairsAndDistancesValue {
                    pairs: Arc::new(p),
                    distances: Arc::new(d),
                }),
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
