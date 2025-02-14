use crate::calc_error::{CalcError, CombineErrors};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use std::iter::zip;

use super::helpers::{clusters_centroids::ClustersCentroidsValue, raw_data::RawDataValue};
use crate::sender::{Sender, Subscriber};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct DaviesBouldinIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    pub fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<usize>,
        clusters_centroids: &Vec<Array2<f64>>,
    ) -> Result<Vec<f64>, CalcError> {
        zip(y.columns(), clusters_centroids)
            .map(|(c, clscntrs)| self.helper(x, &c, clscntrs))
            .collect()
    }
    fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<usize>,
        clusters_centroids: &Array2<f64>,
    ) -> Result<f64, CalcError> {
        let q = clusters_centroids.nrows();
        let mut distances_to_center: Vec<Vec<f64>> = Vec::new();
        distances_to_center.resize(q, Vec::default());
        for (x, y) in zip(x.rows(), y) {
            let d = (&x - &clusters_centroids.row(*y as usize))
                .pow2()
                .sum()
                .sqrt();
            distances_to_center[*y as usize].push(d);
        }
        let mean_distances = distances_to_center
            .into_iter()
            .map(|v| Array1::from_vec(v).mean())
            .collect::<Option<Vec<f64>>>()
            .ok_or("Cant calc mean")?;

        let mut stor = Array1::zeros(q);
        for i in 0..q {
            let mut arr = Vec::with_capacity(q - 1);
            for j in 0..q {
                if i != j {
                    let center_dist = (&clusters_centroids.row(i) - &clusters_centroids.row(j))
                        .pow2()
                        .sum()
                        .sqrt();
                    let coef = (mean_distances[i] + mean_distances[j]) / center_dist;
                    arr.push(coef);
                }
            }
            let max = arr
                .into_iter()
                .max_by(|a, b| a.total_cmp(b))
                .ok_or("Cant calc max")?;
            *stor.get_mut(i).ok_or("Cant save max val")? = max;
        }
        stor.mean().ok_or("Cant get mean".into())
    }
}

pub struct Node<'a> {
    index: Index,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    sender: Sender<'a, DaviesBouldinIndexValue>,
}

impl<'a> Node<'a> {
    fn process_when_ready(&mut self) {
        if let (Some(raw_data), Some(clusters_centroids)) =
            (self.raw_data.as_ref(), self.clusters_centroids.as_ref())
        {
            let res = match raw_data.combine(clusters_centroids) {
                Ok((rd, cls_ctrds)) => self
                    .index
                    .compute(&rd.x, &rd.y, &cls_ctrds.val)
                    .map(|val| DaviesBouldinIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.raw_data = None;
            self.clusters_centroids = None;
        }
    }
    pub fn new(sender: Sender<'a, DaviesBouldinIndexValue>) -> Self {
        Self {
            index: Index,
            raw_data: None,
            clusters_centroids: None,
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
