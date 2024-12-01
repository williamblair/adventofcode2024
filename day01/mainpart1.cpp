#include <algorithm>
#include <cstdlib>
#include <vector>
#include <iostream>
#include <fstream>
#include <sstream>

std::vector<int> list1;
std::vector<int> list2;

static void readLists(const char* fileName)
{
    std::ifstream in(fileName);
    if (!in || !in.is_open()) {
        std::cout << "Failed to open " << fileName << std::endl;
        exit(EXIT_FAILURE);
    }
    
    int val1, val2;
    while (in && (in >> val1)) {
        in >> val2;
        list1.push_back(val1);
        list2.push_back(val2);
    }
    if (list1.size() != list2.size()) {
        std::cout << "List sizes dont match" << std::endl;
        exit(EXIT_FAILURE);
    }
}

int main(int argc, char* argv[])
{
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <filename>" << std::endl;
        exit(EXIT_FAILURE);
    }
    readLists(argv[1]);
    
    std::sort(list1.begin(), list1.end());
    std::sort(list2.begin(), list2.end());
    
    int dist = 0;
    for (size_t i = 0; i < list1.size(); i++) {
        dist += std::abs(list1[i] - list2[i]);
    }
    
    std::cout << "Distance sum: " << dist << std::endl;
    
    return 0;
}
