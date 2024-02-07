#!/bin/bash

# Set the name of your virtual environment
venv_name="apsp_venv"

# Determine the absolute path of the script directory
script_dir="$(cd "$(dirname "$0")" && pwd)"
project_dir="$script_dir/.."

# Function to check and create virtual environment
setup_venv() {
    if [ ! -d "$project_dir/$venv_name" ]; then
        python3 -m venv "$project_dir/$venv_name"
        source "$project_dir/$venv_name/bin/activate"
        pip install -r "$project_dir/requirements.txt"
        echo "Virtual environment '$venv_name' created and dependencies installed."
    else
        source "$project_dir/$venv_name/bin/activate"
        echo "Virtual environment '$venv_name' already exists. Skipping creation."
    fi
}