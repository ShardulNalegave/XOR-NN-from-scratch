use rand::{thread_rng, Rng};


pub struct Network {
  n_inputs: usize,
  n_hidden: usize,
  n_outputs: usize,
  learning_rate: f64,
  weights_h: na::DMatrix<f64>,
  weights_o: na::DMatrix<f64>,
  bias_h: na::DVector<f64>,
  bias_o: na::DVector<f64>,
}

impl Network {
  pub fn new(n_inputs: usize, n_hidden: usize, n_outputs: usize) -> Self {
    Self {
      n_inputs,
      n_hidden,
      n_outputs,
      learning_rate: 0.1,
      weights_h: na::DMatrix::from_fn(n_inputs, n_hidden, |_, _| thread_rng().gen()),
      weights_o: na::DMatrix::from_fn(n_hidden, n_outputs, |_, _| thread_rng().gen()),
      bias_h: na::DVector::from_element(n_inputs, 1.0),
      bias_o: na::DVector::from_element(n_hidden, 1.0),
    }
  }

  pub fn predict(&self, data: na::DVector<f64>) -> f64 {
    unimplemented!()
  }

  pub fn train(&mut self, inp: na::DVector<f64>, output: na::DVector<f64>) {
    unimplemented!()
  }
}