#include <fstream>
#include <iostream>
#include <vector>
#include <cassert>
#include <cstdlib>
#include <utility>
#include <set>

std::vector<std::vector<int>> inMap;
static void readInputMap(const char* fileName)
{
    std::ifstream inFile(fileName);
    if (!inFile || !inFile.is_open()) {
        std::cout << "Failed to open " << fileName << std::endl;
        exit(EXIT_FAILURE);
    }
    std::string s;
    while (std::getline(inFile, s)) {
        std::vector<int> curVec;
        for (const char c : s) {
            assert(c >= '0' && c <= '9');
            curVec.push_back((int)(c - '0'));
        }
        inMap.push_back(curVec);
    }
}

std::set<std::pair<size_t,size_t>> curTrailEnds;
static void doTrailSearch(const size_t curRow, const size_t curCol, const int targetPos)
{
    // check right
    size_t newRow = curRow;
    size_t newCol = curCol+1;
    if (newCol < inMap[0].size() && inMap[newRow][newCol] == targetPos) {
        if (targetPos == 9) {
            curTrailEnds.insert({newRow,newCol});
        } else {
            doTrailSearch(newRow, newCol, targetPos+1);
        }
    }
    // check down
    newRow = curRow+1;
    newCol = curCol;
    if (newRow < inMap.size() && inMap[newRow][newCol] == targetPos) {
        if (targetPos == 9) {
            curTrailEnds.insert({newRow,newCol});
        } else {
            doTrailSearch(newRow, newCol, targetPos+1);
        }
    }
    // check left
    newRow = curRow;
    newCol = curCol-1;
    if (curCol > 0 && inMap[newRow][newCol] == targetPos) {
        if (targetPos == 9) {
            curTrailEnds.insert({newRow,newCol});
        } else {
            doTrailSearch(newRow, newCol, targetPos+1);
        }
    }
    // check up
    newRow = curRow-1;
    newCol = curCol;
    if (curRow > 0 && inMap[newRow][newCol] == targetPos) {
        if (targetPos == 9) {
            curTrailEnds.insert({newRow,newCol});
        } else {
            doTrailSearch(newRow, newCol, targetPos+1);
        }
    }
}
static int getTrailCount(const size_t row, const size_t col)
{
    curTrailEnds.clear();
    doTrailSearch(row, col, 1);
    return curTrailEnds.size();
}

static void part1()
{
    int trailheadSum = 0;
    for (size_t row=0; row<inMap.size(); row++) {
        for (size_t col=0; col<inMap[0].size(); col++) {
            if (inMap[row][col] == 0) {
                int count = getTrailCount(row,col);
                std::cout << "Trailhead count: " << count << std::endl;
                trailheadSum += count;
            }
        }
    }
    std::cout << "Part1 sum: " << trailheadSum << std::endl;
}

std::vector<std::pair<size_t,size_t>> curTrailEndsVec;
static void doTrailRatingSearch(const size_t curRow, const size_t curCol, const int targetPos)
{
    // check right
    size_t newRow = curRow;
    size_t newCol = curCol+1;
    if (newCol < inMap[0].size() && inMap[newRow][newCol] == targetPos) {
        if (targetPos == 9) {
            curTrailEndsVec.push_back({newRow,newCol});
        } else {
            doTrailRatingSearch(newRow, newCol, targetPos+1);
        }
    }
    // check down
    newRow = curRow+1;
    newCol = curCol;
    if (newRow < inMap.size() && inMap[newRow][newCol] == targetPos) {
        if (targetPos == 9) {
            curTrailEndsVec.push_back({newRow,newCol});
        } else {
            doTrailRatingSearch(newRow, newCol, targetPos+1);
        }
    }
    // check left
    newRow = curRow;
    newCol = curCol-1;
    if (curCol > 0 && inMap[newRow][newCol] == targetPos) {
        if (targetPos == 9) {
            curTrailEndsVec.push_back({newRow,newCol});
        } else {
            doTrailRatingSearch(newRow, newCol, targetPos+1);
        }
    }
    // check up
    newRow = curRow-1;
    newCol = curCol;
    if (curRow > 0 && inMap[newRow][newCol] == targetPos) {
        if (targetPos == 9) {
            curTrailEndsVec.push_back({newRow,newCol});
        } else {
            doTrailRatingSearch(newRow, newCol, targetPos+1);
        }
    }
}
static int getTrailRating(const size_t row, const size_t col)
{
    curTrailEndsVec.clear();
    doTrailRatingSearch(row, col, 1);
    return curTrailEndsVec.size();
}

static void part2()
{
    int trailheadRatingSum = 0;
    for (size_t row=0; row<inMap.size(); row++) {
        for (size_t col=0; col<inMap[0].size(); col++) {
            if (inMap[row][col] == 0) {
                int rating = getTrailRating(row,col);
                std::cout << "Trailhead rating: " << rating << std::endl;
                trailheadRatingSum += rating;
            }
        }
    }
    std::cout << "Part2 sum: " << trailheadRatingSum << std::endl;
}

int main(int argc, char* argv[])
{
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <filename>" << std::endl;
        return 1;
    }
    readInputMap(argv[1]);

    part1();
    part2();

    return 0;
}

