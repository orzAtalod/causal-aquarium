/// Ornstein-Uhlenbeck (OU) process network implementation in Rust
/// mathematical provement for this part could be found in `doc/OU_process.md` (written in Chinese)
/// almost everything is public for simplicity

use std::default;
use rand::Rng;
use rand_distr::{Normal, Distribution};

/// OU process variable struct
/// val: [f64; 3] - the value of the variable
/// varies from [0, 100] for each variant
#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub val: [f64; 3], //3-dimensional variable
}

/// default value is [50.0, 50.0, 50.0]
impl default::Default for Var {
    fn default() -> Self {
        Var { val: [50.0, 50.0, 50.0] }
    }
}

/// OU process configuration struct
/// theta: f64 - the speed of mean reversion
/// sigma: f64 - the variance (sd^2)
/// delta_t: f64 - the time step
/// gamma: [[f64; 3]; 3] - the interaction matrix between the 3 variants
/// the elements in the main diagonal of gamma should be 1.0
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub theta: f64,
    pub sigma: f64,
    pub delta_t: f64,
    pub gamma: [[f64; 3]; 3], //3x3 matrix, the element in main diag should be 1.0
}

/// default value is:
/// theta: 0.7,
/// sigma: 0.3,
/// delta_t: 1.0,
/// gamma: [
///     [1.0, 0.2, 0.1],
///     [0.2, 1.0, 0.3],
///     [0.1, 0.3, 1.0],
/// ],
impl default::Default for Config {
    fn default() -> Self {
        Config {
            theta: 0.7,
            sigma: 0.3,
            delta_t: 1.0,
            gamma: [
                [1.0, 0.2, 0.1],
                [0.2, 1.0, 0.3],
                [0.1, 0.3, 1.0],
            ],
        }
    }
}

/// OU process interventions struct
/// index: Option<usize> - the index of the variant to be intervened (0, 1, or 2), None means no intervention
/// value: f64 - the value to set the variant to ([0, 100]), ignored when `index.isNone()`
#[derive(Debug, Clone, PartialEq)]
pub struct Intervention {
    pub index: Option<usize>,
    pub value: f64,
}

/// default value is index=None (no intervention), value=0.0
impl default::Default for Intervention {
    fn default() -> Self {
        Intervention { index: None, value: 0.0 }
    }
}

/// OU process state struct
/// config: Config - the configuration of the OU process
/// var: Var - the variable of the OU process
/// use function `update_ou_state` to update the state
#[derive(Debug, Clone)]
pub struct OUState {
    pub config: Config,
    pub var: Var,
    pub intervention: Intervention,
}

/// default value is Config::default() and Var::default()
impl default::Default for OUState {
    fn default() -> Self {
        OUState {
            config: Config::default(),
            var: Var::default(),
            intervention: Intervention::default(),
        }
    }
}

/// calculate the L2 norm of a 3-dimensional vector
/// requires each dimension is in range [0, 100]
/// # Arguments
/// * `v` - a 3-dimensional vector
/// # Returns
/// * `f64` - the L2 norm of the vector
fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// OU process state update function
/// # Arguments
/// * `os` - current OU process state
/// # Returns
/// * `OUState` - updated OU process state
/// # Panics
/// this function assumes that the input is valid, i.e., the elements in `os.var.index` are in range [0, 3)
/// if note so, the function may panic
pub fn update_ou_state(os: &OUState) -> OUState {
    //calculate mu
    let beta = - norm(os.var.val) / 100.0;
    let gamma = os.config.gamma;
    let old = os.var.val;
    let mu: [f64; 3] = [
        beta*old[0] + gamma[0][0]*old[0] + gamma[0][1]*old[1] + gamma[0][2]*old[2],
        beta*old[1] + gamma[1][0]*old[0] + gamma[1][1]*old[1] + gamma[1][2]*old[2],
        beta*old[2] + gamma[2][0]*old[0] + gamma[2][1]*old[1] + gamma[2][2]*old[2],
    ];

    //update var
    let mut rng = rand::thread_rng();
    let theta = os.config.theta;
    let sigma = os.config.sigma;
    let delta_t = os.config.delta_t;
    let e_theta_dt = (-theta * delta_t).exp();

    //NOTE: sigma here represents variance (sd²), not standard deviation.
    let stddev = (sigma*(1.0-e_theta_dt*e_theta_dt)/2.0/theta).sqrt();
    let normal = Normal::new(0.0, stddev).unwrap();

    let mut new_var = Var {   //mutable to be invtervented
        val: [
            mu[0] + (old[0] - mu[0]) * e_theta_dt + normal.sample(&mut rng),
            mu[1] + (old[1] - mu[1]) * e_theta_dt + normal.sample(&mut rng),
            mu[2] + (old[2] - mu[2]) * e_theta_dt + normal.sample(&mut rng)
        ],
    };

    if let Some(idx) = os.intervention.index {
        new_var.val[idx] = os.intervention.value; //didn't check boundary here, assume the input is valid
    }
    
    //hard boundary fix in case the noise term makes it out of range
    for i in 0..3 {
        if new_var.val[i] < 0.0 {
            new_var.val[i] = 0.0;
        } else if new_var.val[i] > 100.0 {
            new_var.val[i] = 100.0;
        }
    }

    OUState {
        var: new_var,
        ..os.clone()
    }
}