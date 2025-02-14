use crate::{
    calc_error::{CalcError, CombineErrors},
    indexes::helpers::clusters_centroids::ClustersCentroidsValue,
    sender::{Sender, Subscriber},
};

use ndarray::{ArcArray2, Array2, ArrayView1, ArrayView2};
use std::iter::zip;

use super::raw_data::RawDataValue;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct WGDValue {
    pub val: Arc<Vec<Array2<f64>>>,
}
#[derive(Default)]
pub struct WGD;
impl WGD {
    pub fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<usize>,
        clusters_centroids: &Vec<Array2<f64>>,
    ) -> Result<Vec<Array2<f64>>, CalcError> {
        zip(y.columns(), clusters_centroids)
            .map(|(col, clscntrs)| self.helper(x, &col, &clscntrs))
            .collect::<Result<Vec<Array2<f64>>, CalcError>>()
    }
    fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<usize>,
        clusters_centroids: &Array2<f64>,
    ) -> Result<Array2<f64>, CalcError> {
        let (n, d) = x.dim();
        let mut dif: Array2<f64> = Array2::zeros((n, d));
        for (i, (x, y)) in zip(x.rows(), y).enumerate() {
            let temp = &clusters_centroids.row(*y as usize) - &x;
            dif.row_mut(i).assign(&temp);
        }
        let wg = dif.t().dot(&dif);
        Ok(wg)
    }
}
pub struct WGDNode<'a> {
    index: WGD,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    sender: Sender<'a, WGDValue>,
}
impl<'a> WGDNode<'a> {
    pub fn new(sender: Sender<'a, WGDValue>) -> Self {
        Self {
            index: WGD,
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
                    .map(|val| WGDValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.raw_data = None;
            self.clusters_centroids = None;
        }
    }
}
impl<'a> Subscriber<ClustersCentroidsValue> for WGDNode<'a> {
    fn recieve_data(&mut self, data: Result<ClustersCentroidsValue, CalcError>) {
        self.clusters_centroids = Some(data);
        self.process_when_ready();
    }
}

impl<'a> Subscriber<RawDataValue<'a>> for WGDNode<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        self.raw_data = Some(data);
        self.process_when_ready();
    }
}
