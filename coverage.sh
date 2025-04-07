#!/bin/bash
cargo llvm-cov clean --workspace
cargo llvm-cov --lcov --output-path lcov.info