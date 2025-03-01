use crate::{
    calc_error::{CalcError, CombineErrors},
    indexes::helpers::{clusters_centroids::ClustersCentroidsValue, counts::CountsValue},
    sender::{Sender, Subscriber},
};

use itertools::izip;
use ndarray::{ArcArray2, Array2, ArrayView1, ArrayView2};

use super::{counts, raw_data::RawDataValue};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct WGSValue {
    pub val: Arc<Vec<Vec<Array2<f64>>>>,
}
#[derive(Default)]
pub struct WGD;
impl WGD {
    pub fn compute(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView2<usize>,
        clusters_centroids: &Vec<Array2<f64>>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<Vec<Vec<Array2<f64>>>, CalcError> {
        izip!(y.columns(), clusters_centroids, counts)
            .map(|(col, clscntrs, cnts)| self.helper(x, &col, &clscntrs, &cnts))
            .collect::<Result<Vec<Vec<Array2<f64>>>, CalcError>>()
    }
    fn helper(
        &self,
        x: &ArrayView2<f64>,
        y: &ArrayView1<usize>,
        clusters_centroids: &Array2<f64>,
        counts: &Vec<usize>,
    ) -> Result<Vec<Array2<f64>>, CalcError> {
        let mut retval: Vec<Array2<f64>> = Vec::with_capacity(counts.len());
        for _ in counts {
            retval.push(Array2::zeros((0, x.ncols())));
        }
        for (x, y) in izip!(x.rows(), y) {
            let dif = &x - &clusters_centroids.row(*y);
            retval[*y].push_row(dif.view());
        }
        let retval = retval
            .iter()
            .map(|elem| elem.t().dot(elem))
            .collect::<Vec<Array2<f64>>>();
        Ok(retval)
    }
}
pub struct WGDNode<'a> {
    index: WGD,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    raw_data: Option<Result<RawDataValue<'a>, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
    sender: Sender<'a, WGSValue>,
}
impl<'a> WGDNode<'a> {
    pub fn new(sender: Sender<'a, WGSValue>) -> Self {
        Self {
            index: WGD,
            clusters_centroids: None,
            raw_data: None,
            counts: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(clusters_centroids), Some(raw_data), Some(counts)) = (
            self.clusters_centroids.as_ref(),
            self.raw_data.as_ref(),
            self.counts.as_ref(),
        ) {
            let res = match clusters_centroids.combine(raw_data).combine(counts) {
                Ok(((cls_ctrs, rd), cnts)) => self
                    .index
                    .compute(&rd.x, &rd.y, &cls_ctrs.val, &cnts.val)
                    .map(|val| WGSValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.raw_data = None;
            self.clusters_centroids = None;
            self.counts = None;
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
impl<'a> Subscriber<CountsValue> for WGDNode<'a> {
    fn recieve_data(&mut self, data: Result<CountsValue, CalcError>) {
        self.counts = Some(data);
        self.process_when_ready();
    }
}
