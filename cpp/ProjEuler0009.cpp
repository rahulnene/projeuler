#include <iostream>

using namespace std;

int main() {
    for (int a = 1; a <= 998; a++) {
        for (int b = 1; b<=a; b++) {
            int c = 1000-a-b;
            if (a*a + b*b == c*c) {
                cout << a*b*c << endl;
                return 0;
            }
        }
    }
}