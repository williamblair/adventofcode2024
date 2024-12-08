#include <vector>
#include <unordered_map>
#include <set>
#include <iostream>
#include <fstream>

typedef std::vector<std::pair<int,int>> PosList;

std::vector<std::string> map;
std::unordered_map<char,PosList> posMap;

void readPosMap(const char* fileName)
{
    std::ifstream inFile(fileName);
    if (!inFile || !inFile.is_open()) {
        std::cout << "Failed to open " << fileName << std::endl;
        exit(EXIT_FAILURE);
    }

    std::string s;
    while (std::getline(inFile, s)) {
        map.push_back(s);
    }
}

void genPosList()
{
    for (int row=0; row<map.size(); row++) {
        for (int col=0; col<map[0].size(); col++) {
            if (map[row][col] == '.') {
                continue;
            }
            const char ant = map[row][col];
            if (!(ant >= '0' && ant <= '9') && !(ant >= 'a' && ant <= 'z')
                    && !(ant >= 'A' && ant <= 'Z')) {
                std::cout << "Unexpected antenna character" << std::endl;
                exit(EXIT_FAILURE);
            }
            posMap[ant].push_back({row,col});
        }
    }
}

void part1()
{
    std::set<std::pair<int,int>> antinodes;
    for (const auto& posPair : posMap) {
        const PosList& posList = posPair.second;
        for (size_t i=0; i<posList.size()-1; i++) {
            for (size_t j=i+1; j<posList.size(); j++) {

                const int dRow = posList[j].first - posList[i].first;
                const int dCol = posList[j].second - posList[i].second;

                int curRow = posList[j].first + dRow;
                int curCol = posList[j].second + dCol;
                if (curRow >= 0 && curRow < map.size() &&
                        curCol >= 0 && curCol < map[0].size())
                {
                    antinodes.insert({curRow,curCol});
                }


                curRow = posList[i].first - dRow;
                curCol = posList[i].second - dCol;
                if (curRow >= 0 && curCol < map.size() &&
                        curCol >= 0 && curCol < map[0].size())
                {
                    antinodes.insert({curRow,curCol});
                }
            }
        }
    }
    std::cout << "Part1 size: " << antinodes.size() << std::endl;
}

void part2()
{
    std::set<std::pair<int,int>> antinodes;
    for (const auto& posPair : posMap) {
        const PosList& posList = posPair.second;
        for (size_t i=0; i<posList.size()-1; i++) {
            for (size_t j=i+1; j<posList.size(); j++) {

                const int dRow = posList[j].first - posList[i].first;
                const int dCol = posList[j].second - posList[i].second;

                antinodes.insert(posList[i]);
                antinodes.insert(posList[j]);

                int curRow = posList[j].first + dRow;
                int curCol = posList[j].second + dCol;
                while (curRow >= 0 && curRow < map.size() &&
                        curCol >= 0 && curCol < map[0].size())
                {
                    antinodes.insert({curRow,curCol});
                    curRow += dRow;
                    curCol += dCol;
                }


                curRow = posList[i].first - dRow;
                curCol = posList[i].second - dCol;
                while (curRow >= 0 && curCol < map.size() &&
                        curCol >= 0 && curCol < map[0].size())
                {
                    antinodes.insert({curRow,curCol});
                    curRow -= dRow;
                    curCol -= dCol;
                }
            }
        }
    }
    std::cout << "Part2 size: " << antinodes.size() << std::endl;
}

int main(int argc, char* argv[])
{
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <filename>" << std::endl;
        return 1;
    }
 
    readPosMap(argv[1]);
    genPosList();

    part1();
    part2();

    return 0;
}

