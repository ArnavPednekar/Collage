#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

using namespace std;

struct Item {
  int id;
  double profit;
  double weight;
  double ratio;
};

bool compareItems(const Item &a, const Item &b) { return a.ratio > b.ratio; }

void solveFractionalKnapsack(int n, double capacity, vector<Item> &items) {
  for (int i = 0; i < n; i++) {
    items[i].ratio = items[i].profit / items[i].weight;
  }

  sort(items.begin(), items.end(), compareItems);

  double remainingCapacity = capacity;
  double totalProfit = 0.0;

  vector<double> fraction(n, 0.0);

  cout << setw(10) << "Item ID" << setw(12) << "Fraction" << setw(18)
       << "Rem. Capacity" << setw(15) << "Profit Gained" << endl;

  for (int i = 0; i < n; i++) {
    double currentFraction = 0.0;
    double profitGained = 0.0;

    if (items[i].weight <= remainingCapacity) {
      currentFraction = 1.0;
      remainingCapacity -= items[i].weight;
      profitGained = items[i].profit;
      totalProfit += profitGained;
    } else {
      currentFraction = remainingCapacity / items[i].weight;
      profitGained = items[i].profit * currentFraction;
      totalProfit += profitGained;
      remainingCapacity = 0;
    }

    cout << setw(10) << items[i].id << setw(12) << fixed << setprecision(2)
         << currentFraction << setw(18) << remainingCapacity << setw(15)
         << profitGained << endl;

    if (remainingCapacity == 0) {
      break;
    }
  }

  cout << "Maximum Total Profit: " << fixed << setprecision(2) << totalProfit
       << endl;
}

int main() {
  int n;
  double capacity;

  cout << "Enter number of items: ";
  cin >> n;

  vector<Item> items(n);
  for (int i = 0; i < n; i++) {
    items[i].id = i + 1;
    cout << "Enter profit and weight for item " << i + 1 << ": ";
    cin >> items[i].profit >> items[i].weight;
  }

  cout << "Enter knapsack capacity: ";
  cin >> capacity;

  solveFractionalKnapsack(n, capacity, items);

  return 0;
}
