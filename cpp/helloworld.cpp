#include <vector>
#include <iostream>

using namespace std;

class Solution {
public:
    int uniquePathsWithObstacles(vector<vector<int>>& grid) {
        if (grid.back().back() == 1) {
            return 0;
        }
        int m = grid.size(), n = grid[0].size();
        vector<vector<int>> dp(m, vector<int>(n, 0));
        dp[0][0] = 1;
        for (int i = 1; i < m; i++) {
            dp[i][0] = test(i - 1, 0, grid, dp);
        }
        for (int j = 1; j < n; j++) {
            dp[0][j] = test(0, j - 1, grid, dp);
        }
        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                dp[i][j] = test(i - 1, j, grid, dp) + test(i, j - 1, grid, dp);
            }
        }
        return dp.back().back();
    }
private:
    int test(int i, int j, vector<vector<int>>& grid, vector<vector<int>>& dp) {
        return (grid[i][j] != 1) * dp[i][j];
    }
};

int main() {
    Solution s;
    vector<vector<int>> grid{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}};
    int result = s.uniquePathsWithObstacles(grid);
    cout << result << endl;
    return 0;
}