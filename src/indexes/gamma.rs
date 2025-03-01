use std::iter::zip;

use crate::calc_error::CalcError;
use crate::sender::{Sender, Subscriber};
use std::sync::Arc;

use super::helpers::s_plus_and_minus::SPlusAndMinusValue;
#[derive(Clone, Debug)]
pub struct GammaIndexValue {
    pub val: Arc<Vec<f64>>,
}

#[derive(Default)]
pub struct Index;

impl Index {
    fn compute(&self, s_plus: &Vec<isize>, s_minus: &Vec<isize>) -> Result<Vec<f64>, CalcError> {
        zip(s_plus, s_minus)
            .map(|(s_plus, s_minus)| self.helper(*s_plus, *s_minus))
            .collect()
    }
    fn helper(&self, s_plus: isize, s_minus: isize) -> Result<f64, CalcError> {
        let value = (s_plus - s_minus) as f64 / (s_plus + s_minus) as f64;
        Ok(value)
    }
}
#[derive(Default)]
pub struct Node<'a> {
    index: Index,
    sender: Sender<'a, GammaIndexValue>,
}

impl<'a> Node<'a> {
    pub fn new(sender: Sender<'a, GammaIndexValue>) -> Self {
        Self {
            index: Index,
            sender,
        }
    }
}
impl<'a> Subscriber<SPlusAndMinusValue> for Node<'a> {
    fn recieve_data(&mut self, data: Result<SPlusAndMinusValue, CalcError>) {
        let res = match data.as_ref() {
            Ok(spm) => self
                .index
                .compute(&spm.s_plus, &spm.s_minus)
                .map(|val| GammaIndexValue { val: Arc::new(val) }),
            Err(err) => Err(err.clone()),
        };
        self.sender.send_to_subscribers(res);
    }
}
