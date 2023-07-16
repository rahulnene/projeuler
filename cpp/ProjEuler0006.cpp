#include <iostream>

inline static long long squareSum(int n) {
    return n * n * (n + 1) * (n + 1) / 4;
}

inline static long long sumSquare(int n) {
    return n * (n + 1) * (2 * n + 1) / 6;
}

int main() {
    int n = 100;
    std::cout << squareSum(n) - sumSquare(n) << std::endl;
}