#!/usr/bin/env bash

pushd frontend && npx tailwindcss -i ./input.css -o ./public/tailwind.css && popd
pushd frontend && dx serve &
pushd backend && cargo run
