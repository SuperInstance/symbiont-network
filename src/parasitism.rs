//! Parasitism: one benefits, one is harmed.

use crate::host::Host;

/// Parasitism parameters.
pub struct ParasitismParams {
    pub drain: f64,
    pub benefit: f64,
    pub defense_reduction: f64,
}

impl Default for ParasitismParams {
    fn default() -> Self {
        ParasitismParams { drain: 5.0, benefit: 3.0, defense_reduction: 0.1 }
    }
}

/// Apply parasitic interaction: parasite drains host.
pub fn parasitize(parasite: &mut Host, host: &mut Host, params: &ParasitismParams) -> (f64, f64) {
    let effective_drain = params.drain * (1.0 - host.defense).max(0.0);
    let taken = host.remove_energy(effective_drain);
    let gained = taken * params.benefit / params.drain.max(0.001);
    parasite.add_energy(gained);
    host.defense = (host.defense - params.defense_reduction).max(0.0);
    (gained, -taken)
}

/// Host defense response: increase defense.
pub fn defend(host: &mut Host, amount: f64) {
    host.defense = (host.defense + amount).min(1.0);
}

/// Check if host should resist (has defense resources).
pub fn should_resist(host: &Host, threshold: f64) -> bool {
    host.defense >= threshold
}

/// Run parasitism for N ticks.
pub fn run_parasitism(parasite: &mut Host, host: &mut Host, params: &ParasitismParams, ticks: usize) -> (f64, f64) {
    let mut total_p = 0.0;
    let mut total_h = 0.0;
    for _ in 0..ticks {
        if parasite.is_dead() || host.is_dead() { break; }
        let (gp, gh) = parasitize(parasite, host, params);
        total_p += gp;
        total_h += gh;
        parasite.tick();
        host.tick();
    }
    (total_p, total_h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parasitize() {
        let mut p = Host::new(0, 50.0);
        let mut h = Host::new(1, 100.0);
        let (gp, gh) = parasitize(&mut p, &mut h, &ParasitismParams::default());
        assert!(gp > 0.0);
        assert!(gh < 0.0);
    }

    #[test]
    fn test_parasitize_with_defense() {
        let mut p = Host::new(0, 50.0);
        let mut h = Host::new(1, 100.0).with_defense(0.5);
        let (_, gh_no_def) = {
            let mut p2 = Host::new(0, 50.0);
            let mut h2 = Host::new(1, 100.0);
            parasitize(&mut p2, &mut h2, &ParasitismParams::default())
        };
        let (_, gh_def) = parasitize(&mut p, &mut h, &ParasitismParams::default());
        // With defense, less should be drained
        assert!(gh_def > gh_no_def); // less negative
    }

    #[test]
    fn test_defend() {
        let mut h = Host::new(0, 100.0);
        defend(&mut h, 0.3);
        assert!((h.defense - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_defend_cap() {
        let mut h = Host::new(0, 100.0).with_defense(0.9);
        defend(&mut h, 0.5);
        assert!((h.defense - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_should_resist() {
        let h = Host::new(0, 100.0).with_defense(0.6);
        assert!(should_resist(&h, 0.5));
        assert!(!should_resist(&h, 0.7));
    }

    #[test]
    fn test_run_parasitism() {
        let mut p = Host::new(0, 50.0);
        let mut h = Host::new(1, 100.0);
        let (tp, th) = run_parasitism(&mut p, &mut h, &ParasitismParams::default(), 10);
        assert!(tp > 0.0);
        assert!(th < 0.0);
    }

    #[test]
    fn test_run_parasitism_host_death() {
        let mut p = Host::new(0, 50.0);
        let mut h = Host::new(1, 5.0);
        run_parasitism(&mut p, &mut h, &ParasitismParams { drain: 10.0, benefit: 5.0, defense_reduction: 0.0 }, 100);
        assert!(h.is_dead());
    }
}
