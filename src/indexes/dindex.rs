use crate::calc_error::{CalcError, CombineErrors};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use std::iter::zip;

use super::helpers::{
    clusters_centroids::ClustersCentroidsValue, counts::CountsValue, raw_data::RawDataValue,
};
use crate::sender::{Sender, Subscriber};
use itertools::izip;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct DIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    pub fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<u32>,
        clusters_centroids: &Vec<Array2<f64>>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<Vec<f64>, CalcError> {
        izip!(y.columns(), clusters_centroids, counts,)
            .map(|(y, clcntrds, cnts)| {
                let dindex = self.helper(x, &y, &clcntrds, cnts)?;
                let val = dindex;
                Ok(val)
            })
            .collect()
    }
    fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<u32>,
        clusters_centroids: &Array2<f64>,
        counts: &Vec<usize>,
    ) -> Result<f64, CalcError> {
        let q = counts.iter().sum::<usize>();
        let mut stor: Array1<f64> = Array1::default((q));
        for (row, c) in zip(x.rows(), y) {
            let dist = (&clusters_centroids.row(*c as usize) - &row).pow2().sum().sqrt();
            stor[*c as usize] += dist;
        }
        stor.iter_mut()
            .enumerate()
            .map(|(i, elem)| *elem /= counts[i] as f64);
        stor.mean().ok_or("Cant calc mean".into())
    }
}

pub struct Node<'a> {
    index: Index,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
    sender: Sender<'a, DIndexValue>,
}

impl<'a> Node<'a> {
    fn process_when_ready(&mut self) {
        if let (Some(raw_data), Some(clusters_centroids), Some(counts)) = (
            self.raw_data.as_ref(),
            self.clusters_centroids.as_ref(),
            self.counts.as_ref(),
        ) {
            let res = match raw_data.combine(clusters_centroids).combine(counts) {
                Ok(((rd, cls_ctrds), counts)) => self
                    .index
                    .compute(&rd.x, &rd.y, &cls_ctrds.val, &counts.val)
                    .map(|val| DIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.raw_data = None;
            self.clusters_centroids = None;
        }
    }
    pub fn new(sender: Sender<'a, DIndexValue>) -> Self {
        Self {
            index: Index,
            raw_data: None,
            clusters_centroids: None,
            counts: None,
            sender,
        }
    }
}

impl<'a> Subscriber<RawDataValue<'a>> for Node<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        self.raw_data = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<ClustersCentroidsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<ClustersCentroidsValue, CalcError>) {
        self.clusters_centroids = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<CountsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<CountsValue, CalcError>) {
        self.counts = Some(data);
        self.process_when_ready();
    }
}
