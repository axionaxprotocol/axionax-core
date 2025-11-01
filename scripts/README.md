# Axionax Scripts

This directory contains utility scripts for building, testing, and deploying Axionax Core.

## Available Scripts

### Testing Scripts

- **`run_tests.sh`** - Main unified test script (Linux/macOS)
  - Builds Rust core
  - Compiles Python bindings
  - Runs all integration tests
  - Executes benchmarks

- **`test.ps1`** - PowerShell test script (Windows)
  - Windows equivalent of run_tests.sh

- **`quick-test.ps1`** - Quick test runner (Windows)
  - Fast testing for development

- **`test-quick.ps1`** - Alternative quick test (Windows)
  - Rapid testing cycles

## Usage

### Linux/macOS

```bash
# Run all tests
./scripts/run_tests.sh

# Make script executable if needed
chmod +x ./scripts/run_tests.sh
```

### Windows (PowerShell)

```powershell
# Run all tests
.\scripts\test.ps1

# Quick test
.\scripts\quick-test.ps1
```

## Adding New Scripts

When adding new scripts:

1. Place them in this directory
2. Use descriptive names
3. Add execute permissions (Linux/macOS): `chmod +x script.sh`
4. Update this README
5. Add documentation comments in the script

## Best Practices

- Keep scripts simple and focused
- Add error handling
- Document parameters and usage
- Test on target platforms
- Use consistent naming conventions

## Documentation

For more information, see:
- [Testing Guide](../docs/TESTING_GUIDE.md)
- [Contributing Guide](../docs/CONTRIBUTING.md)
- [Main Documentation](../docs/)
