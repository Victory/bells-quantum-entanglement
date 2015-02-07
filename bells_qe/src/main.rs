extern crate bells_qe;

use bells_qe::{run_spooky, run_hidden};

// hidden_information should give +55.6% difference, spooky would give 50%
fn main () {

    let trials: f64 = 1000000f64;

    run_spooky(trials);
    run_hidden(trials, 0.5);
    run_hidden(trials, 1.0);
    run_hidden(trials, 0.0);
}
