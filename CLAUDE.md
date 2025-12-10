# Ferrview - AI Development Context

## Project Overview

**Ferrview** is a lightweight Rust-based system monitoring tool with a client-server architecture:

- **ferrview-node**: Agent that collects system metrics (CPU, memory, temperature, disk, network)
- **ferrview-collector**: Server that receives, stores, and visualizes metrics
- **ferrview-common**: Shared data structures and utilities

**Tech Stack**: Rust, sysinfo, hyper, tokio, tracing, Alpine Linux packaging

## Architecture

```
ferrview-node (agent)
  ├── probes/sysinfo/    # CPU, memory, disk, network, temperature
  ├── client/            # HTTP client with retry logic
  ├── config.rs          # TOML configuration
  └── utils/             # Timestamp and helpers

ferrview-collector (server)
  ├── http/              # API endpoints
  ├── store/             # Time-series storage
  └── charts/            # Web dashboard

ferrview-common
  └── shared types and utilities
```

## Development Workflow

### Quick Commands

```
-> mise tasks --all
Name                Description
build-rust-dev      [DEV] cargo build
build-rust-release  [RELEASE] cargo build --release
check               Cargo check
lint                Lint with Clippy, failing on warnings
```

### Key Files

- `Cargo.toml` - Workspace configuration
- `ferrview-node/ferrview-node.toml` - Node agent config
- `ferrview-collector/ferrview-collector.toml` - Collector config

## PACT Framework (Simplified)

Use this systematic approach for all changes:

### [PREPARE]

**Before coding, understand the context:**

- Read relevant source files and documentation
- Check existing patterns in the codebase
- Verify dependencies and interfaces
- Review related test cases

### [ARCHITECT]

**Design before implementing:**

- Keep components single-purpose and loosely coupled
- Follow existing architectural patterns
- Design for testability
- Consider error handling upfront

### [CODE]

**Write clean, maintainable code:**

- Follow Rust idioms and best practices
- Write self-documenting code with clear names
- Add comments only for complex logic
- Keep functions focused and files under 500 lines
- Handle errors explicitly, avoid unwraps in production code
- Use structured logging (tracing) for observability

### [TEST]

**Verify quality:**

- Write unit tests for business logic
- Test error cases and edge conditions
- Run full test suite before committing
- Verify Alpine packages build correctly

## Code Quality Standards

### Rust Best Practices

- **Error Handling**: Use `Result<T, E>` and proper error types
- **Logging**: Use `tracing` macros (info!, warn!, error!)
- **Async**: Use tokio for async operations
- **Safety**: Minimize `unsafe`, avoid `.unwrap()` in production paths
- **Performance**: Use release builds for benchmarking

### Project Conventions

- Configuration via TOML files
- Logging with UTC timestamps
- Modular probe system for extensibility
- Retry logic for network operations
- Graceful degradation on probe failures

## Common Tasks

### Adding a New Probe

1. Create module in `ferrview-node/src/probes/`
2. Add config field in `config.rs`
3. Implement probe following existing pattern
4. Add tests
5. Update example config file

### Modifying API Endpoints

1. Update handler in `ferrview-collector/src/http/`
2. Update shared types in `ferrview-common`
3. Update client in `ferrview-node/src/client/`
4. Test end-to-end flow

## Current State

### Completed Features

- Core monitoring probes (CPU, memory, temperature, disk, network)
- HTTP client with retry logic
- Collector API and storage
- Web dashboard with charts
- Alpine packaging for node and collector
- Comprehensive test coverage

### Known Issues

- NA

### Next Steps (from README Roadmap)

- Process monitoring probe
- GPU information probe
- Advanced query interface for collector
- Data aggregation and rollups
- Alerting system

## Development Notes

### Performance Considerations

- Release builds use thin LTO and single codegen unit
- Probes designed to be lightweight and non-blocking
- Collection interval configurable per deployment

### Security

- Input validation on API endpoints
- No authentication yet (planned for future)
- Runs as unprivileged user in Alpine packages

### Deployment

- OpenRC init scripts for Alpine
- Systemd support planned
- Configuration in `/etc/ferrview/`
- Binaries in `/usr/bin/`

---

**Last Updated**: 2025-12-10
**Version**: 0.3.0
