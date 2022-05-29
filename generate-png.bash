#!/bin/bash

set -euo pipefail

cargo --color=always run \
	2> /dev/null \
	| dot -Tpng > 'test.png'

python -m http.server 8080
