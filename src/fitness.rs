//! Population fitness tracking.

use crate::host::Host;

/// Fitness metrics for a population.
#[derive(Debug, Clone)]
pub struct FitnessMetrics {
    pub avg_fitness: f64,
    pub max_fitness: f64,
    pub min_fitness: f64,
    pub total_fitness: f64,
    pub population_size: usize,
    pub alive_count: usize,
}

impl FitnessMetrics {
    pub fn from_hosts(hosts: &[Host]) -> Self {
        let alive: Vec<&Host> = hosts.iter().filter(|h| h.alive).collect();
        if alive.is_empty() {
            return FitnessMetrics { avg_fitness: 0.0, max_fitness: 0.0, min_fitness: 0.0, total_fitness: 0.0, population_size: hosts.len(), alive_count: 0 };
        }
        let fitnesses: Vec<f64> = alive.iter().map(|h| h.effective_fitness()).collect();
        let total: f64 = fitnesses.iter().sum();
        FitnessMetrics {
            avg_fitness: total / fitnesses.len() as f64,
            max_fitness: fitnesses.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            min_fitness: fitnesses.iter().cloned().fold(f64::INFINITY, f64::min),
            total_fitness: total,
            population_size: hosts.len(),
            alive_count: alive.len(),
        }
    }

    pub fn diversity(&self) -> f64 {
        if self.alive_count == 0 { return 0.0; }
        let avg = self.avg_fitness;
        (self.max_fitness - self.min_fitness) / (avg + 1.0)
    }
}

/// Track fitness over time.
pub struct FitnessTracker {
    pub history: Vec<FitnessMetrics>,
}

impl Default for FitnessTracker {
    fn default() -> Self { Self::new() }
}

impl FitnessTracker {
    pub fn new() -> Self { FitnessTracker { history: Vec::new() } }

    pub fn record(&mut self, hosts: &[Host]) {
        self.history.push(FitnessMetrics::from_hosts(hosts));
    }

    pub fn latest(&self) -> Option<&FitnessMetrics> {
        self.history.last()
    }

    pub fn trend(&self) -> f64 {
        if self.history.len() < 2 { return 0.0; }
        let first = self.history.first().unwrap().avg_fitness;
        let last = self.history.last().unwrap().avg_fitness;
        last - first
    }

    pub fn is_improving(&self) -> bool {
        self.trend() > 0.0
    }

    pub fn generations(&self) -> usize {
        self.history.len()
    }

    pub fn best_ever(&self) -> Option<f64> {
        self.history.iter().map(|m| m.max_fitness).fold(None, |acc, f| {
            match acc { None => Some(f), Some(v) => Some(v.max(f)) }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_hosts(energies: &[f64]) -> Vec<Host> {
        energies.iter().enumerate().map(|(i, &e)| Host::new(i, e)).collect()
    }

    #[test]
    fn test_metrics_from_hosts() {
        let hosts = make_hosts(&[50.0, 100.0, 75.0]);
        let m = FitnessMetrics::from_hosts(&hosts);
        assert_eq!(m.alive_count, 3);
        assert!(m.avg_fitness > 0.0);
    }

    #[test]
    fn test_metrics_empty() {
        let m = FitnessMetrics::from_hosts(&[]);
        assert_eq!(m.alive_count, 0);
        assert_eq!(m.avg_fitness, 0.0);
    }

    #[test]
    fn test_metrics_with_dead() {
        let mut hosts = make_hosts(&[50.0, 100.0]);
        hosts[1].alive = false;
        let m = FitnessMetrics::from_hosts(&hosts);
        assert_eq!(m.alive_count, 1);
        assert_eq!(m.population_size, 2);
    }

    #[test]
    fn test_metrics_max_min() {
        let hosts = make_hosts(&[10.0, 100.0, 50.0]);
        let m = FitnessMetrics::from_hosts(&hosts);
        assert!(m.max_fitness > m.min_fitness);
    }

    #[test]
    fn test_diversity() {
        let hosts = make_hosts(&[10.0, 100.0]);
        let m = FitnessMetrics::from_hosts(&hosts);
        assert!(m.diversity() >= 0.0);
    }

    #[test]
    fn test_tracker_record() {
        let mut tracker = FitnessTracker::new();
        let hosts = make_hosts(&[50.0]);
        tracker.record(&hosts);
        assert_eq!(tracker.generations(), 1);
    }

    #[test]
    fn test_tracker_latest() {
        let mut tracker = FitnessTracker::new();
        assert!(tracker.latest().is_none());
        tracker.record(&make_hosts(&[50.0]));
        assert!(tracker.latest().is_some());
    }

    #[test]
    fn test_tracker_trend() {
        let mut tracker = FitnessTracker::new();
        tracker.record(&make_hosts(&[50.0]));
        tracker.record(&make_hosts(&[100.0]));
        assert!(tracker.trend() > 0.0);
    }

    #[test]
    fn test_tracker_is_improving() {
        let mut tracker = FitnessTracker::new();
        tracker.record(&make_hosts(&[50.0]));
        tracker.record(&make_hosts(&[100.0]));
        assert!(tracker.is_improving());
    }

    #[test]
    fn test_tracker_best_ever() {
        let mut tracker = FitnessTracker::new();
        tracker.record(&make_hosts(&[50.0]));
        tracker.record(&make_hosts(&[30.0]));
        assert!(tracker.best_ever().unwrap() > 0.0);
    }

    #[test]
    fn test_tracker_trend_single() {
        let mut tracker = FitnessTracker::new();
        tracker.record(&make_hosts(&[50.0]));
        assert!((tracker.trend() - 0.0).abs() < 1e-10);
    }
}
