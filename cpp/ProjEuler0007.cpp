#include <cmath>
#include <iostream>
#include <vector>
using namespace std;

// A sieve of erasthones up to a length n
int nth_prime(int n) {
    vector<int> primes;
    int i = 2;
    while (primes.size() < n) {
        bool isPrime = true;
        for (int j = 2; j <= sqrt(i); ++j) {
            if (i % j == 0) {
                isPrime = false;
                break;
            }
        }
        if (isPrime) {
            primes.push_back(i);
        }
        ++i;
    }
    return primes.back();
}

int main() {
    cout << nth_prime(10001) << endl;
}
