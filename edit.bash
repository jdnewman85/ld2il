#!/bin/bash

set -euo pipefail

sources=$(find "./src" -type f -name "*.rs")

$EDITOR ${sources}
