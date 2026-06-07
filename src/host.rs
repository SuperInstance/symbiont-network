//! Host (recipient) agent.

/// A host agent that can participate in symbiotic relationships.
#[derive(Clone, Debug)]
pub struct Host {
    pub id: usize,
    pub energy: f64,
    pub defense: f64,
    pub resources: f64,
    pub fitness: f64,
    pub alive: bool,
    pub symbiont_count: usize,
}

impl Host {
    pub fn new(id: usize, energy: f64) -> Self {
        Host { id, energy, defense: 0.0, resources: 0.0, fitness: 1.0, alive: true, symbiont_count: 0 }
    }

    pub fn with_defense(mut self, d: f64) -> Self { self.defense = d; self }
    pub fn with_resources(mut self, r: f64) -> Self { self.resources = r; self }

    pub fn add_energy(&mut self, amount: f64) {
        self.energy += amount;
    }

    pub fn remove_energy(&mut self, amount: f64) -> f64 {
        let taken = amount.min(self.energy);
        self.energy -= taken;
        taken
    }

    pub fn add_symbiont(&mut self) {
        self.symbiont_count += 1;
    }

    pub fn remove_symbiont(&mut self) {
        self.symbiont_count = self.symbiont_count.saturating_sub(1);
    }

    pub fn has_symbionts(&self) -> bool {
        self.symbiont_count > 0
    }

    pub fn tick(&mut self) {
        if !self.alive { return; }
        self.energy -= 1.0; // base metabolism
        if self.energy <= 0.0 {
            self.alive = false;
            self.energy = 0.0;
        }
    }

    pub fn is_alive(&self) -> bool { self.alive }
    pub fn is_dead(&self) -> bool { !self.alive }

    pub fn effective_fitness(&self) -> f64 {
        if !self.alive { return 0.0; }
        self.fitness * (1.0 + self.energy / 100.0)
    }
}

/// A collection of hosts.
pub struct HostPopulation {
    pub hosts: Vec<Host>,
    next_id: usize,
}

impl Default for HostPopulation {
    fn default() -> Self { Self::new() }
}

impl HostPopulation {
    pub fn new() -> Self { HostPopulation { hosts: Vec::new(), next_id: 0 } }

    pub fn add(&mut self, energy: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.hosts.push(Host::new(id, energy));
        id
    }

    pub fn alive_count(&self) -> usize {
        self.hosts.iter().filter(|h| h.alive).count()
    }

    pub fn dead_count(&self) -> usize {
        self.hosts.iter().filter(|h| !h.alive).count()
    }

    pub fn avg_energy(&self) -> f64 {
        if self.hosts.is_empty() { return 0.0; }
        self.hosts.iter().map(|h| h.energy).sum::<f64>() / self.hosts.len() as f64
    }

    pub fn tick_all(&mut self) {
        for h in &mut self.hosts { h.tick(); }
    }

    pub fn remove_dead(&mut self) -> usize {
        let before = self.hosts.len();
        self.hosts.retain(|h| h.alive);
        before - self.hosts.len()
    }

    pub fn get(&self, id: usize) -> Option<&Host> {
        self.hosts.iter().find(|h| h.id == id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Host> {
        self.hosts.iter_mut().find(|h| h.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_new() {
        let h = Host::new(0, 100.0);
        assert_eq!(h.id, 0);
        assert!((h.energy - 100.0).abs() < 1e-10);
        assert!(h.alive);
    }

    #[test]
    fn test_host_add_energy() {
        let mut h = Host::new(0, 50.0);
        h.add_energy(30.0);
        assert!((h.energy - 80.0).abs() < 1e-10);
    }

    #[test]
    fn test_host_remove_energy() {
        let mut h = Host::new(0, 50.0);
        let taken = h.remove_energy(30.0);
        assert!((taken - 30.0).abs() < 1e-10);
        assert!((h.energy - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_host_remove_more_than_has() {
        let mut h = Host::new(0, 10.0);
        let taken = h.remove_energy(50.0);
        assert!((taken - 10.0).abs() < 1e-10);
        assert!((h.energy - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_host_tick_survives() {
        let mut h = Host::new(0, 10.0);
        h.tick();
        assert!(h.alive);
        assert!((h.energy - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_host_tick_dies() {
        let mut h = Host::new(0, 0.5);
        h.tick();
        assert!(!h.alive);
    }

    #[test]
    fn test_host_symbiont() {
        let mut h = Host::new(0, 100.0);
        h.add_symbiont();
        assert!(h.has_symbionts());
        h.remove_symbiont();
        assert!(!h.has_symbionts());
    }

    #[test]
    fn test_effective_fitness() {
        let h = Host::new(0, 100.0);
        assert!(h.effective_fitness() > 1.0);
    }

    #[test]
    fn test_effective_fitness_dead() {
        let mut h = Host::new(0, 0.0);
        h.alive = false;
        assert!((h.effective_fitness() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_population_add() {
        let mut pop = HostPopulation::new();
        pop.add(100.0);
        pop.add(50.0);
        assert_eq!(pop.hosts.len(), 2);
    }

    #[test]
    fn test_population_alive_count() {
        let mut pop = HostPopulation::new();
        pop.add(100.0);
        pop.add(0.0);
        pop.hosts[1].alive = false;
        assert_eq!(pop.alive_count(), 1);
    }

    #[test]
    fn test_population_avg_energy() {
        let mut pop = HostPopulation::new();
        pop.add(100.0);
        pop.add(50.0);
        assert!((pop.avg_energy() - 75.0).abs() < 1e-10);
    }

    #[test]
    fn test_population_tick_all() {
        let mut pop = HostPopulation::new();
        pop.add(10.0);
        pop.tick_all();
        assert!((pop.hosts[0].energy - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_population_remove_dead() {
        let mut pop = HostPopulation::new();
        pop.add(100.0);
        pop.add(0.0);
        pop.hosts[1].alive = false;
        assert_eq!(pop.remove_dead(), 1);
    }

    #[test]
    fn test_population_get() {
        let mut pop = HostPopulation::new();
        let id = pop.add(100.0);
        assert!(pop.get(id).is_some());
        assert!(pop.get(999).is_none());
    }
}
