#!/bin/bash

script_dir="$(cd "$(dirname "$0")" && pwd)"
project_dir="$(cd $script_dir/.. && pwd)"

source $script_dir/setup_venv.sh

setup_venv

# mkdir "sparse_instances"
# for num_nodes in 100 200 300 400 500 600 700 800 900 1000 1100 1200 1300 1400 1500 2000 2500 3000 3500 4000 4500 5000 6000 7000 8000
# do
#     probability=$(echo "scale=10; 1/$num_nodes" | bc)  # Adjust the scale as needed
#     output_filename="sparse_instances/sparse_${num_nodes}.gph"

#     # Run the Python script with command-line arguments
#     python3 generate_graph.py $num_nodes $probability $output_filename

#     echo "Generated: $output_filename"

# done
# rm sparse_instances.zip
# zip -r sparse_instances.zip sparse_instances
# rm -r sparse_instances


mkdir "$project_dir/dense_instances"
for num_nodes in 100 200 300 400 500 600 700 800 900 1000 1100 1200 1300 1400 1500 2000 2500 3000 
do
    probability=1  # Adjust the scale as needed
    output_filename="$project_dir/dense_instances/dense_${num_nodes}.gph"

    # Run the Python script with command-line arguments
    python3 $script_dir/generate_graph.py $num_nodes $probability $output_filename

    echo "Generated: $output_filename"

done

# rm dense_instances.zip
# zip -r dense_instances.zip dense_instances
# rm -r dense_instances
deactivate