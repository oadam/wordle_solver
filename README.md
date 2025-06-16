## Build
cargo build --release

# Ensure enough swap
    sudo fallocate -l 64G swapfile
    sudo chmod 600 swapfile
    sudo mkswap swapfile
    sudo swapon swapfile
    swapon --show

# Run
    target/release/wordle_solver

# Disable swap
    sudo swapoff swapfile
    sudo rm swapfile

# Results
    Results are in `result.txt`
