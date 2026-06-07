# Symbiont Network

[![crates.io](https://img.shields.io/crates/v/symbiont-network.svg)](https://crates.io/crates/symbiont-network)
[![docs.rs](https://docs.rs/symbiont-network/badge.svg)](https://docs.rs/symbiont-network)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Symbiotic relationships between agent pairs — mutualism, parasitism, and commensalism.**

---

## The Problem

Multi-agent systems treat all agent interactions as either cooperative or competitive. But biology recognizes a richer spectrum: **mutualism** (both benefit), **parasitism** (one benefits, one suffers), and **commensalism** (one benefits, one is unaffected). Modeling these nuanced relationships enables more sophisticated agent ecosystems.

## Why This Exists

Symbiont Network models the three fundamental biological symbiotic relationships for software agents:
- **Mutualism**: Both agents gain fitness from the relationship
- **Parasitism**: One agent gains at the expense of the other
- **Commensalism**: One agent gains without affecting the other

Combined with host tracking and fitness scoring, this creates a rich ecology of agent interactions.

## Architecture

```
  ┌─────────┐                    ┌─────────┐
  │ Agent A │◄── Mutualism ────►│ Agent B │
  │ +fitness│                    │ +fitness│
  └─────────┘                    └─────────┘

  ┌─────────┐                    ┌─────────┐
  │ Agent C │◄── Parasitism ───►│ Agent D │
  │ +fitness│                    │ -fitness│
  └─────────┘                    └─────────┘

  ┌─────────┐                    ┌─────────┐
  │ Agent E │◄─ Commensalism ──►│ Agent F │
  │ +fitness│                    │  ~same  │
  └─────────┘                    └─────────┘
```

## Installation

```toml
[dependencies]
symbiont-network = "0.1"
```

## Modules

| Module | Description |
|--------|-------------|
| `mutualism` | Mutualistic relationships (both benefit) |
| `parasitism` | Parasitic relationships (one benefits, one suffers) |
| `commensalism` | Commensal relationships (one benefits, one neutral) |
| `host` | Host tracking and relationship management |
| `fitness` | Fitness scoring for relationship evaluation |

## Usage Examples

### Example 1: Mutualistic Agent Pair

```rust
use symbiont_network::mutualism::*;
use symbiont_network::fitness::*;

// Both agents contribute and benefit
let result = apply_mutualism(agent_a, agent_b, contribution_a, contribution_b);
// Both fitness scores increase
```

### Example 2: Fitness Tracking

```rust
use symbiont_network::fitness::*;

// Track agent fitness over time
// Higher fitness = more successful in the ecosystem
```

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests
4. Push and open a Pull Request

## Mathematical Background

**Mutualism** (σ = +/+): Both agents experience positive fitness change. Modeled as:

```
Δf_A = benefit_received_from_B − cost_of_helping_B
Δf_B = benefit_received_from_A − cost_of_helping_A
```

Stable when both Δf > 0.

**Parasitism** (σ = +/−): One agent benefits at the other's expense:

```
Δf_host = −damage
Δf_parasite = +benefit
```

**Commensalism** (σ = +/0): One agent benefits without affecting the other:

```
Δf_A = +benefit
Δf_B = 0
```

### Fitness Equations

Fitness is tracked over time using a decay-weighted sum:

```
F(t) = Σᵢ wᵢ × fᵢ × decay^(t − tᵢ)
```

Where wᵢ is the weight of interaction i, fᵢ is the fitness change, and decay controls how quickly old interactions become irrelevant.

## Theoretical Background

Biological symbiosis was first described by Heinrich Anton de Bary in 1879 as "the living together of unlike organisms." The three primary types map directly to agent relationships:

- **Mutualism** → Cooperative microservices, distributed caching
- **Parasitism** → Resource exhaustion attacks, free-riding agents
- **Commensalism** → Log readers, monitoring agents

Understanding which type of relationship exists between agent pairs enables proper resource allocation, trust scoring, and eviction policies.

## Performance Characteristics

| Operation | Complexity |
|-----------|-----------|
| Relationship classification | O(1) |
| Fitness update | O(1) |
| Fitness history scan | O(n) |
| Relationship lookup | O(1) |

## Comparison with Alternatives

| Feature | symbiont-network | game-theory libs | ecology sim |
|---------|-----------------|------------------|-------------|
| Mutualism modeling | ✅ | ❌ | ✅ |
| Parasitism detection | ✅ | ✅ | ✅ |
| Commensalism | ✅ | ❌ | ❌ |
| Fitness tracking | ✅ | ✅ | Varies |
| Agent-pair focus | ✅ | ❌ | ❌ |

## API Reference

### `mutualism`

Mutualistic relationship management where both agents benefit:

```rust
use symbiont_network::mutualism::*;

// Both agents contribute and receive benefits
// Fitness of both increases from the interaction
```

### `parasitism`

Parasitic relationship detection where one agent exploits another:

```rust
use symbiont_network::parasitism::*;

// One agent gains fitness at the expense of the other
// Useful for detecting resource exhaustion patterns
```

### `commensalism`

Commensal relationships where one benefits and one is neutral:

```rust
use symbiont_network::commensalism::*;

// One agent benefits without affecting the other
// Models monitoring, logging, and observation patterns
```

### `host`

Host tracking and relationship management:

```rust
use symbiont_network::host::*;

// Track which agents are hosts and which are symbionts
// Manage relationship lifecycle
```

### `fitness`

Fitness scoring for relationship evaluation:

```rust
use symbiont_network::fitness::*;

// Compute fitness scores for agents
// Track fitness over time with decay weighting
// Compare fitness between symbiotic pairs
```
