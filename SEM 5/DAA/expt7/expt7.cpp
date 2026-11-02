#include<iostream>
#include <vector>

using namespace std;
const int INF = 1e9;

vector<int> dijkstra(vector<vector<int>> &graph, int src){
    int n = graph.size();
    vector<int> dist(n,INF);
    vector<bool> vis(n, false);

    dist[src] = 0;

    for (int count= 0; count < n; count++) {
        int u = -1, best = INF;
        for(int i =0;i<n;i++){
            if(!vis[i]&& dist[i]<best){
                best = dist[i];
                u = i;
            }
        }
        if(u == -1) break;

        vis[u] = true;

        for(int v = 0; v<n;v++){
            int w = graph[u][v];
            if(!vis[v]&& w != 0 && dist[u] != INF){
                if(dist[u] + w < dist[v]){
                    dist[v] = dist[u] + w;
                }
            }
        }
    }
    return dist;
}

int main(){
    int n;
    cout << "Enter number of vertices: ";
    cin >> n;

    vector<vector<int>> graph(n, vector<int>(n, 0));
    vector<vector<int>> edgeList;

    int m;
    cout << "Enter number of edges: ";
    cin >> m;

    cout << "Example: to add 1-2 = 4, enter source=1, destination=2, weight=4.\n";
    for (int i = 0; i < m; i++) {
        int u, v, w;
        cout << "\nEdge " << i+1 << " of " << m << ":\n";
        cout << "  Source: ";
        cin >> u;
        cout << "  Destination: ";
        cin >> v;
        cout << "  Weight: ";
        cin >> w;
        u--; v--;
        if (u < 0 || u >= n || v < 0 || v >= n || w < 0) {
            cout << "Invalid edge! Try again.\n";
            i--;
            continue;
        }
        graph[u][v] = w;
        graph[v][u] = w;
        edgeList.push_back({u, v, w});
    }

    int src;
    cout << "Enter source vertex: ";
    cin >> src;
    src--;

    if (src < 0 || src >= n){
        cout << "Invalid source vertex!";
        return 1;
    }

    vector<int> dist = dijkstra(graph, src);

    cout << "\nAll edges (weights):\n";
    for (size_t i = 0; i < edgeList.size(); i++) {
        cout << edgeList[i][0]+1 << "-" << edgeList[i][1]+1
             << " = " << edgeList[i][2] << "\n";
    }

    cout << "\nShortest distances from vertex " << src+1 << ":\n";
    for (int i = 0; i < n; i++) {
        cout << src+1 << "-" << i+1 << " = ";
        if (dist[i] == INF)
            cout << "No path";
        else
            cout << dist[i];
        cout << "\n";
    }
    return 0;
}
