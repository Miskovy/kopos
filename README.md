# Kopos

Kopos is a proposed research track for worker execution, parallelism, and a potentially reusable Rust computation backend. Its first architectural reference is [Parallel JavaScript Runtime (PJS)](https://github.com/Miskovy/parallel-javascript). This is not a runtime implementation or a commitment to replace PJS.

**Status: research reference plus a minimal Rust 2024 library scaffold.** Following the planning assignment, the owner authorized Cargo initialization and a SOLID folder design. Runtime behavior, native bindings, and execution backends have not been implemented. Research recommendations remain proposed until reviewed. PJS Node-based v1.0 stabilization proceeds independently and must remain releasable if every Kopos experiment is abandoned.

The project uses a small ports-and-adapters structure with encapsulation, focused traits when needed, and composition. Start with the [Rust project design](docs/design.md) for the folder map, dependency rules, and development commands.

## Read the reference set

| Document                                                                              | Purpose                                                                                                                     |
| ------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| [Architecture assessment](docs/research/architecture-assessment.md)                   | Inspected source identities, dependency map, primary evidence, alternatives, host support, distribution, and recommendation |
| [Execution and ownership contract](docs/research/execution-and-ownership-contract.md) | Execution semantics, data lifetimes, failure boundaries, compiler limits, and the optional `.pjs` workflow hypothesis       |
| [Roadmap](docs/roadmap.md)                                                            | Independent decision gates, future experiment specification, acceptance criteria, and the smallest next implementation task |
| [Decision ledger](docs/decisions.md)                                                  | Owner requirements versus proposed, deferred, and rejected directions; unresolved choices and revision triggers             |

Read the assessment before the roadmap. The recommendation is to preserve host-worker execution for general developer JS/TS, investigate host adapters for portability, and evaluate Rust through a small optional native-kernel experiment. A native scheduler alone does not establish value or execute arbitrary JS callbacks.

## Scope and evidence

The original assignment authorized research and documentation alongside learning Rust. A follow-up authorized the Cargo scaffold and folder structure now present. The crate has no third-party dependencies or runtime behavior; native addons, compilers, worker backends, benchmarks, and PJS production changes remain outside the scaffold. There are no new performance or qualification results. Historical R1/R2/R3 reports are behavioral and methodological references, not qualification of Kopos.

The separate workspace is `/home/miskovy/Documents/kopos`. PJS was read at `/home/miskovy/Documents/pjs`; its checkout was preserved. The assessment records the remote main SHA, local checkout SHA, and unavailable historical S1 candidate separately.

## Local Git setup

The local Git repository is initialized on `main`, with `origin` set to the owner-supplied [Miskovy/kopos](https://github.com/Miskovy/kopos) repository. The initial research session encountered a read-only `.git` mount; initialization and local remote configuration succeeded during scaffolding through the available permission mechanism. Existing `.idea` files were preserved and are ignored alongside Cargo's `target` directory.

No commit, push, publication, merge, tag, registry change, or remote write was performed.
