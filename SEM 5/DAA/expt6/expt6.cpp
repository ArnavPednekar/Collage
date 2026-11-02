#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

struct Edge {
    int cost, u, v;
};

int parentArr[8];

int findParent(int x) {
    if (parentArr[x] == x)
        return x;
    return parentArr[x] = findParent(parentArr[x]);
}

void unionSet(int a, int b) {
    parentArr[findParent(a)] = findParent(b);
}

int main() {
    int n = 7;
    vector<Edge> edges = {
        {10, 1, 6},
        {28, 1, 2},
        {14, 2, 7},
        {16, 2, 3},
        {25, 5, 6},
        {24, 5, 7},
        {18, 4, 7},
        {12, 3, 4},
        {22, 4, 5}
    };

    sort(edges.begin(), edges.end(), [](Edge a, Edge b) {
        return a.cost < b.cost;
    });

    for (int i = 1; i <= n; i++)
        parentArr[i] = i;

    int mincost = 0;
    cout << "Edges in Minimum Cost Spanning Tree:\n";
    int count = 0;
    for (size_t i = 0; i < edges.size() && count < n - 1; i++) {
        int u = edges[i].u, v = edges[i].v, w = edges[i].cost;
        if (findParent(u) != findParent(v)) {
            unionSet(u, v);
            cout << u << " - " << v << " : " << w << "\n";
            mincost += w;
            count++;
        }
    }
    cout << "Minimum Cost = " << mincost << "\n";
    return 0;
}
