#include <iostream>
#include <vector>
#include <cmath>
using namespace std;

vector<int> x;
int n, countSol = 0;

bool Place(int k, int i) {
    for (int j = 0; j < k; j++) {
        if (x[j] == i || abs(x[j] - i) == abs(j - k))
            return false;
    }
    return true;
}

void displaySolution() {
    countSol++;
    cout << "\nSolution " << countSol << ": x = [ ";
    for (int i = 0; i < n; i++)
        cout << x[i] + 1 << " ";
    cout << "]\n";
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            if (x[i] == j)
                cout << " Q ";
            else
                cout << " . ";
        }
        cout << "\n";
    }
}

void NQueens(int k) {
    for (int i = 0; i < n; i++) {
        if (Place(k, i)) {
            x[k] = i;
            if (k == n - 1)
                displaySolution();
            else
                NQueens(k + 1);
        }
    }
}

int main() {
    cout << "Enter value of N: ";
    cin >> n;

    if (n <= 0) {
        cout << "N must be positive.\n";
        return 0;
    }

    x.assign(n, -1);
    countSol = 0;

    NQueens(0);

    if (countSol == 0)
        cout << "\nNo solution exists for N = " << n << ".\n";
    else
        cout << "\nTotal solutions for N = " << n << ": " << countSol << "\n";

    return 0;
}
