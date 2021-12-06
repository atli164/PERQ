#include<bits/stdc++.h>
using namespace std;

int main() {
    int mx = 100000000;
    int shorts[16][16];
    for(int i = 0; i < 16; ++i) {
        for(int j = 0; j < 16; ++j) {
            shorts[i][j] = rand();
        }
    }
    clock_t start = clock();
    for(int i = 0; i < mx; ++i) {
        for(int j = 0; j < 16; ++j) {
            shorts[0][j] = shorts[i % 16][j] + shorts[0][j];
        }
    }
    cout << (float)(clock() - start)/CLOCKS_PER_SEC*1000 << "ms" << endl;
    for(int j = 0; j < 16; ++j) {
        cout << shorts[0][j] << endl;
    }
}
