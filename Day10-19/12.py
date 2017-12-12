from collections import defaultdict


def parse_graph(edges):
    graph = defaultdict(list)
    for e in edges:
        vertices = e.split(' ')
        for v in vertices[2:]:
            graph[vertices[0]].append(v.replace(',', '').strip())
    return graph


def bfs_reach(graph, start):
    to_visit = [start]
    reachable = set([start])
    while to_visit:
        curr = to_visit.pop()
        for neighbor in graph[curr]:
            if neighbor not in reachable:
                reachable.add(neighbor)
                to_visit.append(neighbor)
    return reachable


def num_connected_components(graph):
    unmarked = set(graph.keys())
    num_components = 0
    while unmarked:
        component = bfs_reach(graph, unmarked.pop())
        unmarked -= component
        num_components += 1
    return num_components


if __name__ == '__main__':
    with open('12.txt') as f:
        edges = f.readlines()
    graph = parse_graph(edges)
    part1 = bfs_reach(graph, '0')
    print("Part 1: {}".format(len(part1)))
    part2 = num_connected_components(graph)
    print("Part 2: {}".format(part2))
