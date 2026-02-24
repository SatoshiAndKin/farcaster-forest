# Runbooks

This directory contains operational runbooks for common maintenance and troubleshooting scenarios.

## Available Runbooks

- [Build Failures](./build-failures.md) - How to diagnose and fix build issues
- [Release Process](./release-process.md) - Steps for creating a new release
- [CI/CD Troubleshooting](./ci-troubleshooting.md) - Fixing GitHub Actions failures

## What is a Runbook?

A runbook is a step-by-step guide for handling specific operational scenarios. Each runbook should include:

1. **Problem Description** - What issue are we addressing?
2. **Prerequisites** - What you need before starting
3. **Steps** - Detailed instructions to resolve the issue
4. **Verification** - How to confirm the issue is resolved
5. **Prevention** - How to avoid this issue in the future

## When to Create a Runbook

Create a runbook when:
- You encounter a recurring issue that requires specific steps to resolve
- There's a complex process that needs documentation (releases, deployments)
- New team members or AI agents need guidance on operational tasks
- You want to document tribal knowledge

## Quick Reference

### Build Issues
See [Build Failures](./build-failures.md)

### Release Process
See [Release Process](./release-process.md)

### CI/CD Issues
See [CI/CD Troubleshooting](./ci-troubleshooting.md)

## Contributing

When adding a new runbook:
1. Create a new markdown file in this directory
2. Follow the standard structure (Problem, Prerequisites, Steps, Verification, Prevention)
3. Add a link to it in this README
4. Keep it concise and actionable
