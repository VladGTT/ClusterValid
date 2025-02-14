use crate::sender::Sender;
use ndarray::ArrayView2;
#[derive(Debug, Clone)]
pub struct RawDataValue<'a> {
    pub x: ArrayView2<'a, f64>,
    pub y: ArrayView2<'a, usize>,
}
pub struct RawDataNode<'a> {
    pub sender: Sender<'a, RawDataValue<'a>>,
}

impl<'a> RawDataNode<'a> {
    pub fn new(sender: Sender<'a, RawDataValue<'a>>) -> Self {
        Self { sender }
    }
    pub fn compute(&self, data: RawDataValue<'a>) {
        self.sender.send_to_subscribers(Ok(data));
    }
}
impl<'a> From<(ArrayView2<'a, f64>, ArrayView2<'a, usize>)> for RawDataValue<'a> {
    fn from(value: (ArrayView2<'a, f64>, ArrayView2<'a, usize>)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
