#include <cmath>
#include <iostream>

using namespace std;

bool isPalindromic(int n) {
    int temp = n;
    int reverse = 0;
    while (temp > 0) {
        reverse = reverse * 10 + temp % 10;
        temp /= 10;
    }
    return reverse == n;
}

int main() {
    int max = 0;
    for (int i = 999; i >= 100; i--) {
        for (int j = 999; j >= 100; j--) {
            int product = i * j;
            if (product < max) {
                break;
            }
            if (isPalindromic(product)) {
                if (product > max) {
                    max = product;
                    cout << max << endl;
                }
            }
        }
    }
}
