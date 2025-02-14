use crate::calc_error::{CalcError, CombineErrors};
use ndarray::{ArcArray2, Array2, ArrayView2};
use ndarray_linalg::Determinant;

use super::helpers::{total_dispercion::TDValue, within_group_dispercion::WGDValue};
use crate::sender::{Sender, Subscriber};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RubinIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;
impl Index {
    fn compute(&self, wg: &Vec<Array2<f64>>, td: &ArcArray2<f64>) -> Result<Vec<f64>, CalcError> {
        wg.into_iter().map(|w| self.helper(w, &td.view())).collect()
    }
    fn helper(&self, wg: &Array2<f64>, td: &ArrayView2<f64>) -> Result<f64, CalcError> {
        let det_t = td.det().map_err(|e| CalcError::from(format!("{e:?}")))?;
        let det_wg = wg.det().map_err(|e| CalcError::from(format!("{e:?}")))?;
        Ok(det_t / det_wg)
    }
}

pub struct Node<'a> {
    index: Index,
    wg: Option<Result<WGDValue, CalcError>>,
    td: Option<Result<TDValue, CalcError>>,
    sender: Sender<'a, RubinIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, RubinIndexValue>) -> Self {
        Self {
            index: Index,
            wg: None,
            td: None,
            sender,
        }
    }
    fn process_when_ready(&mut self) {
        if let (Some(wg), Some(td)) = (self.wg.as_ref(), self.td.as_ref()) {
            let res = match wg.combine(td) {
                Ok((wg, td)) => self
                    .index
                    .compute(&wg.val, &td.val)
                    .map(|val| RubinIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.wg = None;
            self.td = None;
        }
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
