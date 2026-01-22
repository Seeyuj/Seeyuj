# Roadmap – Persistent Sandbox Worlds Platform

> This roadmap describes the **major construction steps** of the platform.  
> It is **neither exhaustive nor contractual**, and may evolve according to governance decisions.

## Navigation

- [`readme.md`](readme.md)
- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`DECISIONS.md`](DECISIONS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

The project's absolute priority is the **solidity of the simulation core**, its **maintainability**, and its **long-term sustainability**.

---

## Phase 0 – Conceptual Foundations (Current)

🎯 **Objective: Lock in the vision and invariants**

This phase aims to ensure that the project rests on clear, understandable, and defensible foundations over time.

Expected deliverables:
- Vision and fundamental principles clearly documented
- Explicit definition of what the project **is** and **is not**
- Clear separation between:
  - simulation core
  - optional modules
  - clients
- Foundational documentation:
  - README
  - CONTRIBUTING
  - Code of Conduct
  - initial architecture documents

No "functional" implementation is a priority until foundations are stabilized.

---

## Phase 1 – Minimal Simulation Core (Server Only)

🎯 **Objective: A world that exists without a client**

This phase validates the project's heart: a server capable of simulating a persistent world **without any graphical rendering**.

Key features:
- Deterministic simulation loop
- Persistent time system
- Space representation (zones / regions / chunks)
- Persistent entities (state, identity, lifecycle)
- Basic systemic rules
- Explicit persistence to disk
- Recovery after shutdown / crash
- Headless server execution

At this stage:
- no graphical client
- no UI
- no player-oriented logic

The world must be observable via logs, CLI tools, or state dumps.

---

## Phase 2 – Modular Architecture and Public APIs

🎯 **Objective: Enable extension without weakening the core**

Once the minimal core is stable, the focus shifts to controlled extensibility.

Main axes:
- Definition of versioned public APIs
- Optional module system
- Module loading / activation / deactivation
- Strict isolation between core and extensions
- Compatibility and versioning management
- Documentation of core invariants

This phase is critical to:
- avoid feature creep
- guarantee project longevity
- enable healthy community contribution

---

## Phase 3 – Advanced Simulation and Scalability

🎯 **Objective: A credible world at large scale**

The world must be able to:
- grow
- become more complex
- survive over time

Work axes:
- Simulation by regions with detail levels
- CPU / memory optimization
- Management of large numbers of entities
- Systemic events (economy, shortages, migrations, conflicts)
- Replay and deterministic validation tools
- Advanced observability (metrics, diagnostics)

The focus remains **systemic**, never game-oriented.

---

## Phase 4 – Reference Client (Unreal Engine)

🎯 **Objective: Visualize the world, not define it**

An official client based on Unreal Engine is introduced as:
- reference implementation
- technical showcase
- visual validation tool

Characteristics:
- Strict consumer of server state
- No critical calculations on the client side
- Modern but sober rendering
- Documented graphics standard
- Coherent and extensible asset pipeline

The client **never drives** world evolution.

---

## Phase 5 – Tools, SDK, and Community Opening

🎯 **Objective: Make the project a true platform**

Final structuring phase before maturity:

- SDK for module developers
- Tools for administering persistent worlds
- Advanced documentation (guides, diagrams, examples)
- Server templates
- Expanded community governance
- Stabilized contribution process

The project then becomes a **reusable infrastructure**, independent of any official content.

---

## Out of Scope

This roadmap **does not include**:
- "fun-first" gameplay
- player balancing
- written narration
- quests
- cinematics
- default content
- monetization
- marketing promises

These elements belong to **worlds created from the platform**, not to the platform itself.

---

## Guiding Principle

Each step is validated by a single question:

> **Can the world exist, evolve, and persist without a player or client?**

If the answer is no, the foundation is not yet sufficient.
