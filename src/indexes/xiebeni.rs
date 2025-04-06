use crate::calc_error::{CalcError,CombineErrors};
use itertools::izip;
use ndarray::{ArcArray1, Array1,Array2, ArrayView1};

use crate::sender::{Sender, Subscriber};
use std::{fmt::format, iter::zip};
use std::sync::Arc;

use super::helpers::{counts::CountsValue, pairs_and_distances::{self, PairsAndDistancesValue}, within_group_dispercion::WGDValue};
#[derive(Clone, Debug)]
pub struct XieBeniIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    pub fn compute(
        &self,
        pairs_in_the_same_cluster: &Vec<Array1<i8>>,
        distances: &Vec<Array1<f64>>,
        wg: &Vec<Array2<f64>>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<Vec<f64>, CalcError> {
        izip!(pairs_in_the_same_cluster, distances,wg,counts)
            .map(|(p, d,w,c)| self.helper(p, d,w,c))
            .collect()
    }
    fn helper(
        &self,
        pairs_in_the_same_cluster: &Array1<i8>,
        distances: &Array1<f64>,
        wg: &Array2<f64>,
        counts: &Vec<usize>
    ) -> Result<f64, CalcError> {

        let n = counts.iter().sum::<usize>() as f64;
        let min_intercluster = zip(pairs_in_the_same_cluster, distances)
            .filter(|(i, _)| **i == 0)
            .map(|(_, d)| *d)
            .min_by(|a, b| a.total_cmp(b))
            .ok_or("Can't find min intercluster distance")?;
        let val = wg.diag().sum()/min_intercluster.powi(2)/n;
        Ok(val)
    }
}

pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, XieBeniIndexValue>,
    wg: Option<Result<WGDValue, CalcError>>,
    pairs_and_distances: Option<Result<PairsAndDistancesValue, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, XieBeniIndexValue>) -> Self {
        Self {
            index: Index,
            wg: None,
            pairs_and_distances: None,
            counts: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(pairs_and_distances), Some(wg),Some(counts)) =
            (self.pairs_and_distances.as_ref(), self.wg.as_ref(),self.counts.as_ref())
        {
            let res = match pairs_and_distances.combine(wg).combine(counts) {
                Ok(((pd, wg),counts)) => self
                    .index
                    .compute(&pd.pairs, &pd.distances, &wg.val,&counts.val)
                    .map(|val| XieBeniIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
            self.pairs_and_distances = None;
        }
    }

}

impl<'a> Subscriber<WGDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGDValue, CalcError>) {
        self.wg = Some(data);
        self.process_when_ready();
    }
}

impl<'a> Subscriber<PairsAndDistancesValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<PairsAndDistancesValue, CalcError>) {
        self.pairs_and_distances = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<CountsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<CountsValue, CalcError>) {
        self.counts = Some(data);
        self.process_when_ready();
    }
}

