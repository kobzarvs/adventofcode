from dataclasses import dataclass
from numpy import inf

graph = {(0, 0): {'visited': [], 'neighbors': [(0, 1), (1, 0)]},
    (0, 1): {'visited': [], 'neighbors': [(0, 2), (1, 1), (0, 0)]},
    (0, 2): {'visited': [], 'neighbors': [(0, 1), (1, 2), (0, 3)]}, (0, 3): {'visited': [], 'neighbors': [(0, 2)]},
    (1, 0): {'visited': [], 'neighbors': [(0, 0), (1, 1), (2, 0)]},
    (1, 1): {'visited': [], 'neighbors': [(1, 0), (0, 1), (2, 1), (1, 2)]},
    (1, 2): {'visited': [], 'neighbors': [(1, 1), (0, 2), (2, 2), (1, 3)]},
    (1, 3): {'visited': [], 'neighbors': [(1, 2), (0, 3), (2, 3)]},
    (2, 0): {'visited': [], 'neighbors': [(1, 0), (2, 1), (3, 0)]},
    (2, 1): {'visited': [], 'neighbors': [(2, 0), (1, 1), (3, 1), (2, 2)]},
    (2, 2): {'visited': [], 'neighbors': [(2, 1), (1, 2), (3, 2), (2, 3)]},
    (2, 3): {'visited': [], 'neighbors': [(2, 2), (1, 3), (3, 3)]},
    (3, 0): {'visited': [], 'neighbors': [(2, 0), (3, 1)]},
    (3, 1): {'visited': [], 'neighbors': [(3, 0), (2, 1), (3, 2)]},
    (3, 2): {'visited': [], 'neighbors': [(3, 1), (2, 2), (3, 3)]},
    (3, 3): {'visited': [], 'neighbors': [(3, 2), (2, 3)]}}


@dataclass
class Graph:
    pos: tuple
    visited: list
    neighbors: list
    cost: int
    parent: tuple


# costs = {
#     (0,0): 0, (0,1): inf, (0,2): inf, (0,3): inf, 
#     (1,0): inf, (1,1): inf, (1,2): inf, (1,3): inf,
#     (2,0): inf, (2,1): inf, (2,2): inf, (2,3): inf,
#     (3,0): inf, (3,1): inf, (3,2): inf, (3,3): inf}
costs = {(0, 0): 0, (0, 1): -inf, (0, 2): -inf, (0, 3): -inf, (1, 0): -inf, (1, 1): -inf, (1, 2): -inf, (1, 3): -inf,
    (2, 0): -inf, (2, 1): -inf, (2, 2): -inf, (2, 3): -inf, (3, 0): -inf, (3, 1): -inf, (3, 2): -inf, (3, 3): -inf}

index = {}

for node in graph:
    index[node] = Graph(node, visited=[], neighbors=graph[node]['neighbors'], cost=1, parent=None)

print(index)

parents = {}
visited = []


def search(source, target, graph, costs, parents):
    next_node = source
    while next_node != target:
        for neighbor in graph[next_node]['neighbors']:
            if neighbor in graph[next_node]['visited']:
                print('visited', next_node, neighbor)
                continue
            print('cost', next_node, neighbor, graph[next_node]['neighbors'][neighbor])
            if graph[next_node]['neighbors'][neighbor] + costs[next_node] < costs[neighbor]:
                costs[neighbor] = graph[next_node]['neighbors'][neighbor] + costs[next_node]
                parents[neighbor] = next_node
            # del graph[neighbor]['neighbors'][next_node]
            # if next_node in graph[neighbor]:
            graph[neighbor]['visited'].append(next_node)
        del costs[next_node]
        next_node = min(costs, key=costs.get)  # print('next_node', next_node)
    return parents


def search_2(source, target, index, costs, parents):
    nextNode = index[source]
    target = index[target]
    while nextNode != target:
        for neighbor in nextNode.neighbors:
            n = index[neighbor]
            if neighbor in nextNode.visited:
                print('visited', nextNode, neighbor)
                continue
            if neighbor in costs and n.cost + nextNode.cost > costs[neighbor]:
                costs[neighbor] = n.cost + nextNode.cost
                parents[neighbor] = nextNode.pos
                n.parent = nextNode.pos
            # del graph[neighbor]['neighbors'][nextNode]
            # if nextNode in graph[neighbor]:
            n.visited.append(nextNode.pos)
        del costs[nextNode.pos]
        nextNode = index[max(costs, key=costs.get)]  # print('nextNode', nextNode)
    return parents


def backpedal(source, target, searchResult):
    node = target
    backpath = [target]
    while node != source:
        backpath.append(searchResult[node])
        node = searchResult[node]
    return list(reversed(backpath))


start, end = (0, 0), (3, 3)
result = search_2(start, end, index, costs, parents)

print('parent dictionary={}'.format(result))
print('longest path={}'.format(backpedal(start, end, result)))
print(costs)

for i in index:
    print(index[i])
