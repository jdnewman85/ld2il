#!/bin/bash

set -euo pipefail

main_source=$(fd 'main.rs' 'src')
sources=$(fd '.rs' --exclude 'main.rs' 'src')

$EDITOR ${main_source} ${sources}
