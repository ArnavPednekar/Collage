#include <iostream>
#include <chrono>

using namespace std;

int** create(int n) {
    int** m = new int*[n];
    for(int i = 0; i < n; i++) m[i] = new int[n]();
    return m;
}

void remove(int** m, int n) {
    for(int i = 0; i < n; i++) delete[] m[i];
    delete[] m;
}

void print(int** m, int n) {
    for(int i = 0; i < n; i++) {
        for(int j = 0; j < n; j++) {
            cout << m[i][j] << " ";
        }
        cout << "\n";
    }
}

void standard(int** a, int** b, int** c, int n) {
    for(int i = 0; i < n; i++) {
        for(int j = 0; j < n; j++) {
            c[i][j] = 0;
            for(int k = 0; k < n; k++) {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

void strassen(int** a, int** b, int** c, int n) {
    if(n <= 1) {
        standard(a, b, c, n);
        return;
    }

    int k = n / 2;
    int** a11 = create(k); int** a12 = create(k); int** a21 = create(k); int** a22 = create(k);
    int** b11 = create(k); int** b12 = create(k); int** b21 = create(k); int** b22 = create(k);
    int** c11 = create(k); int** c12 = create(k); int** c21 = create(k); int** c22 = create(k);
    int** m1 = create(k); int** m2 = create(k); int** m3 = create(k); int** m4 = create(k);
    int** m5 = create(k); int** m6 = create(k); int** m7 = create(k);
    int** t1 = create(k); int** t2 = create(k);

    for(int i = 0; i < k; i++) {
        for(int j = 0; j < k; j++) {
            a11[i][j] = a[i][j];
            a12[i][j] = a[i][j + k];
            a21[i][j] = a[i + k][j];
            a22[i][j] = a[i + k][j + k];

            b11[i][j] = b[i][j];
            b12[i][j] = b[i][j + k];
            b21[i][j] = b[i + k][j];
            b22[i][j] = b[i + k][j + k];
        }
    }

    for(int i = 0; i < k; i++) for(int j = 0; j < k; j++) { t1[i][j] = a11[i][j] + a22[i][j]; t2[i][j] = b11[i][j] + b22[i][j]; }
    strassen(t1, t2, m1, k);

    for(int i = 0; i < k; i++) for(int j = 0; j < k; j++) { t1[i][j] = a21[i][j] + a22[i][j]; }
    strassen(t1, b11, m2, k);

    for(int i = 0; i < k; i++) for(int j = 0; j < k; j++) { t2[i][j] = b12[i][j] - b22[i][j]; }
    strassen(a11, t2, m3, k);

    for(int i = 0; i < k; i++) for(int j = 0; j < k; j++) { t2[i][j] = b21[i][j] - b11[i][j]; }
    strassen(a22, t2, m4, k);

    for(int i = 0; i < k; i++) for(int j = 0; j < k; j++) { t1[i][j] = a11[i][j] + a12[i][j]; }
    strassen(t1, b22, m5, k);

    for(int i = 0; i < k; i++) for(int j = 0; j < k; j++) { t1[i][j] = a21[i][j] - a11[i][j]; t2[i][j] = b11[i][j] + b12[i][j]; }
    strassen(t1, t2, m6, k);

    for(int i = 0; i < k; i++) for(int j = 0; j < k; j++) { t1[i][j] = a12[i][j] - a22[i][j]; t2[i][j] = b21[i][j] + b22[i][j]; }
    strassen(t1, t2, m7, k);

    for(int i = 0; i < k; i++) {
        for(int j = 0; j < k; j++) {
            c11[i][j] = m1[i][j] + m4[i][j] - m5[i][j] + m7[i][j];
            c12[i][j] = m3[i][j] + m5[i][j];
            c21[i][j] = m2[i][j] + m4[i][j];
            c22[i][j] = m1[i][j] - m2[i][j] + m3[i][j] + m6[i][j];
        }
    }

    for(int i = 0; i < k; i++) {
        for(int j = 0; j < k; j++) {
            c[i][j] = c11[i][j];
            c[i][j + k] = c12[i][j];
            c[i + k][j] = c21[i][j];
            c[i + k][j + k] = c22[i][j];
        }
    }

    remove(a11, k); remove(a12, k); remove(a21, k); remove(a22, k);
    remove(b11, k); remove(b12, k); remove(b21, k); remove(b22, k);
    remove(c11, k); remove(c12, k); remove(c21, k); remove(c22, k);
    remove(m1, k); remove(m2, k); remove(m3, k); remove(m4, k);
    remove(m5, k); remove(m6, k); remove(m7, k);
    remove(t1, k); remove(t2, k);
}

int main() {
    int n;
    cout << "Enter matrix size (N): ";
    if (!(cin >> n) || n <= 0) return 0;

    int** a = create(n);
    int** b = create(n);
    int** c1 = create(n);
    int** c2 = create(n);

    cout << "Enter elements of matrix A:\n";
    for(int i = 0; i < n; i++) {
        for(int j = 0; j < n; j++) {
            cin >> a[i][j];
        }
    }

    cout << "Enter elements of matrix B:\n";
    for(int i = 0; i < n; i++) {
        for(int j = 0; j < n; j++) {
            cin >> b[i][j];
        }
    }

    auto start = chrono::high_resolution_clock::now();
    standard(a, b, c1, n);
    auto end = chrono::high_resolution_clock::now();
    cout << "Standard Multiplication Time: " << chrono::duration<double, milli>(end - start).count() << " ms\n";

    start = chrono::high_resolution_clock::now();
    strassen(a, b, c2, n);
    end = chrono::high_resolution_clock::now();
    cout << "Strassen Multiplication Time: " << chrono::duration<double, milli>(end - start).count() << " ms\n";

    cout << "Result Matrix (Standard):\n";
    print(c1, n);

    cout << "Result Matrix (Strassen):\n";
    print(c2, n);

    remove(a, n);
    remove(b, n);
    remove(c1, n);
    remove(c2, n);

    return 0;
}
