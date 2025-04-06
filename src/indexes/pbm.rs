use crate::calc_error::{CalcError, CombineErrors};
use crate::sender::{Sender, Subscriber};
use itertools::izip;
use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use std::sync::Arc;

use super::helpers::clusters_centroids::ClustersCentroidsValue;
use super::helpers::counts::CountsValue;
use super::helpers::raw_data::RawDataValue;

#[derive(Clone, Debug)]
pub struct PBMIndexValue {
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
        let q = centroids.nrows();

        let x_centroid = x.mean_axis(Axis(0)).ok_or("Cant calc mean")?;
        let mut e_w = 0.;
        let mut e_t = 0.;
        for (x_row,y_row) in izip!(x.rows(),y){
            let dist = (&centroids.row(*y_row as usize)-&x_row).pow2().sum().sqrt();
            let dist2 = (&x_centroid - &x_row).pow2().sum().sqrt();

            e_w+=dist;
            e_t+=dist2;
        }
        let mut max = f64::MIN;
        for (i,center1) in centroids.rows().into_iter().enumerate(){
            for (j,center2) in centroids.rows().into_iter().enumerate(){
                if i<j{
                    let dist = (&center1-&center2).pow2().sum().sqrt();
                    if dist > max{
                        max = dist;
                    }
                }
            }
        }
        let val = ((max/q as f64)*(e_t/e_w)).powi(2);
        Ok(val)
    }
}
pub struct Node<'a> {
    index: Index,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
    sender: Sender<'a, PBMIndexValue>,
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
                    .map(|val|PBMIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.raw_data = None;
            self.clusters_centroids = None;
        }
    }
    pub fn new(sender: Sender<'a, PBMIndexValue>) -> Self {
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
