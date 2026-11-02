#include <iostream>
#include <sstream>
#include <string>
using namespace std;
int main() {
    string ip;
    cout << "Enter IP address: ";
    cin >> ip;
    int a, b, c, d;
    char dot1, dot2, dot3;
    stringstream ss(ip);
    ss >> a >> dot1 >> b >> dot2 >> c >> dot3 >> d;
    if (ss.fail() || dot1 != '.' || dot2 != '.' || dot3 != '.' ||
        a < 0 || a > 255 || b < 0 || b > 255 || c < 0 || c > 255 || d < 0 || d > 255) {
        cout << "Invalid IP address!" << endl;
        return 0;
    }
    cout << "\nIP Address: " << ip << endl;
    if (a >= 1 && a <= 126) {
        cout << "Class: A" << endl;
        cout << "Default Subnet Mask: 255.0.0.0" << endl;
    } else if (a >= 128 && a <= 191) {
        cout << "Class: B" << endl;
        cout << "Default Subnet Mask: 255.255.0.0" << endl;
    } else if (a >= 192 && a <= 223) {
        cout << "Class: C" << endl;
        cout << "Default Subnet Mask: 255.255.255.0" << endl;
    } else {
        cout << "Class: Not A/B/C" << endl;
        cout << "Default Subnet Mask: N/A" << endl;
    }
    return 0;
}
