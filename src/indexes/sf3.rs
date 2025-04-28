use crate::calc_error::{CalcError, CombineErrors};
use crate::sender::{Sender, Subscriber};
use itertools::izip;
use ndarray::{Array2, ArrayView1, ArrayView2, Axis};
use std::f64;
use std::sync::Arc;

use super::helpers::clusters_centroids::ClustersCentroidsValue;
use super::helpers::counts::CountsValue;
use super::helpers::raw_data::RawDataValue;

#[derive(Clone, Debug)]
pub struct SF3IndexValue {
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
                let val = self.helper(x, &y, &clcntrds, cnts)?;
                Ok(val)
            })
            .collect()
    }
    fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<u32>,
        centroids: &Array2<f64>,
        counts: &Vec<usize>,
    ) -> Result<f64, CalcError> {
        let n = counts.iter().sum::<usize>() as f64;
        let q = counts.len() as f64;
        let centroids_mean = centroids.mean_axis(Axis(0)).ok_or("Cant find mean")?;
        let mut sum1 = 0.;
        for (nk,c) in izip!(counts,centroids.rows()){
            sum1+=(*nk as f64)*(&centroids_mean-&c).pow2().sum();
        }
        let mut dists: Vec<f64> = vec![0.;q as usize];
        for (x_row,y_row) in izip!(x.rows(),y){
            let dist = (&centroids.row(*y_row as usize)-&x_row).pow2().sum().sqrt();
            dists[*y_row as usize]+=dist;
        } 
        for (d,c) in izip!(dists.iter_mut(),counts){
            *d /=*c as f64;
        }
        let sum2 = dists.iter().sum::<f64>();
        let temp: f64 = sum1/(n*q)-sum2/q;
        let val = 1.-1./temp.exp().exp();
        Ok(val)

    }
}
pub struct Node<'a> {
    index: Index,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
    sender: Sender<'a, SF3IndexValue>,
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
                    .map(|val|SF3IndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.raw_data = None;
            self.clusters_centroids = None;
        }
    }
    pub fn new(sender: Sender<'a, SF3IndexValue>) -> Self {
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
