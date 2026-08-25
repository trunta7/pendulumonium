#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendulumParams {
    pub mass1: f64,
    pub mass2: f64,
    pub length1: f64,
    pub length2: f64,
    pub gravity: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct State {
    pub theta1: f64,
    pub theta2: f64,
    pub omega1: f64,
    pub omega2: f64,
}

impl State {
    pub fn mul_scalar(&self, scalar: f64) -> Self {
        Self {
            theta1: self.theta1 * scalar,
            theta2: self.theta2 * scalar,
            omega1: self.omega1 * scalar,
            omega2: self.omega2 * scalar,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            theta1: self.theta1 + other.theta1,
            theta2: self.theta2 + other.theta2,
            omega1: self.omega1 + other.omega1,
            omega2: self.omega2 + other.omega2,
        }
    }
}

/// calculates the derivatives (dθ/dt, dω/dt) for the current state.
pub fn evaluate_derivatives(state: &State, params: &PendulumParams) -> State {
    let delta = state.theta1 - state.theta2;
    let den1 = (2.0 * params.mass1 + params.mass2) - params.mass2 * (2.0 * delta).cos();
    
    let num1 = -params.gravity * (2.0 * params.mass1 + params.mass2) * state.theta1.sin()
        - params.mass2 * params.gravity * (state.theta1 - 2.0 * state.theta2).sin()
        - 2.0 * delta.sin() * params.mass2 * (state.omega2.powi(2) * params.length2 + state.omega1.powi(2) * params.length1 * delta.cos());
    
    let num2 = 2.0 * delta.sin() * (
        state.omega1.powi(2) * params.length1 * (params.mass1 + params.mass2)
        + params.gravity * (params.mass1 + params.mass2) * state.theta1.cos()
        + state.omega2.powi(2) * params.length2 * params.mass2 * delta.cos()
    );

    let d_omega1 = num1 / (params.length1 * den1);
    let d_omega2 = num2 / (params.length2 * den1);

    State {
        theta1: state.omega1, // d(theta1)/dt = omega1
        theta2: state.omega2, // d(theta2)/dt = omega2
        omega1: d_omega1,
        omega2: d_omega2,
    }
}


