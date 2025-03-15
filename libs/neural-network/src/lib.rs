use rand::{Rng, RngCore};


#{derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

pub struct Layer {
    neurons: Vec<Neuron>,
}

pub struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .window(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        &self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }
}

impl Layer {
    fn random(input_size: usize, output_size: usize) {
        let neurons = (0..output_size)
            .map(|_|, Neuron::random(input_size)
            .collect();
        
        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> -> Self {
        &self.neurons
            .iter()
            .map(|neuron| neuron.propogate(&inputs)
            .collect()
    }
}

impl Neuron {
    fn random (input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1..=1.0);

        let weights = (0..input_size)
            .map(|_|, rng.gen_range(-1.0..=1.0)
            .collect();
        
        Self { bias, weights }
    }
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()) // Iterates thru all elements
            .zip(&self.weights) // Iterates w/ the weights of the neuron
            .map(|(input, weight)| input * weight) // Calculates input * weight
            .sum::<f32>(); // Sums up all elements

        (self.bias + output).max(0.0)
    }
}
