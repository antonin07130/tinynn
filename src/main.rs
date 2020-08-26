use ndarray::Array2;
use ndarray::arr2;
use ndarray::Array;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

fn main() {
    println!("Hello, world!");
    // Fixed-size array (type signature is superfluous)

    let rn = init();
    println!("{:?}", rn);
    println!("{:?}", rn);

}

fn activation(x: f64) -> f64 {
    if x < 0. {
        return 0.;
    }
    return x;
}

fn activation_deriv(x: f64) -> f64 {
    if x < 0. {
        return 0.;
    }
    return 1.;
}

#[derive(Debug)]
struct NeuralNetwork {
    x: Array2<f64>, // input
    //                    [x1,
    //                     x2,
    //                     x3]
    y: f64,  // output
    w1: Array2<f64>, // weights 1
    //                 [w11, w12, w13]
    //                 [w21, w22, w23]
    //                 [w31, w32, w33]
    //                 [w41, w42, w43]
    b1: f64, // bias 1
    w2: Array2<f64>, // weights 2
    //                 [w11, w12, w13, w14]
    b2: f64, // bias 2
}

 fn init() -> NeuralNetwork {
     return NeuralNetwork{
         x : arr2(&[[1.],
             [1.],
             [0.]]),
         w1 : Array2::random((4, 3), Uniform::new(0., 10.)),
         b1 : 1.,
         w2 : Array2::random((1, 4), Uniform::new(0., 10.)),
         b2 : 2.,
         y : 0.
     }; }

 fn feed_forward(n : NeuralNetwork) -> NeuralNetwork {
     let mut layer1 : Array2<f64> = n.w1.dot(x);
     layer1 = layer1.map(|el| -> f64 {return activation(*el)});
     let mut layer2 : Array2<_> = layer1.dot(n.w2);


 }