/// performs a single RK8 step. 
pub fn rk8_step(state: &State, params: &PendulumParams, dt: f64) -> (State, [State; 12]) {
    // DOP853 Runge-Kutta Matrix (a) constants
    const A21: f64 = 5.26001519587677318785587544488e-2;
    const A31: f64 = 1.97250569845378994544595329183e-2;
    const A32: f64 = 5.91751709536136983633785987549e-2;
    const A41: f64 = 2.95875854768068491816892993775e-2;
    const A43: f64 = 8.87627564304205475450678981324e-2;
    const A51: f64 = 2.41365134159266685502369798665e-1;
    const A53: f64 = -8.84549479328286085344864962717e-1;
    const A54: f64 = 9.24834003261792003115737966543e-1;
    const A61: f64 = 3.7037037037037037037037037037e-2;
    const A64: f64 = 1.70828608729473871279604482173e-1;
    const A65: f64 = 1.25467687566822425016691814123e-1;
    const A71: f64 = 3.7109375e-2;
    const A74: f64 = 1.70252211019544039314978060272e-1;
    const A75: f64 = 6.02165389804559606850219397283e-2;
    const A76: f64 = -1.7578125e-2;
    const A81: f64 = 3.70920001185047927108779319836e-2;
    const A84: f64 = 1.70383925712239993810214054705e-1;
    const A85: f64 = 1.07262030446373284651809199168e-1;
    const A86: f64 = -1.53194377486244017527936158236e-2;
    const A87: f64 = 8.27378916381402288758473766002e-3;
    const A91: f64 = 6.24110958716075717114429577812e-1;
    const A94: f64 = -3.36089262944694129406857109825;
    const A95: f64 = -8.68219346841726006818189891453e-1;
    const A96: f64 = 2.75920996994467083049415600797e+1;
    const A97: f64 = 2.01540675504778934086186788979e+1;
    const A98: f64 = -4.34898841810699588477366255144e+1;
    const A101: f64 = 4.77662536438264365890433908527e-1;
    const A104: f64 = -2.48811461997166764192642586468;
    const A105: f64 = -5.90290826836842996371446475743e-1;
    const A106: f64 = 2.12300514481811942347288949897e+1;
    const A107: f64 = 1.52792336328824235832596922938e+1;
    const A108: f64 = -3.32882109689848629194453265587e+1;
    const A109: f64 = -2.03312017085086261358222928593e-2;
    const A111: f64 = -9.3714243008598732571704021658e-1;
    const A114: f64 = 5.18637242884406370830023853209;
    const A115: f64 = 1.09143734899672957818500254654;
    const A116: f64 = -8.14978701074692612513997267357;
    const A117: f64 = -1.85200656599969598641566180701e+1;
    const A118: f64 = 2.27394870993505042818970056734e+1;
    const A119: f64 = 2.49360555267965238987089396762;
    const A1110: f64 = -3.0467644718982195003823669022;
    const A121: f64 = 2.27331014751653820792359768449;
    const A124: f64 = -1.05344954667372501984066689879e+1;
    const A125: f64 = -2.00087205822486249909675718444;
    const A126: f64 = -1.79589318631187989172765950534e+1;
    const A127: f64 = 2.79488845294199600508499808837e+1;
    const A128: f64 = -2.85899827713502369474065508674;
    const A129: f64 = -8.87285693353062954433549289258;
    const A1210: f64 = 1.23605671757943030647266201528e+1;
    const A1211: f64 = 6.43392746015763530355970484046e-1;

    // Weights (b) constants
    const B1: f64 = 5.42937341165687622380535766363e-2;
    const B6: f64 = 4.45031289275240888144113950566;
    const B7: f64 = 1.89151789931450038304281599044;
    const B8: f64 = -5.8012039600105847814672114227;
    const B9: f64 = 3.1116436695781989440891606237e-1;
    const B10: f64 = -1.52160949662516078556178806805e-1;
    const B11: f64 = 2.01365400804030348374776537501e-1;
    const B12: f64 = 4.47106157277725905176885569043e-2;

    macro_rules! compute_stage {
        ( $( $k:ident => $a:ident ),+ ) => {
            {
                let mut s = *state;
                $( s = s.add(&$k.mul_scalar(dt * $a)); )+
                evaluate_derivatives(&s, params)
            }
        };
    }

    let k1 = evaluate_derivatives(state, params);
    let k2 = compute_stage!(k1 => A21);
    let k3 = compute_stage!(k1 => A31, k2 => A32);
    let k4 = compute_stage!(k1 => A41, k3 => A43);
    let k5 = compute_stage!(k1 => A51, k3 => A53, k4 => A54);
    let k6 = compute_stage!(k1 => A61, k4 => A64, k5 => A65);
    let k7 = compute_stage!(k1 => A71, k4 => A74, k5 => A75, k6 => A76);
    let k8 = compute_stage!(k1 => A81, k4 => A84, k5 => A85, k6 => A86, k7 => A87);
    let k9 = compute_stage!(k1 => A91, k4 => A94, k5 => A95, k6 => A96, k7 => A97, k8 => A98);
    let k10 = compute_stage!(k1 => A101, k4 => A104, k5 => A105, k6 => A106, k7 => A107, k8 => A108, k9 => A109);
    let k11 = compute_stage!(k1 => A111, k4 => A114, k5 => A115, k6 => A116, k7 => A117, k8 => A118, k9 => A119, k10 => A1110);
    let k12 = compute_stage!(k1 => A121, k4 => A124, k5 => A125, k6 => A126, k7 => A127, k8 => A128, k9 => A129, k10 => A1210, k11 => A1211);

    let mut next_state = *state;
    next_state = next_state.add(&k1.mul_scalar(dt * B1));
    next_state = next_state.add(&k6.mul_scalar(dt * B6));
    next_state = next_state.add(&k7.mul_scalar(dt * B7));
    next_state = next_state.add(&k8.mul_scalar(dt * B8));
    next_state = next_state.add(&k9.mul_scalar(dt * B9));
    next_state = next_state.add(&k10.mul_scalar(dt * B10));
    next_state = next_state.add(&k11.mul_scalar(dt * B11));
    next_state = next_state.add(&k12.mul_scalar(dt * B12));

    let k_values = [k1, k2, k3, k4, k5, k6, k7, k8, k9, k10, k11, k12];

    (next_state, k_values)
}