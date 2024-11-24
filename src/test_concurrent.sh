#!/bin/bash

# Function to make a request and show its process ID
make_request() {
    local num=$1
    echo "Starting request $num (PID: $$)"
    curl -i http://localhost/ &
    echo "Request $num sent"
}

# Make 7 concurrent requests
for i in {1..7}; do
    make_request $i
    sleep 0.1  # Small delay between requests
done

# Wait for all background processes to complete
wait

echo "All requests completed"