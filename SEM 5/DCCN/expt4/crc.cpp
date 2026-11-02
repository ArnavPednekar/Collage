#include <iostream>
#include <string>


using namespace std;

string findXor(string a , string b){
    int n = b.length();
    string result = "";
    for(int i = 0; i < n; i++){
        if(a[i] == b[i]){
            result += '0';
        } else {
            result += '1';
        }
    }
    return result;
}

string mod2div(string dividend, string divisor){
    int n = dividend.length();
    int pick = divisor.length();
    string tmp = dividend.substr(0, pick);

    while (pick < n) {
        if(tmp[0] == '1'){
            tmp = findXor(tmp, divisor) + dividend[pick];
        } else {
            tmp = findXor(tmp, divisor) + dividend[pick];
        }
        pick++;
    }
    return tmp;
}

string encodeCRC(string data, string generator){
    string tmp = data + string(generator.length() - 1, '0');
    string remainder = mod2div(tmp, generator);
    return data + remainder;
}

int receiverCRC(string data, string generator){
    string tmp = data.substr(0, data.length() - generator.length() + 1);
    string remainder = mod2div(tmp, generator);
    return remainder == string(generator.length() - 1, '0') ? 0 : 1;
}

string addBinary1sComplement(string a, string b) {
    int n = a.length();
    string sum = "";
    int carry = 0;

    for (int i = n - 1; i >= 0; i--) {
        int bitA = a[i] - '0';
        int bitB = b[i] - '0';
        int total = bitA + bitB + carry;
        if (total >= 2) {
            carry = 1;
            sum = to_string(total - 2) + sum;
        } else {
            carry = 0;
            sum = to_string(total) + sum;
        }
    }

    if (carry == 1) {
        string one = string(n - 1, '0') + "1";
        sum = addBinary1sComplement(sum, one);
    }

    return sum;
}

string onesComplement(string s) {
    string result = "";
    for (char c : s) {
        result += (c == '0' ? '1' : '0');
    }
    return result;
}

int main(){
    int choice;
    cout << "1. CRC\n";
    cout << "2. Checksum\n";
    cout << "Enter choice: ";
    cin >> choice;

    if (choice == 1) {
        string data, key;
        cout << "Enter data: ";
        cin >> data;
        cout << "Enter key: ";
        cin >> key;
        string encoded = encodeCRC(data, key);
        cout << "Encoded: " << encoded << endl;
        int result = receiverCRC(encoded, key);
        cout << "Result: " << result << endl;
    } else if (choice == 2) {
        string sub1, sub2;
        cout << "Enter Subunit 1: ";
        cin >> sub1;
        cout << "Enter Subunit 2: ";
        cin >> sub2;

        string sum = addBinary1sComplement(sub1, sub2);
        string checksum = onesComplement(sum);

        cout << "Checksum: " << checksum << endl;

        string recvSum = addBinary1sComplement(sum, checksum);
        string finalSumComp = onesComplement(recvSum);

        int result = 0;
        for (char c : finalSumComp) {
            if (c != '0') {
                result = 1;
                break;
            }
        }
        cout << "Receiver Result: " << result << endl;
    } else {
        cout << "Invalid choice!" << endl;
    }

    return 0;
}
