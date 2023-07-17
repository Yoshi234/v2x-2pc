//TODO
/*
Rebuild the model from the compactCNN (GitHub in rust) for compatibility
with Delphi 

Look into the MiniONN model to see what it's architecture is
*/
use ::neural_network as nn;
use nn::{
    layers::{convolution::Padding, Layer},
    NeuralNetwork,
};
use rand::{CryptoRng, RngCore};

// import all names from parent module
use super::*;

pub fn construct_compact_cnn<R: RngCore + CryptoRng>(
    vs: Option<&tch::nn::Path>,
    batch_size: usize,
    // not utilizing any 
    rng: &mut R,
) -> NeuralNetwork<TenBitAs, TenBitExpFP> {
    // no approximations for the relu_layer
    // there is only one layer in the compact_cnn to approximate
    let relu_layers = match num_poly {
        0 => vec![1],
        1 => vec![],
        _ => unreachable(),
    };
    // network code / definition 
    let mut network = match &vs {
        Some(vs) => NeuralNetwork {
            layers: vec![],
            eval_method: ::neural_network::EvalMethod::TorchDevice(vs.device()),
        },
        None => NeuralNetwork {
            layers: vec![],
            ..Default::default()
        }
    };
    // Dimensions of the input - tuple
    // tuple = (batch_size, num_channels, n_height, n_width)
    // tensor.shape = ([num_samples, 1, 1, 384])
    let input_dims = (batch_size, 1, 1, 384);

    // 1 (lin - CONV LAYER)
    // ([num_samples, 1, 1, 384]) -> ([num_samples, 32, 1, 321])
    // tuple = (num_filters, num_channels, n_ker_height, n_ker_width )
    let kernel_dims = (32, 1, 1, 64);
    let conv = sample_conv_layer(vs, input_dims, kernel_dims, 1, Padding::Valid, rng).0;
    network.layers.push(Layer::LL(conv));

    // 2 (BATCH NORM LAYER)
    // ([num_samples, 32, 1, 321]) -> ([num_samples, 32, 1, 321])
    // TODO: Implement batch norm layer functionality for the neural network struct
    // Not sure how to do this at the moment

    // 3 (nonlin - ReLU ACTIVATION LAYER)
    // ([num_samples, 32, 1, 321]) -> ([num_samples, 32, 1, 321])
    add_activation_layer(&mut network, &relu_layers);

    // 4 (pool - GAP LAYER)
    // ([num_samples, 32, 1, 321]) -> ([num_samples, 32, 1, 1])
    // Do we need to construct a separate GAP function for Delphi as well?
    // the window is 1, and the stride is 384
    let input_dims = network.layers.last().unwrap().output_dimensions();
    let a = sample_avg_pool_layer(input_dims, (1, 1), 384);
    network.layers.push(Layer::LL(pool));

    // 6 (FULLY CONNECTED LAYER)
    let fc_input_dims = network.layers.last().unwrap().output_dimensions();
    let (fc, _) = sample_fc_layer(vs, fc_input_dims, 2, rng);  
    network.layers.push(Layer::LL(fc));
    assert!(network.validate());

    // 7 (nonlin - SOFTMAX LAYER)
    // need to implement the softmax layer to complete the network architecture
    // will need to build a garbled circuit for this function!!!
    network
}