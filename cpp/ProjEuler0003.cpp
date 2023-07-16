#include <cmath>
#include <iostream>

using namespace std;

long largestPrimeFactor(long n) {
    long ans = -1;
    while (n % 2 == 0) {
        ans = 2;
        n /= 2;
    }
    for (long i = 3; i <= sqrt(n); i += 2) {
        while (n % i == 0) {
            ans = i;
            n /= i;
        }
    }
    if (n > 2) {
        ans = n;
    }
    return ans;
}

int main() {
    cout << largestPrimeFactor(600851475143) << endl;
    return 0;
}