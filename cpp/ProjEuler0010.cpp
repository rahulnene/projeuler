#include <cmath>
#include <iostream>
#include <numeric>
#include <vector>
using namespace std;

long long int nth_prime(long long int n) {
    vector<bool> isPrime(n + 1, true);
    long long int sum = 0;
    for (long long int i = 2; i <= n; ++i) {
        if (isPrime[i]) {
            sum += i;
            for (long long int j = i * i; j <= n; j += i) {
                isPrime[j] = false;
            }
        }
    }
    return sum;
}


int main() {
    cout << nth_prime(2000001) << endl;
}