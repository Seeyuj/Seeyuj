# Persistent Sandbox Worlds Platform

> Open-source infrastructure for simulating autonomous and persistent worlds.

This repository contains the **simulation core** of a platform enabling the construction of living, coherent, and durable sandbox worlds.

## Navigation

- [`readme.md`](readme.md)
- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`DECISIONS.md`](DECISIONS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

This project **is not**:

- a video game
- a graphics engine
- a narrative RPG
- a gameplay framework
- a technology showcase

---

## Project Objective

The objective is to provide a **stable software foundation**, maintainable in the long term, enabling developers, creators, or communities to build **their own persistent worlds**, each with their own rules, systems, and uses.

The project positions itself as an **infrastructure**, similar to an operating system or a distributed orchestrator.

> Content, gameplay, and aesthetics are outside the scope of the core.

---

## Fundamental Principles

These principles are **non-negotiable**.  
Any contribution or proposal that violates them is rejected.

### Autonomous World

- The world exists independently of players
- The server can run without any connected clients
- No central scenario
- No hero
- Players are agents among others

The world evolves according to its own rules.

---

### Simulation Before Narration

- The heart of the project is a systemic simulation
- Time, space, entities, and rules are explicitly modeled
- Narration is emergent
- No narrative content is coded in the core

If a story exists, it is the result of the system.

---

### Authoritative Server

- The server is the sole source of truth
- The client never decides the world state
- Solo mode = local server
- Multiplayer mode = remote server
- Real persistence on disk
- Simulation by regions with detail levels

The client consumes the world state, it does not define it.

---

### Strict Decoupling Between Simulation and Rendering

- The core knows no graphics engine
- The official client uses Unreal Engine
- Other clients can exist (Godot, Web, CLI)
- Rendering is interchangeable

The visual is an implementation, not a dependency.

---

### Framed Freedom

- Worlds are defined by systems and rules
- Extensions go through versioned APIs
- Compatibility and stability take precedence over total freedom

Freedom exists **within the framework of the system**, never outside.

---

### Pragmatic AI

- No conscious AI
- No marketing promises
- Agents are deterministic and explainable
- Generative AI is optional and peripheral

AI is a tool, not an architectural pillar.

---

## What the Core Provides

The core is intentionally **minimal and strict**.

It provides:

- a persistent time system
- a representation of simulated space
- persistent entities
- modular rule systems
- systemic events
- a save and recovery mechanism
- documented and versioned APIs
- functional simulation without a client

---

## What the Core Will Never Do

The core **does not contain**:

- user interface
- HUD
- quests
- written dialogues
- narrative content
- graphical assets
- "fun"-oriented balancing
- player tutorials
- central generative AI

If a feature is not necessary for world simulation, it has no place here.

---

## Graphics Standard (Official Client)

An official client based on **Unreal Engine** is provided as a reference implementation.

Objectives:

- modern rendering (PBR, credible lighting)
- professional asset pipeline
- high but scalable visual quality
- compatibility with community content

Constraints:

- no fragile photorealism
- no imposed cinematics
- no heavy visual narration

The client is a technical showcase, not the heart of the project.

---

## Governance

This project is open-source, but **not without governance**.

- Identified core maintainers
- Clearly defined responsibilities
- Strict review process
- Documentation prioritized
- Assumed refusals
- Stability > speed

The project prioritizes coherence and longevity over popularity.

---

## Contributing

Before any contribution:

1. Read this README in full
2. Accept the fundamental principles
3. Understand that some ideas will be refused

Detailed rules are defined in:

- `CONTRIBUTING.md`
- `ARCHITECTURE.md`
- `DECISIONS.md`

---

## Decision Rule

Any proposal is evaluated according to a single question:

> **Is this feature necessary for the persistent simulation of the world?**

If the answer is no, it does not belong in the core.

---

## Project Status

The project is in the foundation phase.  
The current priority is **conceptual and architectural solidity**, not development speed.
