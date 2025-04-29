#!/bin/sh
cd frontend && dx bundle
cd ../server && cargo run
