use super::helpers::pairs_and_distances::PairsAndDistancesValue;
use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use itertools::izip;
use ndarray::Array1;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FreyIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    fn compute(
        &self,
        pairs_in_the_same_cluster: &Vec<Array1<i8>>,
        distances: &[Array1<f64>],
    ) -> Result<Vec<f64>, CalcError> {
        let mut retval = vec![f64::NAN;pairs_in_the_same_cluster.len()];
        for i in 0..pairs_in_the_same_cluster.len()-1{

            let (sb, sw) = self.helper(&pairs_in_the_same_cluster[i], &distances[i])?;
            let (sb_plus_one, sw_plus_one) = self.helper(&pairs_in_the_same_cluster[i+1], &distances[i+1])?;
            let val = (sb_plus_one-sb)/(sw_plus_one-sw);
            retval[i]=val;
        }
        Ok(retval)

    }
    fn helper(&self, p: &Array1<i8>, d: &Array1<f64>) -> Result<(f64, f64), CalcError> {
        let nw = p.iter().filter(|i| **i == 1).count() as f64;
        let sw = izip!(p, d)
            .filter(|(p, _)| **p == 1)
            .map(|(_, d)| *d)
            .sum::<f64>();
        let nb = p.iter().filter(|i| **i == 0).count() as f64;
        let sb = izip!(p, d)
            .filter(|(p, _)| **p == 0)
            .map(|(_, d)| *d)
            .sum::<f64>();
        // let std_d = distances.std(0.);
        // let val = ((sb / nb - sw / nw) * (nw * nb / (nt * nt)).sqrt()) / std_d;
        Ok((sb / nb, sw / nw))
    }
}
pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, FreyIndexValue>,
}
impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, FreyIndexValue>) -> Self {
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
                .map(|val| FreyIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
