use crate::{
    calc_error::{CalcError, CombineErrors},
    sender::{Sender, Subscriber},
};
use ndarray::{Array2, ArrayView1, ArrayView2, Axis};

use super::{clusters_centroids::ClustersCentroidsValue, raw_data::RawDataValue};
use std::iter::zip;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct BGDValue {
    pub val: Arc<Vec<Array2<f64>>>,
}

#[derive(Default)]
pub struct BGD;
impl BGD {
    pub fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<u32>,
        clusters_centroids: &Vec<Array2<f64>>,
    ) -> Result<Vec<Array2<f64>>, CalcError> {
        zip(y.columns(), clusters_centroids)
            .map(|(col, clscntrs)| self.helper(x, &col, &clscntrs))
            .collect::<Result<Vec<Array2<f64>>, CalcError>>()
    }
    fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<u32>,
        clusters_centroids: &Array2<f64>,
    ) -> Result<Array2<f64>, CalcError> {
        let (n, d) = x.dim();
        let data_center = x.mean_axis(Axis(0)).ok_or("Cant calc data centroid")?;
        let mut b: Array2<f64> = Array2::zeros((n, d));
        for (i, y) in y.iter().enumerate() {
            let temp = &data_center - &clusters_centroids.row(*y as usize);
            b.row_mut(i).assign(&temp);
        }
        let bg = b.t().dot(&b);
        Ok(bg)
    }
}
pub struct BGDNode<'a> {
    index: BGD,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    sender: Sender<'a, BGDValue>,
}
impl<'a> BGDNode<'a> {
    pub fn new(sender: Sender<'a, BGDValue>) -> Self {
        Self {
            index: BGD,
            clusters_centroids: None,
            raw_data: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(clusters_centroids), Some(raw_data)) =
            (self.clusters_centroids.as_ref(), self.raw_data.as_ref())
        {
            let res = match clusters_centroids.combine(raw_data) {
                Ok((cls_ctrs, rd)) => self
                    .index
                    .compute(&rd.x, &rd.y, &cls_ctrs.val)
                    .map(|val| BGDValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.raw_data = None;
            self.clusters_centroids = None;
        }
    }
}
impl<'a> Subscriber<ClustersCentroidsValue> for BGDNode<'a> {
    fn recieve_data(&mut self, data: Result<ClustersCentroidsValue, CalcError>) {
        self.clusters_centroids = Some(data);
        self.process_when_ready();
    }
}

impl<'a> Subscriber<RawDataValue<'a>> for BGDNode<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        self.raw_data = Some(data);
        self.process_when_ready();
    }
}
