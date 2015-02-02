extern crate core;

use core::num::FromPrimitive;
use core::fmt;

use std::rand::random;

use Direction::{SpinUp, SpinDown, SpinSuper};
use Plan::{Trivial, OddBall};

#[derive(Copy)]
#[derive(Show)]
#[derive(PartialEq)]
enum Direction {
    SpinUp,
    SpinDown,
    SpinSuper,
}

impl fmt::String for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = match self {
            &SpinUp => "SpinUp",
            &SpinDown => "SpinDown",
            &SpinSuper => "SpinSuper"
        };
        write!(f, "{}", r)
    }
}

impl fmt::String for Detector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Detector::{D12, D3, D9};
        let r = match self {
            &D12 => "12 oclock",
            &D3 => "3 oclock",
            &D9 => "9 oclock"
        };
        write!(f, "{}", r)
    }
}

struct Pair<T> {
    lhs: T,
    rhs: T
}

struct Particle {
    spin: Direction
}

enum Plan {
    Trivial, // up-up-up -> down-down-down
    OddBall  // up-down-up -> down-up-down
}

#[derive(Show)]
enum Detector {
    D12, // 12 o'clock
    D3, // 3 o'clock
    D9, // 9 o' clock
}

impl Particle {
    fn new_pair () -> Pair<Particle> {
        let d1 = SpinSuper;
        let d2 = SpinSuper;
        let p1 = Particle{spin: d1};
        let p2 = Particle{spin: d2};

        return Pair{lhs: p1, rhs: p2};
    }

    pub fn measure (&mut self) {
        // TODO theta = 60degrees use 3/4th and 1/4th

        let detector = Particle::get_detector_direction();
        let rnd = random::<f32>();

        self.spin = match detector {
            Detector::D12 => SpinDown,
            Detector::D3 => match rnd {
                0.0  ... 0.33 => SpinUp,
                0.33 ... 1.00 => SpinDown,
                _ => unreachable!()
            },
            Detector::D9 => match rnd {
                0.0  ... 0.33 => SpinDown,
                0.33 ... 1.00 => SpinUp,
                _ => unreachable!()
            },
        };
    }

    // NOTE on spooky and premeditated only if measured in the same
    // direction must the spins be opposite

    // measure with with a message
    pub fn spooky (&mut self, friend: &mut Particle) -> Pair<Direction> {

        self.measure();
        friend.spin = match self.spin {
            SpinUp => SpinDown,
            SpinDown => SpinUp,
            _ => panic!("broke the universe")
        };
        return Pair{lhs: self.spin, rhs: friend.spin};
    }


    fn get_detector_direction () -> Detector {
        let rnd = random::<f32>();

        let detector = match rnd {
            0.0  ... 0.33 => Detector::D12,
            0.33 ... 0.66 => Detector::D3,
            0.66 ... 1.00 => Detector::D9,
            _ => unreachable!()
        };

        return detector;
    }

    pub fn hidden_information (&mut self, friend: &mut Particle, plan: Plan) -> Pair<Direction> {

        let rnd = random::<f32>();

        let detector = Particle::get_detector_direction();        
        let spin = match plan {
            Trivial => SpinUp,
            OddBall => match detector {
                Detector::D3 => SpinDown,
                _ => SpinUp
            }
        };
        friend.spin = spin;

        let detector = Particle::get_detector_direction();
        let spin = match plan {
            Trivial => SpinDown,
            OddBall => match detector {
                Detector::D3 => SpinUp,
                _ => SpinDown
            }
        };
        self.spin = spin;

        return Pair{lhs: self.spin, rhs: friend.spin};
    }
}


// hidden_information should give +55.6% difference, spooky would give 50%
fn main () {


    let mut particle = Particle{spin: SpinSuper};
    particle.measure();
    println!("particle {}", particle.spin);


    let mut trials: f64 = 1000f64;
    let mut num_different: f64 = 0f64;

    let particles = Particle::new_pair();
    let mut lhs = particles.lhs;
    let mut rhs = particles.rhs;
    
    for _ in range(0, trials as usize) {   
        let particles = Particle::new_pair();
        lhs = particles.lhs;
        rhs = particles.rhs;

        lhs.spooky(&mut rhs);

        if lhs.spin != rhs.spin {
            num_different += 1.0;
        }
    }

    println!("lhs.spin {}, rhs.spin {} num_different {}", lhs.spin, rhs.spin, num_different);

    println!("num_different {}%", 100f64 * (trials - num_different) / trials as f64);


    trials = 1000f64;
    num_different = 0f64;

    for _ in range(0, trials as usize) {
        let particles = Particle::new_pair();
        lhs = particles.lhs;
        rhs = particles.rhs;
        lhs.hidden_information(&mut rhs, OddBall);
        
        //println!("lhs.spin {}, rhs.spin {}", lhs.spin, rhs.spin);
        if lhs.spin != rhs.spin {
            num_different += 1.0;
        }
    }

    println!("Percent the same {}%", 100f64 * (trials - num_different) / trials as f64);


}
