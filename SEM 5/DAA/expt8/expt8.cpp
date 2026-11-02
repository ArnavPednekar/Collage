#include <iostream>
#include <vector>
using namespace std;

const int INF = 1e9;

struct ForwardResult {
  int minCost;
  vector<int> path;
  vector<int> cost;
  vector<int> stage;
};

ForwardResult forwardMultistage(const vector<vector<int>> &c, int n, int k) {
  vector<int> cost(n + 1, INF), d(n + 1, -1), stage(n + 1, 1);
  for (int j = 2; j <= n; j++) {
    int best = 1;
    for (int r = 1; r < j; r++) {
      if (c[r][j] != INF && stage[r] + 1 > best) {
        best = stage[r] + 1;
      }
    }
    stage[j] = best;
  }
  cost[n] = 0;
  for (int j = n - 1; j >= 1; j--) {
    int best = INF, r = -1;
    for (int nxt = j + 1; nxt <= n; nxt++) {
      if (c[j][nxt] != INF && cost[nxt] != INF &&
          c[j][nxt] + cost[nxt] < best) {
        best = c[j][nxt] + cost[nxt];
        r = nxt;
      }
    }
    if (r != -1) {
      cost[j] = best;
      d[j] = r;
    }
  }

  vector<int> p;
  p.push_back(1);
  while (p.back() != n && d[p.back()] != -1) {
    p.push_back(d[p.back()]);
  }

  ForwardResult res;
  res.minCost = cost[1];
  res.path = p;
  res.cost = cost;
  res.stage = stage;
  return res;
}

struct GraphInput {
  int k, n;
  vector<vector<int>> c;
};

GraphInput getGraphInput() {
  GraphInput g;
  int e;
  cout << "Enter no. of stages: ";
  cin >> g.k;
  cout << "Enter no. of vertices: ";
  cin >> g.n;
  cout << "Enter no. of edges: ";
  cin >> e;

  g.c.assign(g.n + 1, vector<int>(g.n + 1, INF));

  for (int i = 0; i < e; i++) {
    int u, v, w;
    cout << "enter the edge "<<(i + 1) << " of " << e << "\n";
    cout << "enter from: ";
    cin >> u;
    cout << "enter to: ";
    cin >> v;
    cout << "enter cost ("<< u << " -> " << v <<"): ";
    cin >> w;
    g.c[u][v] = w;
  }
  return g;
}

void printGraph(const GraphInput &g) {
  cout << "Stages k: " << g.k << "\nVertices n: " << g.n << "\nEdges:\n";
  for (int u = 1; u <= g.n; u++) {
    for (int v = 1; v <= g.n; v++) {
      if (g.c[u][v] != INF) {
        cout << u << " -> " << v << " cost " << g.c[u][v] << "\n";
      }
    }
  }
}

ForwardResult backwardMultistage(const vector<vector<int>> &c, int n, int k) {
  vector<int> bcost(n + 1, INF), d(n + 1, -1), stage(n + 1, 1);
  for (int j = 2; j <= n; j++) {
    int best = 1;
    for (int r = 1; r < j; r++) {
      if (c[r][j] != INF && stage[r] + 1 > best) {
        best = stage[r] + 1;
      }
    }
    stage[j] = best;
  }
  bcost[1] = 0;
  for (int j = 2; j <= n; j++) {
    int best = INF, r = -1;
    for (int pre = 1; pre < j; pre++) {
      if (c[pre][j] != INF && bcost[pre] != INF &&
          bcost[pre] + c[pre][j] < best) {
        best = bcost[pre] + c[pre][j];
        r = pre;
      }
    }
    if (r != -1) {
      bcost[j] = best;
      d[j] = r;
    }
  }

  vector<int> p(k + 1, -1);
  p[1] = 1;
  p[k] = n;
  for (int j = k - 1; j >= 2; j--) {
    p[j] = d[p[j + 1]];
  }
  vector<int> path;
  for (int j = 1; j <= k; j++) {
    if (p[j] != -1)
      path.push_back(p[j]);
  }

  ForwardResult res;
  res.minCost = bcost[n];
  res.path = path;
  res.cost = bcost;
  res.stage = stage;
  return res;
}

void printResult(const ForwardResult &ans) {
  for (size_t j = 1; j < ans.cost.size(); j++) {
    cout << "cost(" << ans.stage[j] << "," << j << ") = " << ans.cost[j] << "\n";
  }
  cout << "Minimum cost: " << ans.minCost << "\nPath: ";
  for (size_t i = 0; i < ans.path.size(); i++) {
    if (i)
      cout << " -> ";
    cout << ans.path[i];
  }
  cout << endl;
}

void printBackwardResult(const ForwardResult &ans) {
  for (size_t j = 1; j < ans.cost.size(); j++) {
    cout << "bcost(" << ans.stage[j] << "," << j << ") = " << ans.cost[j] << "\n";
  }
  cout << "Minimum cost: " << ans.minCost << "\nPath: ";
  for (size_t i = 0; i < ans.path.size(); i++) {
    if (i)
      cout << " -> ";
    cout << ans.path[i];
  }
  cout << endl;
}

GraphInput getDefaultGraph() {
  GraphInput g;
  g.k = 4;
  g.n = 5;
  g.c.assign(g.n + 1, vector<int>(g.n + 1, INF));
  vector<vector<int>> edges = {
    {1, 2, 4},
    {1, 3, 2},
    {2, 4, 5},
    {3, 4, 1},
    {4, 5, 3},
    {2, 5, 9},
  };
  for (auto &e : edges) {
    g.c[e[0]][e[1]] = e[2];
  }
  return g;
}

int main() {
  GraphInput g = getGraphInput();
  printGraph(g);

  cout << "\n--- Forward Approach (source to sink via cost) ---\n";
  ForwardResult fwd = forwardMultistage(g.c, g.n, g.k);
  printResult(fwd);

  cout << "\n--- Backward Approach (source to vertex via bcost) ---\n";
  ForwardResult bwd = backwardMultistage(g.c, g.n, g.k);
  printBackwardResult(bwd);
  return 0;
}
