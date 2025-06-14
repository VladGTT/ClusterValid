use crate::calc_error::{CalcError,CombineErrors};
use itertools::izip;
use ndarray::{ArcArray1, Array1,Array2, ArrayView1};

use crate::sender::{Sender, Subscriber};
use std::f64;
use std::{fmt::format, iter::zip};
use std::sync::Arc;

use super::helpers::clusters_centroids::{self, ClustersCentroidsValue};
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
        clusters_centroids: &Vec<Array2<f64>>,
        wg: &Vec<Array2<f64>>,
        counts: &Vec<Vec<usize>>,
    ) -> Result<Vec<f64>, CalcError> {
        izip!(clusters_centroids,wg,counts)
            .map(|(clc,w,c)| self.helper(clc,w,c))
            .collect()
    }
    fn helper(
        &self,
        clusters_centroids: &Array2<f64>,
        wg: &Array2<f64>,
        counts: &Vec<usize>
    ) -> Result<f64, CalcError> {

        let n = counts.iter().sum::<usize>() as f64;
        let mut min_dist = f64::MAX;
        for (i,c1) in clusters_centroids.rows().into_iter().enumerate(){
            for (j,c2) in clusters_centroids.rows().into_iter().enumerate(){
                if i<j{
                    let dist = (&c1-&c2).pow2().sum();
                    if dist<min_dist{
                        min_dist = dist;
                    }
                }   
            }
        }
        let val = wg.diag().sum()/min_dist/n;
        Ok(val)
    }
}

pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, XieBeniIndexValue>,
    wg: Option<Result<WGDValue, CalcError>>,
    clusters_centroids: Option<Result<ClustersCentroidsValue, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, XieBeniIndexValue>) -> Self {
        Self {
            index: Index,
            wg: None,
            clusters_centroids: None,
            counts: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(clusters_centroids), Some(wg),Some(counts)) =
            (self.clusters_centroids.as_ref(), self.wg.as_ref(),self.counts.as_ref())
        {
            let res = match clusters_centroids.combine(wg).combine(counts) {
                Ok(((clc, wg),counts)) => self
                    .index
                    .compute(&clc.val, &wg.val,&counts.val)
                    .map(|val| XieBeniIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
            self.clusters_centroids = None;
        }
    }

}

impl<'a> Subscriber<WGDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGDValue, CalcError>) {
        self.wg = Some(data);
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

