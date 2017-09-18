#!/usr/bin/env python3

def read_graph(f):
    num_nodes = int(f.readline())
    nodes = []
    for _ in range(num_nodes):
        nodes.append([])

    for i in range(num_nodes):
        num_labels = int(f.readline())
        labels = []
        for _ in range(num_labels):
            labels.append(int(f.readline()))
        nodes[i] = [labels]

        num_edges = int(f.readline())
        for _ in range(num_edges):
            edge = f.readline().split()
            nodes[i].append((edge[0], int(edge[1]), edge[2]))

    return nodes

# Returns a list of tuples of (node, port, node, port)
def reformat_graph_edges(g):
    edges = []
    for i in range(len(g)):
        for (e_sport, e_dnode, e_dport) in g[i][1:]:
            edges.append((i, e_sport, e_dnode, e_dport))
    return edges

# Returns a list of list of numbers
def reformat_graph_labels(g):
    labels = []
    for g_ in g:
        labels.append(g_[0])
    return labels

with open("testtest_graph.txt", "r") as inf:
    dgraph = read_graph(inf)
    ngraph = read_graph(inf)
    # print(dgraph)
    # print(ngraph)
    dgraph_e = reformat_graph_edges(dgraph)
    ngraph_e = reformat_graph_edges(ngraph)
    # print(dgraph_e)
    # print(ngraph_e)
    dgraph_l = reformat_graph_labels(dgraph)
    ngraph_l = reformat_graph_labels(ngraph)
    print(dgraph_l)
    print(ngraph_l)
