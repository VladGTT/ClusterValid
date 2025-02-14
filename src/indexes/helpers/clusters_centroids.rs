use crate::{
    calc_error::{CalcError, CombineErrors},
    sender::{Sender, Subscriber},
};
use ndarray::{ArcArray1, ArcArray2, Array2, ArrayView1, ArrayView2};
use std::iter::zip;

use super::{counts::CountsValue, raw_data::RawDataValue};
#[derive(Clone, Debug)]
pub struct ClustersCentroidsValue {
    pub val: Vec<Array2<f64>>,
}
#[derive(Default)]
pub struct ClustersCentroids;
impl ClustersCentroids {
    pub fn compute(
        &self,
        data: &ArrayView2<f64>,
        clusters: &ArrayView2<usize>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<Vec<Array2<f64>>, CalcError> {
        zip(clusters.columns(), counts)
            .map(|(cl, cnts)| self.compute_centroids_for_column(data, &cl, cnts))
            .collect::<Result<Vec<Array2<f64>>, CalcError>>()
    }
    fn compute_centroids_for_column(
        &self,
        data: &ArrayView2<f64>,
        clusters: &ArrayView1<usize>,
        counts: &Vec<usize>,
    ) -> Result<Array2<f64>, CalcError> {
        let q = counts.len();
        let mut centroids: Array2<f64> = Array2::default((q, data.ncols()));
        for (x, y) in zip(data.rows(), clusters.iter()) {
            let mut r = centroids.row_mut(*y as usize);
            r += &(&x / counts[*y as usize] as f64);
        }
        Ok(centroids)
    }
}
pub struct ClustersCentroidsNode<'a> {
    index: ClustersCentroids,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
    sender: Sender<'a, ClustersCentroidsValue>,
}
impl<'a> ClustersCentroidsNode<'a> {
    pub fn new(sender: Sender<'a, ClustersCentroidsValue>) -> Self {
        Self {
            index: ClustersCentroids,
            raw_data: None,
            counts: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(raw_data), Some(counts)) = (self.raw_data.as_ref(), self.counts.as_ref()) {
            let res = match raw_data.combine(counts) {
                Ok((raw_data, cnts)) => self
                    .index
                    .compute(&raw_data.x, &raw_data.y, &cnts.val)
                    .map(|val| ClustersCentroidsValue { val }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.raw_data = None;
            self.counts = None;
        }
    }
}

impl<'a> Subscriber<RawDataValue<'a>> for ClustersCentroidsNode<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        self.raw_data = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<CountsValue> for ClustersCentroidsNode<'a> {
    fn recieve_data(&mut self, data: Result<CountsValue, CalcError>) {
        self.counts = Some(data);
        self.process_when_ready();
    }
}
