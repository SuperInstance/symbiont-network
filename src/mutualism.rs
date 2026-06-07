//! Mutualism: both agents benefit.

use crate::host::Host;

/// A mutualistic interaction where both parties gain.
pub struct MutualismParams {
    pub benefit_a: f64,
    pub benefit_b: f64,
    pub cost_a: f64,
    pub cost_b: f64,
}

impl Default for MutualismParams {
    fn default() -> Self {
        MutualismParams { benefit_a: 5.0, benefit_b: 5.0, cost_a: 1.0, cost_b: 1.0 }
    }
}

/// Apply mutualistic interaction between two hosts.
pub fn interact(a: &mut Host, b: &mut Host, params: &MutualismParams) -> (f64, f64) {
    let gain_a = params.benefit_a - params.cost_a;
    let gain_b = params.benefit_b - params.cost_b;
    a.add_energy(gain_a);
    b.add_energy(gain_b);
    (gain_a, gain_b)
}

/// Net benefit of mutualism for a host.
pub fn net_benefit(params: &MutualismParams, for_a: bool) -> f64 {
    if for_a { params.benefit_a - params.cost_a } else { params.benefit_b - params.cost_b }
}

/// Check if mutualism is beneficial for both parties.
pub fn is_stable(params: &MutualismParams) -> bool {
    net_benefit(params, true) > 0.0 && net_benefit(params, false) > 0.0
}

/// Run mutualism over N ticks between two hosts.
pub fn run_mutualism(a: &mut Host, b: &mut Host, params: &MutualismParams, ticks: usize) -> (f64, f64) {
    let mut total_a = 0.0;
    let mut total_b = 0.0;
    for _ in 0..ticks {
        if a.is_dead() || b.is_dead() { break; }
        let (ga, gb) = interact(a, b, params);
        total_a += ga;
        total_b += gb;
        a.tick();
        b.tick();
    }
    (total_a, total_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interact() {
        let mut a = Host::new(0, 50.0);
        let mut b = Host::new(1, 50.0);
        let (ga, gb) = interact(&mut a, &mut b, &MutualismParams::default());
        assert!(ga > 0.0);
        assert!(gb > 0.0);
    }

    #[test]
    fn test_net_benefit() {
        let p = MutualismParams::default();
        assert!(net_benefit(&p, true) > 0.0);
    }

    #[test]
    fn test_is_stable_default() {
        assert!(is_stable(&MutualismParams::default()));
    }

    #[test]
    fn test_not_stable() {
        let p = MutualismParams { benefit_a: 0.5, benefit_b: 5.0, cost_a: 1.0, cost_b: 1.0 };
        assert!(!is_stable(&p));
    }

    #[test]
    fn test_run_mutualism() {
        let mut a = Host::new(0, 50.0);
        let mut b = Host::new(1, 50.0);
        let (ta, tb) = run_mutualism(&mut a, &mut b, &MutualismParams::default(), 10);
        assert!(ta > 0.0);
        assert!(tb > 0.0);
    }

    #[test]
    fn test_run_mutualism_death_stops() {
        let mut a = Host::new(0, 3.0); // very low energy
        let mut b = Host::new(1, 3.0);
        let p = MutualismParams { benefit_a: 0.0, benefit_b: 0.0, cost_a: 0.0, cost_b: 0.0 };
        let (ta, _) = run_mutualism(&mut a, &mut b, &p, 100);
        assert!(a.is_dead());
        assert!(ta == 0.0);
    }

    #[test]
    fn test_custom_params() {
        let p = MutualismParams { benefit_a: 10.0, benefit_b: 3.0, cost_a: 0.0, cost_b: 0.0 };
        let mut a = Host::new(0, 50.0);
        let mut b = Host::new(1, 50.0);
        let (ga, gb) = interact(&mut a, &mut b, &p);
        assert!((ga - 10.0).abs() < 1e-10);
        assert!((gb - 3.0).abs() < 1e-10);
    }
}
