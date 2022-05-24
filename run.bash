#!/bin/bash

set -euo pipefail

cargo --color=always run 2>&1 | \
	less -r -X
