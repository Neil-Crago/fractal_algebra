# Fractal Algebra

A Rust framework for modular, recursive, and entropy-aware algebraic structures inspired by fractals, resonance, and cosmological symmetry.

## ✨ Overview

This project explores the intersection of algebra, geometry, and modular resonance through code. It defines a vector space over ℂ (`Complex<f32>`) using spatially distributed edge structures, and builds trait-based abstractions for algebraic operations, ring axioms, and fractal composition.

### Philosophy

Fractal Algebra is a meditation on mathematical beauty, modular coherence, and the recursive nature of structure. It seeks to formalize the intuition that symmetry, resonance, and entropy are not merely physical phenomena, but algebraic principles waiting to be encoded.

This framework treats algebra not as a static set of rules, but as a dynamic language for expressing transformation, emergence, and failure. By embedding fractal logic into edge-based structures and trait abstractions, we aim to explore:

- **Resonance as Algebra**: Phase, amplitude, and modular location become operands in a system where interference patterns encode meaning.
- **Entropy as Deformation**: Algebraic operations are not always clean; they stretch, decay, and mutate—capturing the thermodynamic spirit of real systems.
- **Structure as Emergence**: Recursive composition of edges and fields reveals higher-order symmetries, suggesting that complexity arises from simple, coherent rules.
- **Failure as Insight**: Experimental modules are separated from canonical ones to preserve clarity and embrace the creative role of breakdowns in discovery.

This is not just a Rust crate—it is an evolving mathematical language for expressing the geometry of thought.

## 🧱 Core Structures

### `FractalEdge`
- Represents modular elements with amplitude, location, and phase.
- Supports convolution, phase alignment, and entropy modulation.

### `GraphEdge`
- Encodes spatial edges with origin, direction, length, and depth.
- Used for recursive fractal generation and geometric scaffolding.

### `FractalField`
- A vector space over `Complex<f32>`, composed of `GraphEdge`s.
- Supports addition, scalar multiplication, negation, and identity.

## 🧠 Traits

### `FractalAlgebra`
Defines basic algebraic operations on `FractalEdge`:
- `add`, `scale`, `multiply`, `zero`

### `FractalRing`
Implements ring-like behavior on `FractalField`:
- `add`, `scale`, `multiply`, `negate`, `one`, `zero`
- `multiply_and_preserve_symmetry` for phase-aware fusion

## 🧪 Testing

Includes a suite of tests for:
- Ring axioms (associativity, identity, inverse, distributivity)
- Vector space laws over ℂ
- Canonical fractal structures and symmetry preservation

## 📦 Usage

```bash
cargo test
```

## 🚀 Future Directions

- `FractalSignature`: extract invariant features from fields
- `EntropyPulse`: simulate thermodynamic deformation
- `PhaseAligner`: normalize and fuse modular phase clusters
- `GraphField`: recursive spatial fractal generation
- Category-theoretic morphisms between algebraic domains

## 🧑‍💻 Author

Neil Crago — experimental mathematician, Rust architect, and philosophical explorer of mathematical beauty.

## 📄 License

MIT

---

