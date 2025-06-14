use crate::calc_error::{CalcError, CombineErrors};
use ndarray::Array2;
use ndarray_linalg::Determinant;

use crate::sender::{Sender, Subscriber};

use super::helpers::{counts::CountsValue, within_group_dispercion::WGDValue};
use rand::distr::{Distribution, Uniform};
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct GapIndexValue {
    pub val: Arc<Vec<f64>>,
}

#[derive(Default)]
pub struct Index;
impl Index {
    pub fn compute(
        &self,
        counts: &Vec<Vec<usize>>,
        wg: &Vec<Array2<f64>>,
    ) -> Result<Vec<f64>, CalcError> {
        Err("Not implermented".into())
    }
    fn helper(&self, counts: &Vec<usize>, wg: &Array2<f64>) -> Result<f64, CalcError> {
        let b = 10;
        let n = counts.iter().sum();
        let p = wg.ncols();
        // min max for every feature
        // random in range (min max) for columns
        let uniform_distr = Uniform::new(10., 10000.).map_err(|e| e.to_string())?;
        let mut rng = rand::rng();
        for _ in 0..b {
            let mut data: Array2<f64> = Array2::zeros((n, p));
            data.iter_mut()
                .for_each(|v| *v = uniform_distr.sample(&mut rng));
        }
        let kmean: KMeans<_, 4, _> =
            KMeans::new(samples, sample_cnt, sample_dims, EuclideanDistance);
        let result = kmean.kmeans_lloyd(
            k,
            max_iter,
            KMeans::init_kmeanplusplus,
            &KMeansConfig::default(),
        );
        println!("Cluster-Assignments: {:?}", result.assignments);
        Err("Not implermented".into())
    }
}

pub struct Node<'a> {
    index: Index,
    counts: Option<Result<CountsValue, CalcError>>,
    wg: Option<Result<WGDValue, CalcError>>,
    sender: Sender<'a, GapIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, GapIndexValue>) -> Self {
        Self {
            index: Index,
            counts: None,
            wg: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(counts), Some(wg)) = (self.counts.as_ref(), self.wg.as_ref()) {
            let res = match wg.combine(counts) {
                Ok((wg, cnts)) => self
                    .index
                    .compute(&cnts.val, &wg.val)
                    .map(|val| GapIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
            self.counts = None;
        }
    }
}

impl<'a> Subscriber<CountsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<CountsValue, CalcError>) {
        self.counts = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<WGDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<WGDValue, CalcError>) {
        self.wg = Some(data);
        self.process_when_ready();
    }
}
