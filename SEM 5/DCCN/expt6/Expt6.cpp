#include <iostream>
#include <string>

using namespace std;

void byteStuffing() {
    string data;
    char flag = 'F';
    char esc = 'E';
    cout << "Enter data: ";
    cin >> data;

    string stuffed = "";
    stuffed += flag;
    for (char c : data) {
        if (c == flag || c == esc) {
            stuffed += esc;
        }
        stuffed += c;
    }
    stuffed += flag;
    cout << "Stuffed Data: " << stuffed << endl;

    string destuffed = "";
    for (int i = 1; i < (int)stuffed.length() - 1; i++) {
        if (stuffed[i] == esc) {
            i++;
            destuffed += stuffed[i];
        } else {
            destuffed += stuffed[i];
        }
    }
    cout << "Destuffed Data: " << destuffed << endl;
}

void bitStuffing() {
    string data;
    cout << "Enter binary data: ";
    cin >> data;

    string stuffed = "";
    int count = 0;
    for (char c : data) {
        stuffed += c;
        if (c == '1') {
            count++;
            if (count == 5) {
                stuffed += '0';
                count = 0;
            }
        } else {
            count = 0;
        }
    }
    string flag = "01111110";
    string finalStuffed = flag + stuffed + flag;
    cout << "Stuffed Data: " << finalStuffed << endl;

    string destuffed = "";
    count = 0;
    for (int i = 0; i < (int)stuffed.length(); i++) {
        destuffed += stuffed[i];
        if (stuffed[i] == '1') {
            count++;
            if (count == 5) {
                i++;
                count = 0;
            }
        } else {
            count = 0;
        }
    }
    cout << "Destuffed Data: " << destuffed << endl;
}

int main() {
    int choice;
    cout << "1. Byte Stuffing\n2. Bit Stuffing\nEnter choice: ";
    cin >> choice;

    if (choice == 1) {
        byteStuffing();
    } else if (choice == 2) {
        bitStuffing();
    } else {
        cout << "Invalid choice" << endl;
    }

    return 0;
}
