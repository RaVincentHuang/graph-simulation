import networkx as nx
import random

from networkx import DiGraph

def random_graph_gen(n, p, k):
    G: DiGraph = nx.fast_gnp_random_graph(n, p, directed=True)
    for node in G.nodes:
        G.nodes[node]['label'] = random.randint(1, k)
    return G

def graph_permutation(g: DiGraph) -> DiGraph:
    nodes = list(g.nodes)
    random.shuffle(nodes)
    mapping = dict(zip(g.nodes, nodes))
    g_perm = nx.relabel_nodes(g, mapping)
    return g_perm

def save_graph(g: DiGraph, filename: str):
    nx.write_edgelist(g, filename, data=False)
    
def dump_graph(g: DiGraph, k) -> str:
    s = f"{g.number_of_nodes()} {g.number_of_edges()} {k}\n"
    for node in g.nodes:
        s += f"{node} {g.nodes[node]['label']}\n"
    for u, v in g.edges:
        s += f"{u} {v}\n"
    return s
    
if __name__ == '__main__':
    N = 100
    for i in range(N):
        
        size = random.randint(8, 80)
        p = random.uniform(0.1, 0.5)
        k = random.randint(4, size // 2)
        g1 = random_graph_gen(size, p, k)
        if random.random() < 0.5:
            g2 = graph_permutation(g1)
            s1, s2 = dump_graph(g1, k), dump_graph(g2, k)
            s = f"t\n{s1}{s2}\n"
        else:
            g2 = random_graph_gen(size, p, k)
            s1, s2 = dump_graph(g1, k), dump_graph(g2, k)
            s = f"f\n{s1}{s2}\n"
        
        with open(f"data/label_graph/simulation_test/iso_{i}", 'w') as f:
            f.write(s)
            
