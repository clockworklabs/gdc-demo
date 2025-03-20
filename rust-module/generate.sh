#!/bin/bash

mkdir -p ../demo-client/Assets/autogen
spacetime generate --out-dir ../demo-client/Assets/autogen --lang cs
spacetime generate --out-dir ../typescript-client --lang typescript
