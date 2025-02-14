use crate::calc_error::{CalcError, CombineErrors};
use ndarray::{ArcArray2, Array2, ArrayView1, ArrayView2, Axis};
use std::{iter::zip, sync::Arc};

use crate::sender::{Sender, Subscriber};

use super::helpers::{
    between_group_dispercion::BGDValue, counts::CountsValue, total_dispercion::TDValue,
};

#[derive(Clone, Debug)]
pub struct RatkowskyIndexValue {
    pub val: Arc<Vec<f64>>,
}
#[derive(Default)]
pub struct Index;

impl Index {
    fn compute(
        &self,
        counts: &Vec<Vec<usize>>,
        tg: &ArcArray2<f64>,
        bg: &Vec<Array2<f64>>,
    ) -> Result<Vec<f64>, CalcError> {
        zip(counts, bg)
            .map(|(c, b)| self.helper(c, tg, b))
            .collect()
    }
    fn helper(
        &self,
        counts: &Vec<usize>,
        tg: &ArcArray2<f64>,
        bg: &Array2<f64>,
    ) -> Result<f64, CalcError> {
        let diag_bg = bg.diag();
        let diag_tg = tg.diag();
        let div = &diag_bg / &diag_tg;
        let r = div.mean().ok_or(CalcError::from("Cant calculate mean"))?;
        let q = counts.len() as f64;
        let value = (r / q).sqrt();
        Ok(value)
    }
}

pub struct Node<'a> {
    index: Index,
    bg: Option<Result<BGDValue, CalcError>>,
    tg: Option<Result<TDValue, CalcError>>,
    counts: Option<Result<CountsValue, CalcError>>,
    sender: Sender<'a, RatkowskyIndexValue>,
}

impl<'a> Node<'a> {
    fn process_when_ready(&mut self) {
        if let (Some(counts), Some(tg), Some(bg)) =
            (self.counts.as_ref(), self.tg.as_ref(), self.bg.as_ref())
        {
            let res = match counts.combine(tg).combine(bg) {
                Ok(((cnts, tg), bg)) => self
                    .index
                    .compute(&cnts.val, &tg.val, &bg.val)
                    .map(|val| RatkowskyIndexValue { val: Arc::new(val) }),
                Err(err) => Err(err),
            };
            self.sender.send_to_subscribers(res);
            self.counts = None;
            self.tg = None;
            self.bg = None;
        }
    }
    pub fn new(sender: Sender<'a, RatkowskyIndexValue>) -> Self {
        Self {
            index: Index,
            counts: None,
            tg: None,
            bg: None,
            sender,
        }
    }
}

impl<'a> Subscriber<CountsValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<CountsValue, CalcError>) {
        self.counts = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<TDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<TDValue, CalcError>) {
        self.tg = Some(data);
        self.process_when_ready();
    }
}
impl<'a> Subscriber<BGDValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<BGDValue, CalcError>) {
        self.bg = Some(data);
        self.process_when_ready();
    }
}
