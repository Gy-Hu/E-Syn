import os
import sys
import csv
import re
from tqdm import tqdm
import networkx as nx
import concurrent.futures

def process_circuits(i):
    # preprocess_dot 
    with open(f"out_dot/out_graph_dot{i}.dot", 'r') as f:
        dot_string = f.read()
    # Remove subgraph and cluster lines
    dot_string = re.sub(r'\s*subgraph.*{', '', dot_string)
    dot_string = re.sub(r'\s*}\s*', '', dot_string)
    dot_string = re.sub(r'\s*style=.*', '', dot_string)
    # Remove compass points
    dot_string = re.sub(r':\w+', '', dot_string)

    # add additional `}` to the end of the string
    dot_string += "}"
    with open(f"out_dot/out_graph_dot{i}.dot", 'w') as f:
        f.write(dot_string)

    graph = nx.DiGraph(nx.nx_pydot.read_dot(f"out_dot/out_graph_dot{i}.dot"))
    print("graph density:", nx.density(graph))
    graph_info = {}
    # fast calculation
    graph_info['graph_density'] = nx.density(graph)
    graph_info['graph_edge_count'] = graph.number_of_edges()
    return graph_info

if __name__ == "__main__":
    file_count = int(sys.argv[1])
    tasks_args = list(range(file_count))
    results = []

    with concurrent.futures.ProcessPoolExecutor(64) as executor:
        futures = [executor.submit(process_circuits, task) for task in tasks_args]
        for future in concurrent.futures.as_completed(futures):
            result = future.result()
            results.append(result)

    with open("graph_info/out_graph_info.csv", 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(["Index", "Graph Density", "Graph Edge Count"])
        for i, result in enumerate(results):
            writer.writerow([i, result['graph_density'], result['graph_edge_count']])
    
    