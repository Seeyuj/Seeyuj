/see-yuj-platform (Seeyuj/)
├── /docs                      # Documentation en anglais
│   ├── ARCHITECTURE.md
│   ├── DATA_FLOW.md
│   └── DEPENDENCY_RULES.md
│
├── /schemas
│   └── README.md
│
├── /server                    # Workspace Rust
│   ├── README.md
│   ├── Cargo.toml
│   ├── rust-toolchain.toml
│   │
│   ├── /crates
│   │   ├── /sy_types          # NIV 0
│   │   ├── /sy_config         # NIV 0
│   │   ├── /sy_protocol       # NIV 1 (avec build.rs)
│   │   ├── /sy_api            # NIV 1 (commands, events, errors, validation)
│   │   ├── /sy_core           # NIV 2 (world, sim, ports/)
│   │   ├── /sy_infra          # NIV 3 (net, store, clock, rng, observability)
│   │   ├── /sy_loader         # NIV 4
│   │   ├── /sy_tools          # NIV 3 (inspect, replay)
│   │   └── /sy_testkit        # NIV 3 (mocks, scenarios)
│   │
│   ├── /mods
│   │   ├── /mod_economics     # NIV 2b
│   │   └── /mod_physics       # NIV 2b
│   │
│   ├── /bin
│   │   ├── /server_d          # Daemon principal
│   │   └── /sy_cli            # CLI admin
│   │
│   └── /examples
│       ├── minimal_headless.rs
│       └── replay_demo.rs
│
└── /clients                   # Vide (prêt pour plus tard)