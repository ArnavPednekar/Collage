#include <iostream>
#include <vector>
using namespace std;

vector<int> encode_hamming(const vector<int>&data){
    int m = data.size();
    int r = 0;

  while((1<<r)<m+r+1){
      r++;
  }
  int n = m + r;
  vector<int> codeword(n+1,0);

  for(int i = 1, d = 0; i<= n;i++){
      if((i &(i-1))!=0){
          codeword[i] = data[d++];
      }
  }
  for (int i = 0; i < r; i++) {
      int pos = 1 <<i ;
      int val = 0;
      for(int j = 1; j<=n;j++){
          if(j&pos) val ^=codeword[j];
      }
      codeword[pos] = val;
  }

   return vector<int>(codeword.begin()+2,codeword.end());

}

void decode_hamming(vector<int> & received){
    int n = received.size();
    
    vector<int> codeword(n + 1);
       for (int i = 0; i < n; i++) {
           codeword[i + 1] = received[i];
       }
       int r = 0;
   
     while((1<<r)<m+r+1){
         r++;
     }  

     int error_pos = 0;

     for (int i = 0; i < r; i++) {
            int pos = 1 << i;
            int val = 0;
            for (int j = 1; j <= n; j++) {
                if (j & pos) val ^= codeword[j];
            }

            if (val != 0){
                error_pos += pos;
            }
            if (error_pos == 0) {
                    cout << "No error detected in the received codeword." << endl;
           } else{
                cout << "error at"<< error_pos << endl;
            }
}
int main() {
    int m;
    cout << "Enter number of data bits: ";
    cin >> m;

    cout << "Enter " << m << " data bits: ";
    string s;
    cin >> s;
    vector<int> data(m);
    for (int i = 0; i < m; ++i) {
        data[i] = s[i] - '0';
    }

    vector<int> codeword = encode_hamming(data);

    cout << "Encoded Codeword: ";
    for (int bit : codeword) {
        cout << bit;
    }
    cout << endl;

    return 0;
}
