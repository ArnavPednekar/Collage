#include <iostream>
#include <vector>
using namespace std;

vector<vector<int>> G;
vector<int> x;
int n, m, countSol = 0;

void NextValue(int k) {
    do {
        x[k] = (x[k] + 1) % (m + 1);
        if (x[k] == 0) return;
        int j;
        for (j = 0; j < n; j++) {
            if (G[k][j] != 0 && x[k] == x[j]) break;
        }
        if (j == n) return;
    } while (true);
}

void mColoring(int k) {
    do {
        NextValue(k);
        if (x[k] == 0) return;
        if (k == n - 1) {
            countSol++;
            cout << "\nColoring " << countSol << ": [";
            for (int i = 0; i < n; i++) cout << " " << x[i];
            cout << " ]";
        } else {
            mColoring(k + 1);
        }
    } while (true);
}

int main() {
    cout << "Enter number of vertices: ";
    cin >> n;

    G.assign(n, vector<int>(n, 0));
    cout << "Enter the adjacency matrix:\n";
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            cin >> G[i][j];

    cout << "Enter maximum number of colors: ";
    cin >> m;

    x.assign(n, 0);
    countSol = 0;

    mColoring(0);

    if (countSol == 0)
        cout << "\nNo valid coloring exists with " << m << " colors.\n";
    else
        cout << "\nTotal valid colorings: " << countSol << "\n";

    return 0;
}