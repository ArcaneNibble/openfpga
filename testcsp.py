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

# Returns a list of tuples of lists [([in, in, in], [out, out, out]), ...]
def reformat_graph_nodes(e, num_nodes):
    nodes = [None] * num_nodes
    for i in range(num_nodes):
        nodes[i] = ([], [])

    for i, e_ in enumerate(e):
        nodes[e_[0]][1].append(i)
        nodes[e_[2]][0].append(i)

    return nodes

def backtrack(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment):
    if -1 not in assignment:
        print(assignment)
        return assignment

    # Variable selection choice point
    selected_var = assignment.index(-1)

    # Ordering choice point
    for choice in domains[selected_var]:
        assignment[selected_var] = choice
        # TODO: Check legality
        backtrack(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment)
        assignment[selected_var] = -1

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
    # print(dgraph_l)
    # print(ngraph_l)
    dgraph_n = reformat_graph_nodes(dgraph_e, len(dgraph))
    ngraph_n = reformat_graph_nodes(ngraph_e, len(ngraph))
    # print(dgraph_n)
    # print(ngraph_n)

    domains = [None] * len(ngraph_e)
    for i in range(len(ngraph_e)):
        domains[i] = []
        for j in range(len(dgraph_e)):
            # Pre-filter the domains using the labels
            ngraph_src_node = ngraph_e[i][0]
            ngraph_dst_node = ngraph_e[i][2]
            dgraph_src_node = dgraph_e[j][0]
            dgraph_dst_node = dgraph_e[j][2]

            # print(ngraph_src_node, ngraph_dst_node, dgraph_src_node, dgraph_dst_node)
            # print(ngraph_l[ngraph_src_node], ngraph_l[ngraph_dst_node], dgraph_l[dgraph_src_node], dgraph_l[dgraph_dst_node])

            assert len(ngraph_l[ngraph_src_node]) == 1
            assert len(ngraph_l[ngraph_dst_node]) == 1

            if (ngraph_l[ngraph_src_node][0] in dgraph_l[dgraph_src_node] and
                ngraph_l[ngraph_dst_node][0] in dgraph_l[dgraph_dst_node]):
                domains[i].append(j)

    # print(domains)
    backtrack(dgraph_e, ngraph_e, dgraph_l, ngraph_l, dgraph_n, ngraph_n, domains, [-1] * len(ngraph_e))
