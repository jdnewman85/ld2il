#!/bin/bash

set -euo pipefail

echo "Clearing dot dir"
rm "./dot/"* && true

cargo --color=always run 2>&1 | \
	less -r -X
