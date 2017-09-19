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
    # No repeats
    count = set()
    for x in assignment:
        if x != -1:
            if x in count:
                # print("dup")
                return False
            count.add(x)

    # Sharing the node correctly
    for (nn_in, nn_out) in ngraph_n:
        # print("nn", nn_in, nn_out)
        d_in = []
        for x in nn_in:
            if assignment[x] != -1:
                d_in.append(dgraph_e[assignment[x]])
        d_out = []
        for x in nn_out:
            if assignment[x] != -1:
                d_out.append(dgraph_e[assignment[x]])
        # print(d_in, d_out)

        # Now are these sharing the same node?
        # print("d", d_in, d_out)
        dn = None
        for x in d_in:
            if dn is None:
                dn = x[2]
            if dn != x[2]:
                # print("in")
                return False
        for x in d_out:
            if dn is None:
                dn = x[0]
            if dn != x[0]:
                # print("out")
                return False
    # print(assignment)
    # print("true")
    return True

def ac3(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment):
    new_domains = [set(x) for x in domains]

    # Set up all arcs
    q = []

    def add_neighbors_to_queue(e1_):
        e1 = ngraph_e[e1_]
        for e2_ in ngraph_n[e1[0]][0] + ngraph_n[e1[0]][1]:
            e2 = ngraph_e[e2_]
            if e1 == e2:
                continue
            q.append((e2_, e2, e1_, e1, e1[0]))
        for e2_ in ngraph_n[e1[2]][0] + ngraph_n[e1[2]][1]:
            e2 = ngraph_e[e2_]
            if e1 == e2:
                continue
            q.append((e2_, e2, e1_, e1, e1[2]))

    for e1_ in range(len(ngraph_e)):
        add_neighbors_to_queue(e1_)

    # print(q)

    while len(q):
        tail_idx, tail_obj, head_idx, head_obj, constraint_node = q[0]
        q = q[1:]

        to_remove = set()
        for x in new_domains[tail_idx]:
            any_ok = False
            for y in new_domains[head_idx]:
                new_assignment = list(assignment)
                new_assignment[head_idx] = y
                any_ok = check_partial_assignment(dgraph_e, ngraph_e, dgraph_n, ngraph_n, new_assignment)
                if any_ok:
                    break
            if not any_ok:
                to_remove.add(x)

        if to_remove:
            print("rm", tail_idx, to_remove)
            for x in to_remove:
                new_domains[tail_idx].remove(x)
            add_neighbors_to_queue(tail_idx)

    # print(domains)
    # print(new_domains)

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
        new_domains = ac3(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, assignment)
        # new_domains = domains
        backtrack(dgraph_e, ngraph_e, dgraph_n, ngraph_n, new_domains, assignment)
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
        domains[i] = set()
        for j in range(len(dgraph_e)):
            # Pre-filter the domains using the labels
            ngraph_src_node = ngraph_e[i][0]
            ngraph_dst_node = ngraph_e[i][2]
            dgraph_src_node = dgraph_e[j][0]
            dgraph_dst_node = dgraph_e[j][2]

            assert len(ngraph_l[ngraph_src_node]) == 1
            assert len(ngraph_l[ngraph_dst_node]) == 1

            # The node labels have to match
            if (ngraph_l[ngraph_src_node][0] in dgraph_l[dgraph_src_node] and
                ngraph_l[ngraph_dst_node][0] in dgraph_l[dgraph_dst_node]):

                # print(ngraph_src_node, ngraph_dst_node, dgraph_src_node, dgraph_dst_node)
                # print(ngraph_l[ngraph_src_node], ngraph_l[ngraph_dst_node], dgraph_l[dgraph_src_node], dgraph_l[dgraph_dst_node])
                # print(ngraph_e[i][1], dgraph_e[j][1], ngraph_e[i][3], dgraph_e[j][3])

                # The port labels have to match as well
                if (ngraph_e[i][1] == dgraph_e[j][1] and ngraph_e[i][3] == dgraph_e[j][3]):
                    domains[i].add(j)

    # print(domains)
    backtrack(dgraph_e, ngraph_e, dgraph_n, ngraph_n, domains, [-1] * len(ngraph_e))
