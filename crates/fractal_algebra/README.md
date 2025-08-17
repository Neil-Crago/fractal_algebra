<p align="center">
  <img src="docs/assets/big_fractal_algebra.svg" alt="Fractal Algebra Banner" width="30%" />
</p>    

# Fractal Algebra

This is the primary research and application crate where the computational and geometric engines are brought together. It focuses on discovering and analyzing the emergent structures from number-theoretic data.

One of the key innovations in this crate is the **`FractalGraph`**:

- A custom data structure designed as a hybrid of a graph, a trie, and a spatial index.
- It moves beyond simple visualization to create a rich, explorable network of relationships between numbers based on their prime factorizations.
- Nodes in the graph represent `n!`, and relationships (edges) are formed dynamically based on mathematical similarity and shared number-theoretic traits.
- Traversal of the graph mimics fractal expansion: recursive, self-similar, and semantically rich.

Another important innovation is that of **Resonance Algebra** a trait-based algebraic extension for composing, transforming, and evaluating mathematical objects based on resonance. It encodes semantic meaning directly into traits, operations, and laws—bridging mathematical abstraction with computational clarity.

#### This includes:

- Resonance trait for scoring, comparing, and classifying fractal objects
- ResonanceLaw enum for semantic behavior (e.g. Harmony, Echo, Dissonance)
- ResonantTransform trait for resonance-aware evolution
- CompositeTransform for chaining transformations
- FractalCollection and ResonantFractalCollection for algebraic composition
- ResonanceFilter for semantic selection and pruning
- ResonanceRuleEngine for enforcing algebraic laws

## 🧪 Example Usage (for Resonance)

```rust
let a = FractalEdge::new(...); // Harmony, score 0.8
let b = FractalEdge::new(...); // Harmony, score 0.9

let union = a.add(&b); // Allowed by ResonanceRuleEngine

let filter = LawFilter {
    allowed: vec![ResonanceLaw::Harmony],
};

let filtered = resonant_collection.filter(&filter);
```

---


## Related Projects within the workspace

### 1. `factorial_engine`

A high-performance, zero-error crate for computing the prime factorization of factorials (`n!`). It uses **Legendre's Formula** to calculate results directly and efficiently, serving as a robust computational backend for number-theoretic analysis.

### 2. `tma_engine`

A lightweight crate providing the algebraic tools for 2D affine transformations (`TMA`). This engine is the geometric heart of the project, used for generating fractals via Iterated Function Systems (IFS) and the Chaos Game algorithm.

## Vision

The overarching goal of this project is to develop tools and theories based on a "fractal" understanding of mathematical and physical systems. We explore the idea that complexity arises from simple, iterative rules, and that the relationships between objects are as important as the objects themselves.

The project investigates concepts such as:
- The geometric patterns hidden in the prime factorizations of integers.
- The potential for a unified "fractal algebra" to describe phenomena across different scales.
- The nature of time and observation in a universe with fractal structure.


## 📚 Documentation

- Overview & Philosophy
- Core Concepts
- Trait Architecture
- Semantic Scaffolding
- Examples & Use Cases
- Design Philosophy
- Contributor Guide
- Manifesto

---

## 🤝 Contributing

We welcome thoughtful contributions that align with the framework’s philosophy.  
Please read the Contributor Guide before submitting a PR.

---

## 🧬 License

MIT License © Neil Crago  
This project is open-source and designed for philosophical and mathematical exploration.

---

## 🌱 A Final Note

> “This is not just a framework. It is a way of thinking.”
`
