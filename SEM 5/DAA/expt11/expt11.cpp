#include <iostream>
#include <stack>
#include <vector>
#include <algorithm>
using namespace std;
struct State {
    int s;
    int k;
    int r;
    vector<int> x;
};
int main() {
    int n, m;
    cout << "Enter number of elements: ";
    cin >> n;
    vector<int> w(n);
    cout << "Enter weights: ";
    for (int i = 0; i < n; i++) cin >> w[i];
    cout << "Enter target sum: ";
    cin >> m;
    sort(w.begin(), w.end());
    int total = 0;
    for (int i = 0; i < n; i++) total += w[i];
    stack<State> st;
    st.push({0, 0, total, vector<int>(n, 0)});
    bool found = false;
    while (!st.empty()) {
        State cur = st.top();
        st.pop();
        int s = cur.s;
        int k = cur.k;
        int r = cur.r;
        vector<int> x = cur.x;
        if (k >= n) continue;
        vector<int> inc = x;
        inc[k] = 1;
        if (s + w[k] == m) {
            found = true;
            cout << "{ ";
            for (int i = 0; i <= k; i++) if (inc[i]) cout << w[i] << " ";
            cout << "}" << endl;
        } else if (k + 1 < n && s + w[k] + w[k + 1] <= m) {
            st.push({s + w[k], k + 1, r - w[k], inc});
        }
        if (k + 1 < n && s + r - w[k] >= m && s + w[k + 1] <= m) {
            vector<int> exc = x;
            exc[k] = 0;
            st.push({s, k + 1, r - w[k], exc});
        }
    }
    if (!found) cout << "No subset found" << endl;
    return 0;
}
