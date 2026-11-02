#include <iostream>
#include <vector>
#include <algorithm>
#include <string>

using namespace std;

struct maxmin {
    int max;
    int min;
};

maxmin getMinMax(const vector<int>& array, int low, int high, int depth = 0) {
    string indent(depth * 4, ' ');
    cout << indent << "low = " << low << " high = " << high << "\n";

    maxmin result, left, right;

    if (low == high) {
        result.max = array[low];
        result.min = array[low];
        cout << indent << "min = " << result.min << " max = " << result.max << "\n";
        return result;
    }

    if (high == low + 1) {
        if (array[low] > array[high]) {
            result.max = array[low];
            result.min = array[high];
        } else {
            result.max = array[high];
            result.min = array[low];
        }
        cout << indent << "min = " << result.min << " max = " << result.max << "\n";
        return result;

    }

    int mid = low + (high - low) / 2;

    left = getMinMax(array, low, mid, depth + 1);
    right = getMinMax(array, mid + 1, high, depth + 1);

    result.min = min(left.min, right.min);
    result.max = max(left.max, right.max);

    cout  << indent << "min = " << result.min << " max = " << result.max << "\n";

    return result;
}

int main() {
    vector<int> array = {22, 13, -5, -8, 15, 60, 17, 31, 47};

    maxmin yes = getMinMax(array, 0, array.size() - 1);

    cout << "\nmax: " << yes.max << "\n";
    cout << "min: " << yes.min << "\n";

    return 0;
}
