use super::raw_data::RawDataValue;
use crate::{
    calc_error::CalcError,
    sender::{Sender, Subscriber},
};
use ndarray::{ArcArray2, ArrayView2, Axis};

#[derive(Clone, Debug)]
pub struct TDValue {
    pub val: ArcArray2<f64>,
}

#[derive(Default)]
pub struct TD;
impl TD {
    pub fn compute(&self, x: &ArrayView2<f64>) -> Result<ArcArray2<f64>, CalcError> {
        let data_center = x.mean_axis(Axis(0)).ok_or("Cant calc data centroid")?;
        let t = x - &data_center;
        let td = t.t().dot(&t);
        Ok(td.into_shared())
    }
}
pub struct TDNode<'a> {
    index: TD,
    sender: Sender<'a, TDValue>,
}
impl<'a> TDNode<'a> {
    pub fn new(sender: Sender<'a, TDValue>) -> Self {
        Self { index: TD, sender }
    }
}

impl<'a> Subscriber<RawDataValue<'a>> for TDNode<'a> {
    fn recieve_data(&mut self, data: Result<RawDataValue<'a>, CalcError>) {
        let res = match data {
            Ok(rd) => self.index.compute(&rd.x).map(|val| TDValue { val }),
            Err(err) => Err(err),
        };
        self.sender.send_to_subscribers(res);
    }
}
