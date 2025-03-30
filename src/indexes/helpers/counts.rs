use crate::{
    calc_error::CalcError,
    sender::{Sender, Subscriber},
};
use itertools::Itertools;
use ndarray::ArrayView2;
use std::sync::Arc;

use super::raw_data::RawDataValue;
#[derive(Clone, Debug)]
pub struct CountsValue {
    pub val: Arc<Vec<Vec<usize>>>,
}
#[derive(Default)]
pub struct Counts;
impl Counts {
    pub fn compute(&self, clusters: &ArrayView2<u32>) -> Result<Vec<Vec<usize>>, CalcError> {
        let counts = clusters
            .columns()
            .into_iter()
            .map(|c| {
                c.iter()
                    .counts()
                    .into_iter()
                    .sorted_by(|(a, _), (b, _)| a.cmp(b))
                    .map(|(_, v)| v)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        Ok(counts)
        // let counts = clusters.iter().counts();
        // let vec = counts
        //     .into_iter()
        //     .sorted_by(|(a, _), (b, _)| a.cmp(b))
        //     .map(|(_, v)| v)
        //     .collect::<Vec<usize>>();
        // let res = Array1::from_vec(vec);
        // Ok(res.to_shared())
    }
}
pub struct CountsNode<'a> {
    index: Counts,
    sender: Sender<'a, CountsValue>,
}
impl<'a> CountsNode<'a> {
    pub fn new(sender: Sender<'a, CountsValue>) -> Self {
        Self {
            index: Counts,
            sender,
        }
    }
}

impl<'a> Subscriber<RawDataValue<'a>> for CountsNode<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        let res = match data {
            Ok(data) => self
                .index
                .compute(&data.y)
                .map(|v| CountsValue { val: Arc::new(v) }),
            Err(err) => Err(err),
        };
        self.sender.send_to_subscribers(res);
    }
}
