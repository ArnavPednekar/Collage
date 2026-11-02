#include <iostream>
#include <vector>
#include <iomanip>
using namespace std;
#define INF 9999

void display(vector<vector<int>> &A, int n) {
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            if (A[i][j] == INF)
                cout << setw(5) << "INF";
            else
                cout << setw(5) << A[i][j];
        }
        cout << "\n";
    }
}


int main() {
    int n;
    cout << "Enter number of vertices: ";
    cin >> n;

    vector<vector<int>> cost(n, vector<int>(n, INF));
    vector<vector<int>> A(n, vector<int>(n));

    for (int i = 0; i < n; i++)
        cost[i][i] = 0;

    int m;
    cout << "Enter number of edges: ";
    cin >> m;

    cout << "Enter edges as (u v w) with vertices 1.." << n << ":\n";
    for (int e = 0; e < m; e++) {
        int u, v, w;
        cin >> u >> v >> w;
        u--; v--;
        cost[u][v] = w;
    }

    // A0(i,j) = cost[i][j]
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            A[i][j] = cost[i][j];

    cout << "\nInitial Cost Matrix A0:\n";
    display(A, n);

    // A^k(i,j) = min(A^{k-1}(i,j), A^{k-1}(i,k) + A^{k-1}(k,j))
    for (int k = 0; k < n; k++) {
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (A[i][k] != INF && A[k][j] != INF)
                    if (A[i][j] > A[i][k] + A[k][j])
                        A[i][j] = A[i][k] + A[k][j];
            }
        }
        cout << "\nMatrix A" << k + 1 << " after considering vertex " << k + 1 << ":\n";
        display(A, n);
    }

    cout << "\nShortest-Path Cost Matrix:\n";
    display(A, n);

    return 0;
}
