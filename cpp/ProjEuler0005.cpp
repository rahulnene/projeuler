#include <iostream>

long long gcd(long long a, long long b) {
    if (b == 0) {
        return a;
    }
    return gcd(b, a % b);
}

long long lcm(long long a, long long b) {
    return a * b / gcd(a, b);
}

long long lcm_of_1_to_n(int n) {
    long long result = 1;
    for (int i = 2; i <= n; i++) {
        result = lcm(result, i);
    }
    return result;
}

int main() {
    int n = 20;
    std::cout << "LCM of 1 to " << n << " is " << lcm_of_1_to_n(n) << std::endl;
    return 0;
}