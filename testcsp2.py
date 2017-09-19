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

def check_partial_assignment(dgraph_e, ngraph_e, dgraph_n, ngraph_n, assignment):
    dgraph_e_ = set(dgraph_e)

    # No repeats
    count = set()
    for x in assignment:
        if x != -1:
            if x in count:
                # print("dup")
                return False
            count.add(x)

    for srcnode_n_idx, srcport, dstnode_n_idx, dstport in ngraph_e:
        if assignment[srcnode_n_idx] != -1 and assignment[dstnode_n_idx] != -1:
            # Nodes are linked by an edge in the netlist
            srcnode_d_idx = assignment[srcnode_n_idx]
            dstnode_d_idx = assignment[dstnode_n_idx]
            if (srcnode_d_idx, srcport, dstnode_d_idx, dstport) not in dgraph_e_:
                return False

    return True

def check_one_edge(dgraph_e, ngraph_e, dgraph_n, ngraph_n, assignment, edge):
    dgraph_e_ = set(dgraph_e)
    srcnode_n_idx, srcport, dstnode_n_idx, dstport = edge
    if assignment[srcnode_n_idx] != -1 and assignment[dstnode_n_idx] != -1:
        # Nodes are linked by an edge in the netlist
        srcnode_d_idx = assignment[srcnode_n_idx]
        dstnode_d_idx = assignment[dstnode_n_idx]
        if (srcnode_d_idx, srcport, dstnode_d_idx, dstport) not in dgraph_e_:
            return False

    return True

def ac3(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment):
    new_domains = [set(x) for x in domains]

    # Set up all arcs
    q = []

    # def add_neighbors_to_queue(e1_):
    #     e1 = ngraph_e[e1_]
    #     for e2_ in ngraph_n[e1[0]][0] + ngraph_n[e1[0]][1]:
    #         e2 = ngraph_e[e2_]
    #         if e1 == e2:
    #             continue
    #         q.append((e2_, e2, e1_, e1, e1[0]))
    #     for e2_ in ngraph_n[e1[2]][0] + ngraph_n[e1[2]][1]:
    #         e2 = ngraph_e[e2_]
    #         if e1 == e2:
    #             continue
    #         q.append((e2_, e2, e1_, e1, e1[2]))

    # for e1_ in range(len(ngraph_e)):
    #     # add_neighbors_to_queue(e1_)

    for e in ngraph_e:
        q.append((e[0], e[2], e))
        q.append((e[2], e[0], e))

    # print(q)

    while len(q):
        tail_node_idx, head_node_idx, involved_edge = q[0]
        q = q[1:]

        to_remove = set()
        for x in new_domains[tail_node_idx]:
            any_ok = False
            for y in new_domains[head_node_idx]:
                new_assignment = list(assignment)
                new_assignment[tail_node_idx] = x
                new_assignment[head_node_idx] = y
                any_ok = check_one_edge(dgraph_e, ngraph_e, dgraph_n, ngraph_n, new_assignment, involved_edge)
                if any_ok:
                    break
            if not any_ok:
                to_remove.add(x)

        if to_remove:
            # print("rm", tail_node_idx, to_remove)
            for x in to_remove:
                new_domains[tail_node_idx].remove(x)
            # Add any edges touching this node
            tail_node = ngraph_n[tail_node_idx]
            for e_ in tail_node[0]:
                e = ngraph_e[e_]
                q.append((e[0], tail_node_idx, e))
            for e_ in tail_node[1]:
                e = ngraph_e[e_]
                q.append((e[2], tail_node_idx, e))
            # print(q)

    # print(domains)
    print(new_domains)

    return new_domains

def fc(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment, selected_var):
    new_domains = [set(x) for x in domains]

    # Neighbors
    q = []
    tail_node = ngraph_n[selected_var]
    for e_ in tail_node[0]:
        e = ngraph_e[e_]
        q.append((e[0], e))
    for e_ in tail_node[1]:
        e = ngraph_e[e_]
        q.append((e[2], e))

    while len(q):
        node_idx, involved_edge = q[0]
        q = q[1:]

        to_remove = set()
        any_ok = False
        for y in new_domains[node_idx]:
            new_assignment = list(assignment)
            new_assignment[node_idx] = y
            if not check_one_edge(dgraph_e, ngraph_e, dgraph_n, ngraph_n, new_assignment, involved_edge):
                to_remove.add(y)

        if to_remove:
            # print("rm", tail_node_idx, to_remove)
            for x in to_remove:
                new_domains[node_idx].remove(x)

    return new_domains

def backtrack(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment):
    # print("**********")

    if -1 not in assignment:
        assert check_partial_assignment(dgraph_e, ngraph_e, dgraph_n, ngraph_n, assignment)
        print("SUCCESS")
        print(assignment)
        asdf
        return assignment

    # Variable selection choice point
    selected_var = -1
    selected_var_min_remaining_vals = float('inf')
    for i in range(len(assignment)):
        # print("*****")
        # print(i)
        # print(assignment[i])
        if assignment[i] == -1:
            # print(selected_var_min_remaining_vals)
            # print(len(domains[i]))
            if len(domains[i]) < selected_var_min_remaining_vals:
                # print("XXX")
                selected_var_min_remaining_vals = len(domains[i])
                selected_var = i

    # print("#####")
    # print(selected_var)
    # print(len(domains[selected_var]))

    # Ordering choice point
    for choice in sorted(domains[selected_var]):
        assignment[selected_var] = choice
        # print(selected_var)
        print(assignment)
        if not check_partial_assignment(dgraph_e, ngraph_e, dgraph_n, ngraph_n, assignment):
            assignment[selected_var] = -1
            # print("Z")
            continue
        # new_domains = ac3(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment)
        # new_domains = fc(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment, selected_var)
        new_domains = domains
        ret = backtrack(dgraph_e, ngraph_e, dgraph_n, ngraph_n, new_domains, assignment)
        assignment[selected_var] = -1

        # if ret is not None:
        #     return ret

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

    domains = [None] * len(ngraph_n)
    for i in range(len(ngraph_n)):
        domains[i] = set()
        for j in range(len(dgraph_n)):
            # Pre-filter the domains using the labels

            assert len(ngraph_l[i]) == 1

            # The node labels have to match
            if (ngraph_l[i][0] in dgraph_l[j]):
                domains[i].add(j)

    print(domains)
    backtrack(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, [-1] * len(ngraph_n))
