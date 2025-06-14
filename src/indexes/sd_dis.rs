use std::sync::Arc;

use crate::calc_error::CalcError;
use ndarray::Array2;

use crate::sender::{Sender, Subscriber};

use super::helpers::clusters_centroids::ClustersCentroidsValue;

#[derive(Clone, Debug)]
pub struct SDDisIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    pub fn compute(&self, clusters_centroids: &Vec<Array2<f64>>) -> Result<Vec<f64>, CalcError> {
        clusters_centroids
            .into_iter()
            .map(|c| self.helper(c))
            .collect()
    }
    fn helper(&self, clusters_centroids: &Array2<f64>) -> Result<f64, CalcError> {
        let mut d = 0.0;
        let mut d_max = f64::MIN;
        let mut d_min = f64::MAX;
        for (i, row1) in clusters_centroids.rows().into_iter().enumerate() {
            let mut dist_acum = 0.0;
            for (j, row2) in clusters_centroids.rows().into_iter().enumerate() {
                if i != j {
                    let dist = (&row2 - &row1).pow2().sum().sqrt();
                    dist_acum += dist;
                    if i < j {
                        if dist > d_max {
                            d_max = dist;
                        }
                        if dist < d_min {
                            d_min = dist;
                        }
                    }
                }
            }
            if dist_acum != 0.0 {
                d += 1. / dist_acum;
            }
        }
        let value = d * d_max / d_min;

        Ok(value)
    }
}

pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, SDDisIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, SDDisIndexValue>) -> Self {
        Self {
            index: Index,
            sender,
        }
    }
}

impl<'a> Subscriber<ClustersCentroidsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<ClustersCentroidsValue, CalcError>) {
        let res = match data {
            Ok(cls_ctrds) => self
                .index
                .compute(&cls_ctrds.val)
                .map(|val| SDDisIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err),
        };
        self.sender.send_to_subscribers(res);
    }
}
// impl Computable for IndexDis {
//     fn compute(&self, x: ArrayView2<f64>, y: ArrayView1<i32>) -> Result<f64, CalcError> {
//         let mut d = 0.0;
//         let mut d_max = f64::MIN;
//         let mut d_min = f64::MAX;
//         let clusters = calc_clusters(&y);
//         let cluster_centroids = calc_clusters_centers(&clusters, &x);
//
//         for (i, row1) in &cluster_centroids {
//             let mut dist_acum = 0.0;
//             for (j, row2) in &cluster_centroids {
//                 if i != j {
//                     let dist = find_euclidean_distance(&row1.view(), &row2.view());
//                     dist_acum += dist;
//                     if i < j {
//                         if dist > d_max {
//                             d_max = dist;
//                         }
//                         if dist < d_min {
//                             d_min = dist;
//                         }
//                     }
//                 }
//             }
//             if dist_acum != 0.0 {
//                 d += 1. / dist_acum;
//             }
//         }
//         let value = d * d_max / d_min;
//         Ok(value)
//     }
// }
