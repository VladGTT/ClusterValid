use crate::calc_error::{CalcError, CombineErrors};
use crate::sender::{Sender, Subscriber};
use itertools::izip;
use ndarray::{ArcArray2, Array2, ArrayView2};
use std::sync::Arc;

use super::helpers::clusters_centroids::ClustersCentroidsValue;
use super::helpers::within_group_dispercion::WGDValue;
use super::helpers::total_dispercion::TDValue;

#[derive(Clone, Debug)]
pub struct PBMIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    fn compute(
        &self,
        wg: &Vec<Array2<f64>>,
        td: &ArcArray2<f64>,
        centroids: &Vec<Array2<f64>>,
    ) -> Result<Vec<f64>, CalcError> {
        izip!(wg, centroids)
            .map(|(w, c)| self.helper(w, &td.view(), c))
            .collect()
    }
    fn helper(
        &self,
        wg: &Array2<f64>,
        td: &ArrayView2<f64>,
        centroids: &Array2<f64>,
    ) -> Result<f64, CalcError> {
        let q = centroids.nrows();
        let tss = td.diag().sum();
        let wgss = wg.diag().sum();

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
        let val = ((max/q as f64)*(tss/wgss)).powi(2);
        Ok(val)
    }
}

pub struct Node<'a> {
    index: Index,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    wg: Option<Result<WGDValue, CalcError>>,
    td: Option<Result<TDValue, CalcError>>,
    sender: Sender<'a, PBMIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, PBMIndexValue>) -> Self {
        Self {
            index: Index,
            wg: None,
            td: None,
            clusters_centroids: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(wg), Some(td), Some(clusters_centroids)) =
            (self.wg.as_ref(), self.td.as_ref(), self.clusters_centroids.as_ref())
        {
            let res = match wg.combine(td).combine(clusters_centroids) {
                Ok(((wg, td), cntrds)) => self
                    .index
                    .compute(&wg.val, &td.val, &cntrds.val)
                    .map(|val| PBMIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
            self.td = None;
            self.clusters_centroids = None;
        }
    }
}
impl<'a> Subscriber<ClustersCentroidsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<ClustersCentroidsValue, CalcError>) {
        self.clusters_centroids = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<WGDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGDValue, CalcError>) {
        self.wg = Some(data);
        self.process_when_ready();
    }
}

impl<'a> Subscriber<TDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<TDValue, CalcError>) {
        self.td = Some(data);
        self.process_when_ready();
    }
}
