# SeeYuj

> **Worlds shouldn't end just because the server restarts.**
> An infrastructure for persistent, autonomous realities.

🔗 GitHub: [https://github.com/Seeyuj/Seeyuj](https://github.com/Seeyuj/Seeyuj)
🧵 Follow: [https://x.com/SeeYuj](https://x.com/SeeYuj)

---

## The Manifesto

SeeYuj is not a game engine. It does not care about your frame rate, your polygons, or your shaders.

**SeeYuj cares about the Truth.**

It is a specialized kernel designed to run persistent simulations that continue to breathe, evolve, and function whether a player is watching or not. It is the "backend of the metaverse" stripped of the buzzwords and built on cold, hard systems engineering.

We believe that:
1.  **Simulation > Narration.** Stories should emerge from systems colliding, not from scripts written by a designer.
2.  **The World is Sovereign.** It exists without players. It does not pause. It does not wait.
3.  **Determinism is King.** Same input + Same state = Same result. Always.

---

## The Mission

**To enable the creation of persistent worlds without the infrastructural headache.**

Building a living, breathing world is usually a nightmare of database locks, race conditions, and netcode. Most creators give up before their world even starts ticking.

SeeYuj handles the heavy lifting. We provide a bulletproof, reliable platform for **any type of world**—whether it's a sci-fi economy, a fantasy simulation, or a social experiment.

We provide the **bedrock**:
*   **Time:** A rigorous, deterministic clock.
*   **Persistence:** State that survives crashes and restarts automatically.
*   **Safety:** A foundation where your world's history is preserved forever.

**You bring the rules. We bring the reality.**

---

## Why Build With Us?

Most game engines solve the problem of *rendering* and *input*. We are solving the problem of **existence**.

Building on SeeYuj means grappling with deep, fascinating engineering challenges:
*   How do you simulate a million entities without melting the CPU?
*   How do you ensure a simulation run today produces the *exact* same result ten years from now?

**Join us if:**
*   You are tired of "gameplay code" and want to write **system architectures**.
*   You want to build the foundation that thousands of future worlds will stand upon.
*   You love Rust and uncompromising quality.

---

## Architecture

We don't do "spaghetti code". SeeYuj is built on strict strict architectural layers (NIV 0 to NIV 4).

*   **The Core (Rust):** Pure, isolated, standard-library only. It computes the next tick. That's it.
*   **The Ports:** We use dependency inversion. The simulation doesn't know it's saving to a disk or sending packets. It just emits events.
*   **The Modules:** Physics, Economics, AI. Plug them in, or write your own.

> **Status: Foundation Phase 🟡**
> The bedrock is setting. We are locking in the invariants. There is no gameplay yet. Only pure, unadulterated infrastructure.

---

## Quick Start

You need **Rust**. That's it.

```bash
# 1. Clone the reality
git clone https://github.com/Seeyuj/Seeyuj.git
cd Seeyuj

# 2. Ignite the core
cd seeyuj/server
cargo run --bin server_d
```

If it compiles, the laws of physics are holding.

---

## Contributing

**We are looking for Architects.**

This is a project for those who enjoy the craft of software engineering. Whether you are a master of distributed systems, a determinism wizard, or a documentation artist, there is a place for you here.

We maintain high standards because we are building a foundation for others to dream on.

1.  Check out the [Issues](https://github.com/Seeyuj/Seeyuj/issues) to find a quest.
2.  Read [CONTRIBUTING.md](CONTRIBUTING.md) to understand our laws of physics.
3.  Open a PR and help us build the engine of the future.

---

> *"A persistent world is a system, not a story."*
