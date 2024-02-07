import networkx as nx
import random  # Import the random module
import argparse

def generate_random_undirected_graph(num_nodes, probability):
    # Generate a random undirected graph
    graph = nx.fast_gnp_random_graph(num_nodes, probability, directed=False)

    # Assign random positions to nodes
    # positions = {node: (i, j) for i, node in enumerate(graph.nodes()) for j in range(num_nodes)}

    # Assign random weights to edges
    for edge in graph.edges():
        graph[edge[0]][edge[1]]['weight'] = int(round(random.uniform(0, 1), 2) * 100)

    return graph

# def save_graph_to_gph(graph, filename):
#     with open(filename, 'w') as file:
#         # Write node positions and weights
#         file.write(f"{graph.number_of_nodes()} {graph.number_of_edges()}\n")

#         # file.write(f"{positions[node][0]} {positions[node][1]}\n")
#         for node in graph.nodes():
#             # file.write(f"{positions[node][0]} {positions[node][1]}\n")

#             for neighbor in graph.neighbors(node):
#                 weight = graph[node][neighbor]['weight']
#                 file.write(f"{node+1} {neighbor+1} {weight}\n")

def save_graph_to_gph(graph, filename):
    with open(filename, 'w') as file:
        # Write number of nodes and number of edges
        file.write(f"{graph.number_of_nodes()} {graph.number_of_edges()}\n")

        # Write node positions and weights
        written_edges = set()  # To keep track of written edges
        for node in graph.nodes():
            for neighbor in graph.neighbors(node):
                if (node, neighbor) not in written_edges and (neighbor, node) not in written_edges:
                    # Write edge information
                    weight = graph[node][neighbor]['weight']
                    file.write(f"{node+1} {neighbor+1} {weight}\n")
                    # file.write(f"{positions[neighbor][0]} {positions[neighbor][1]} {weight}\n")

                    # Mark the edge as written
                    written_edges.add((node, neighbor))

if __name__ == "__main__":
    # Set up command-line argument parser
    parser = argparse.ArgumentParser(description="Generate a random undirected graph and save it to a .gph file.")
    parser.add_argument("num_nodes", type=int, help="Number of nodes in the graph")
    parser.add_argument("probability", type=float, help="Probability of an edge between any pair of nodes")
    parser.add_argument("output_filename", type=str, help="Output filename for the .gph file")

    # Parse command-line arguments
    args = parser.parse_args()

    # Example usage
    graph = generate_random_undirected_graph(args.num_nodes, args.probability)
    save_graph_to_gph(graph, args.output_filename)
