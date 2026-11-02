#include <iostream>
#include <climits>
using namespace std;
int main()
{
    int n = 7;
    int graph[7][7] = {
        {INT_MAX, 28, INT_MAX, INT_MAX, INT_MAX, 10, INT_MAX},
        {28, INT_MAX, 16, INT_MAX, INT_MAX, INT_MAX, 14},
        {INT_MAX, 16, INT_MAX, 12, INT_MAX, INT_MAX, INT_MAX},
        {INT_MAX, INT_MAX, 12, INT_MAX, 22, INT_MAX, 18},
        {INT_MAX, INT_MAX, INT_MAX, 22, INT_MAX, 25, 24},
        {10, INT_MAX, INT_MAX, INT_MAX, 25, INT_MAX, INT_MAX},
        {INT_MAX, 14, INT_MAX, 18, 24, INT_MAX, INT_MAX}};
    int t[7][2];
    int near[7];
    int mincost = 0;
    int k = 0, l = 0;
    int min = INT_MAX;
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            if (graph[i][j] < min)
            {
                min = graph[i][j];
                k = i;
                l = j;
            }
        }
    }
    mincost = graph[k][l];
    t[0][0] = k;
    t[0][1] = l;
    for (int i = 0; i < n; i++)
    {
        if (graph[i][l] < graph[i][k])
            near[i] = l;
        else
            near[i] = k;
    }
    near[k] = -1;
    near[l] = -1;
    for (int i = 1; i < n - 1; i++)
    {
        min = INT_MAX;
        int j = 0;
        for (int v = 0; v < n; v++)
        {
            if (near[v] != -1 && graph[v][near[v]] < min)
            {
                min = graph[v][near[v]];
                j = v;
            }
        }
        t[i][0] = j;
        t[i][1] = near[j];
        mincost += graph[j][near[j]];
        near[j] = -1;
        for (int p = 0; p < n; p++)
        {
            if (near[p] != -1 && graph[p][near[p]] > graph[p][j])
                near[p] = j;
        }
    }
    cout << "Edges in Minimum Cost Spanning Tree:\n";
    for (int i = 0; i < n - 1; i++)
    {
        cout << t[i][0] << " - " << t[i][1] << " : " << graph[t[i][0]][t[i][1]] << "\n";
    }
    cout << "Minimum Cost = " << mincost << "\n";
    return 0;
}
