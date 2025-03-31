use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use ndarray::Array1;
use std::iter::zip;
use std::sync::Arc;
use std::cmp::Ordering;
use super::pairs_and_distances::PairsAndDistancesValue;

#[derive(Clone, Debug)]
pub struct SPlusAndMinusValue {
    pub s_plus: Arc<Vec<isize>>,
    pub s_minus: Arc<Vec<isize>>,
    pub ties: Arc<Vec<isize>>,
}

#[derive(Default)]
pub struct Index;
impl Index {
    fn compute(
        &self,
        pairs_in_the_same_cluster: &Vec<Array1<i8>>,
        distances: &Vec<Array1<f64>>,
    ) -> Result<(Vec<isize>, Vec<isize>, Vec<isize>), CalcError> {
        zip(pairs_in_the_same_cluster, distances)
            .map(|(p, d)| self.helper(p, d))
            .collect::<Result<Vec<(isize, isize, isize)>, CalcError>>()
            .map(|v| {
                let (first, rest): (Vec<_>, Vec<_>) =
                    v.into_iter().map(|(x, y, z)| (x, (y, z))).unzip();
                let (second, third): (Vec<_>, Vec<_>) = rest.into_iter().unzip();
                (first, second, third)
            })
    }
    fn helper(
        &self,
        pairs_in_the_same_cluster: &Array1<i8>,
        distances: &Array1<f64>,
    ) -> Result<(isize, isize, isize), CalcError> {
        // // finding s_plus which represents the number of times a distance between two points
        // // which belong to the same cluster is strictly smaller than the distance between two points not belonging to the same cluster
        // // and s_minus which represents the number of times distance between two points lying in the same cluster  is strictly greater than a distance between two points not
        // //belonging to the same cluster
        //
        // let (mut s_plus, mut s_minus, mut ties) = (0, 0, 0);
        //
        // for (&d1, &b1) in distances.iter().zip(pairs_in_the_same_cluster) {
        //     if b1 == 1 {
        //         for (&d2, &b2) in distances.iter().zip(pairs_in_the_same_cluster) {
        //             if b2 == 0 {
        //                 if d1 < d2 {
        //                     s_plus += 1;
        //                 } else if d1 > d2 {
        //                     s_minus += 1;
        //                 } else {
        //                     ties += 1;
        //                 }
        //             }
        //         }
        //     }
        // }
        //
        // Ok((s_plus, s_minus, ties))
let mut s_plus = 0;
    let mut s_minus = 0;
    let mut ties = 0;

    // Collect and sort distances
    let mut cluster_distances: Vec<f64> = distances
        .iter()
        .zip(pairs_in_the_same_cluster.iter())
        .filter_map(|(&dist, &label)| if label == 1 { Some(dist) } else { None })
        .collect();
    
    let mut non_cluster_distances: Vec<f64> = distances
        .iter()
        .zip(pairs_in_the_same_cluster.iter())
        .filter_map(|(&dist, &label)| if label == 0 { Some(dist) } else { None })
        .collect();
    
    cluster_distances.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    non_cluster_distances.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    // Two-pointer approach for efficient comparison
    let (mut i, mut j) = (0, 0);
    let m = cluster_distances.len();
    let n = non_cluster_distances.len();

    while i < m && j < n {
        match cluster_distances[i].partial_cmp(&non_cluster_distances[j]) {
            Some(Ordering::Less) => {
                s_plus += (n - j) as isize; // All remaining elements in non_cluster_distances are greater
                i += 1;
            }
            Some(Ordering::Greater) => {
                s_minus += (m - i) as isize; // All remaining elements in cluster_distances are greater
                j += 1;
            }
            Some(Ordering::Equal) => {
                ties += 1;
                i += 1;
                j += 1;
            }
            None => return Err("Comparison failed due to NaN values".to_string().into()),
        }
    }

    Ok((s_plus, s_minus, ties))
    }
}
#[derive(Default)]
pub struct SPlusAndMinusNode<'a> {
    index: Index,
    sender: Sender<'a, SPlusAndMinusValue>,
}

impl<'a> SPlusAndMinusNode<'a> {
    pub fn new(sender: Sender<'a, SPlusAndMinusValue>) -> Self {
        Self {
            index: Index,
            sender,
        }
    }
}
impl<'a> Subscriber<PairsAndDistancesValue> for SPlusAndMinusNode<'a> {
    fn recieve_data(&mut self, data: Result<PairsAndDistancesValue, CalcError>) {
        let res = match data.as_ref() {
            Ok(pd) => {
                self.index
                    .compute(&pd.pairs, &pd.distances)
                    .map(|(s_plus, s_minus, ties)| SPlusAndMinusValue {
                        s_plus: Arc::new(s_plus),
                        s_minus: Arc::new(s_minus),
                        ties: Arc::new(ties),
                    })
            }
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
