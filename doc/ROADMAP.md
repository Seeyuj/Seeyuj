# Roadmap – Seeyuj

> This roadmap describes the **major construction steps** of the platform.  
> It is **neither exhaustive nor contractual**, and may evolve according to governance decisions.

## Navigation

- [`README.md`](../README.md)
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

🟢 **Status: Foundational phase — non-negotiable**

🎯 **Single (exclusive) objective**

Prove that a simulated world can exist, evolve, and persist with **no client, no player, and no graphical interface**.

This phase explicitly does **not** target:
- feature richness
- performance
- advanced extensibility
- user experience

👉 It only aims to make the world's autonomous existence undeniable.

🧠 **Guiding principle**

If the server can run alone for hours, is killed abruptly, restarts, and the world continues as if nothing happened, then Phase 1 is validated.

Anything that does not directly serve this proof is out of scope.

✅ **Mandatory capabilities (strict scope)**

1. **Deterministic simulation loop**
   - Explicit tick execution
   - Strictly controlled execution order
   - No implicit dependencies:
     - no direct system clock access
     - no non-injected RNG
   - Same inputs ⇒ same outputs

   📌 Determinism is a functional requirement, not an optimization.

2. **Persistent simulated time**
   - World time is simulated data
   - It progresses independently of any human presence
   - It is explicitly stored and restored on restart
   - No dependency on real time

3. **Minimal spatial representation**
   - A structured space exists (zones / regions / chunks — free form)
   - Space is persistent
   - Space can be partially loaded / simulated
   - No realism or spatial optimization goals

   📌 Space may be abstract. It only needs to exist.

4. **Persistent non-player entities**
   - Entities have:
     - stable identity
     - internal state
     - lifecycle
   - They exist without players, evolve via simple systemic rules, and survive restarts

   📌 No “player” entities. No exceptions.

5. **Minimal systemic rules**
   - At least one causal rule exists (e.g., consumption, degradation, transformation, movement)
   - It produces observable state changes
   - It depends only on the simulation
   - No gameplay or balancing goals

   📌 A single rule is enough if it is real and persistent.

6. **Explicit on-disk persistence**
   - Every world mutation is explicitly written to disk and traceable
   - The world must never vanish when the server stops
   - No reliance on implicit in-memory state

   Persistence is critical.

7. **Recovery after stop or crash**
   - The server may be killed abruptly and restarted
   - The world is automatically restored, coherent, and requires no human intervention

   📌 A lost world = Phase 1 failure.

8. **Headless server execution**
   - No graphical interface
   - No dependency on any rendering engine
   - No connected client required
   - “Solo” = local server; “Multi” = the same server, remote

👁️ **Minimal observability required**

The world must be observable without a UI. Acceptable means:
- structured logs
- state dumps
- basic CLI tools
- diagnostic files

📌 No graphical visualization is required.

⛔ **Explicit out of scope (forbidden in Phase 1)**

Formally excluded:
- any graphical client
- any UI (even advanced debug UI)
- any player-oriented logic
- optional modules
- advanced AI
- complex economy
- advanced networking
- CPU/memory optimization
- parallelism
- balancing or “fun”
- graphics standards
- stabilized public API

👉 Introducing any of these invalidates Phase 1.

🧪 **Validation criteria (measurable)**

Phase 1 is complete only if:
- the server can run alone indefinitely
- the world evolves without human interaction
- a restart destroys nothing
- two runs with the same inputs produce the same world
- the server has zero graphical dependencies

🧱 **Final rule (non-negotiable)**

Phase 1 is not meant to be impressive. It is meant to be irrefutable.

Anything that does not help prove the world's autonomous, persistent existence does not belong here.

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
