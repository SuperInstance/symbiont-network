//! Commensalism: one benefits, one is neutral.

use crate::host::Host;

/// Commensalism parameters.
pub struct CommensalismParams {
    pub benefit: f64,
    pub cost: f64, // usually zero or negligible
}

impl Default for CommensalismParams {
    fn default() -> Self {
        CommensalismParams { benefit: 3.0, cost: 0.0 }
    }
}

/// Apply commensal interaction: one benefits, host unaffected.
pub fn commensal_interact(beneficiary: &mut Host, host: &mut Host, params: &CommensalismParams) -> (f64, f64) {
    beneficiary.add_energy(params.benefit);
    host.add_energy(-params.cost);
    (params.benefit, -params.cost)
}

/// Pure commensalism: cost is zero.
pub fn is_pure_commensalism(params: &CommensalismParams) -> bool {
    params.cost == 0.0
}

/// Run commensalism for N ticks.
pub fn run_commensalism(beneficiary: &mut Host, host: &mut Host, params: &CommensalismParams, ticks: usize) -> (f64, f64) {
    let mut total_b = 0.0;
    let mut total_h = 0.0;
    for _ in 0..ticks {
        if beneficiary.is_dead() || host.is_dead() { break; }
        let (gb, gh) = commensal_interact(beneficiary, host, params);
        total_b += gb;
        total_h += gh;
        beneficiary.tick();
        host.tick();
    }
    (total_b, total_h)
}

/// Determine relationship type based on net effects.
#[derive(Debug, PartialEq)]
pub enum RelationshipType {
    Mutualism,
    Parasitism,
    Commensalism,
    Amensalism,
    Neutralism,
}

pub fn classify_relationship(benefit_a: f64, benefit_b: f64) -> RelationshipType {
    match (benefit_a > 0.0, benefit_a < 0.0, benefit_b > 0.0, benefit_b < 0.0) {
        (true, _, true, _) => RelationshipType::Mutualism,
        (true, _, false, true) => RelationshipType::Parasitism,
        (false, true, true, _) => RelationshipType::Parasitism,
        (true, _, false, false) => RelationshipType::Commensalism,
        (false, false, true, _) => RelationshipType::Commensalism,
        (false, true, false, true) => RelationshipType::Amensalism,
        _ => RelationshipType::Neutralism,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commensal_interact() {
        let mut b = Host::new(0, 50.0);
        let mut h = Host::new(1, 100.0);
        let (gb, gh) = commensal_interact(&mut b, &mut h, &CommensalismParams::default());
        assert!((gb - 3.0).abs() < 1e-10);
        assert!((gh - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_pure() {
        assert!(is_pure_commensalism(&CommensalismParams::default()));
        assert!(!is_pure_commensalism(&CommensalismParams { benefit: 1.0, cost: 0.1 }));
    }

    #[test]
    fn test_run_commensalism() {
        let mut b = Host::new(0, 50.0);
        let mut h = Host::new(1, 100.0);
        let (tb, th) = run_commensalism(&mut b, &mut h, &CommensalismParams::default(), 10);
        assert!(tb > 0.0);
        assert!((th - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_classify_mutualism() {
        assert_eq!(classify_relationship(1.0, 1.0), RelationshipType::Mutualism);
    }

    #[test]
    fn test_classify_parasitism() {
        assert_eq!(classify_relationship(1.0, -1.0), RelationshipType::Parasitism);
    }

    #[test]
    fn test_classify_commensalism() {
        assert_eq!(classify_relationship(1.0, 0.0), RelationshipType::Commensalism);
    }

    #[test]
    fn test_classify_amensalism() {
        assert_eq!(classify_relationship(-1.0, -1.0), RelationshipType::Amensalism);
    }

    #[test]
    fn test_classify_neutralism() {
        assert_eq!(classify_relationship(0.0, 0.0), RelationshipType::Neutralism);
    }
}
