#include <cstdlib>
#include <cmath>
#include <iostream>
#include <vector>
#include <sstream>
#include <fstream>
#include <cstdint>

struct Equation
{
    enum class Operator
    {
        Add = 0,
        Mul,
        NumOperators
    };
    
    enum class OperatorP2
    {
        Add = 0,
        Mul,
        Concat,
        NumOperators
    };
    
    Equation(const std::string& input)
    {
        resultVal = 0;
        std::stringstream ss(input);
        char c;
        ss >> resultVal;
        ss >> c;
        
        uint64_t curVal;
        while (ss >> curVal) {
            values.push_back(curVal);
        }
        if (values.empty()) {
            std::cout << "Failed to parse input values" << std::endl;
            exit(EXIT_FAILURE);
        }
    }
    
    bool IsSolvable()
    {
        if (values.size() == 1) {
            std::cout << "IsSolvable values size == 1" << std::endl;
            exit(EXIT_FAILURE);
        }
        size_t eqNumOperators = values.size()-1;
        size_t numPossibleCombinations = (int)pow((int)Operator::NumOperators,eqNumOperators);
        std::vector<std::vector<Operator>> allCombos(numPossibleCombinations);
        for (auto& vec : allCombos) {
            vec.resize(eqNumOperators);
        }
        size_t numTillSwitch = 1;
        for (int i=eqNumOperators-1; i>=0; i--) {
            int curOp = 0;
            for (size_t j=0; j<numPossibleCombinations; j++) {
                allCombos[j][i] = (Operator)curOp;
                if ((j+1)%numTillSwitch == 0) {
                    curOp = (curOp+1)%(int)Operator::NumOperators;
                }
            }
            numTillSwitch *= (int)Operator::NumOperators;
        }
        for (const std::vector<Operator>& ops : allCombos) {
            if (Evaluate(ops) == resultVal) {
                return true;
            }
        }
        return false;
    }
    
    uint64_t Evaluate(const std::vector<Operator>& operators)
    {
        if (operators.size() != values.size()-1) {
            std::cout << "Unexpected operators/values size" << std::endl;
            exit(EXIT_FAILURE);
        }
        uint64_t curResult = values[0];
        for (size_t i=1; i<values.size(); i++) {
            switch (operators[i-1])
            {
            case Operator::Add:
                curResult = curResult + values[i];
                break;
            case Operator::Mul:
                curResult = curResult * values[i];
                break;
            default:
                std::cout << "Unhandled operator" << std::endl;
                exit(EXIT_FAILURE);
            }
        }
        return curResult;
    }
    
    bool IsSolvableP2()
    {
        if (values.size() == 1) {
            std::cout << "IsSolvable values size == 1" << std::endl;
            exit(EXIT_FAILURE);
        }
        size_t eqNumOperators = values.size()-1;
        size_t numPossibleCombinations = (int)pow((int)OperatorP2::NumOperators,eqNumOperators);
        std::vector<std::vector<OperatorP2>> allCombos(numPossibleCombinations);
        for (auto& vec : allCombos) {
            vec.resize(eqNumOperators);
        }
        size_t numTillSwitch = 1;
        for (int i=eqNumOperators-1; i>=0; i--) {
            int curOp = 0;
            for (size_t j=0; j<numPossibleCombinations; j++) {
                allCombos[j][i] = (OperatorP2)curOp;
                if ((j+1)%numTillSwitch == 0) {
                    curOp = (curOp+1)%(int)OperatorP2::NumOperators;
                }
            }
            numTillSwitch *= (int)OperatorP2::NumOperators;
        }
        for (const std::vector<OperatorP2>& ops : allCombos) {
            if (EvaluateP2(ops) == resultVal) {
                return true;
            }
        }
        return false;
    }
    
    uint64_t EvaluateP2(const std::vector<OperatorP2>& operators)
    {
        if (operators.size() != values.size()-1) {
            std::cout << "Unexpected operators/values size" << std::endl;
            exit(EXIT_FAILURE);
        }
        uint64_t curResult = values[0];
        for (size_t i=1; i<values.size(); i++) {
            switch (operators[i-1])
            {
            case OperatorP2::Add:
                curResult = curResult + values[i];
                break;
            case OperatorP2::Mul:
                curResult = curResult * values[i];
                break;
            case OperatorP2::Concat:
            {
                std::string s1 = std::to_string(curResult);
                std::string s2 = std::to_string(values[i]);
                std::string s3 = s1+s2;
                std::stringstream ss(s3);
                ss >> curResult;
                break;
            }
            default:
                std::cout << "Unhandled operator" << std::endl;
                exit(EXIT_FAILURE);
            }
        }
        return curResult;
    }
    
    uint64_t resultVal;
    std::vector<uint64_t> values;
};
std::vector<Equation> equations;
static void readEquations(const char* fileName)
{
    std::ifstream inFile(fileName);
    if (!inFile || !inFile.is_open()) {
        std::cout << "Failed to open " << fileName << std::endl;
        exit(EXIT_FAILURE);
    }
    std::string s;
    while (std::getline(inFile, s)) {
        equations.push_back(Equation(s));
        s = "";
    }
}

static void part1()
{
    uint64_t valuesSum = 0;
    for (Equation& eq : equations) {
        if (eq.IsSolvable()) {
            valuesSum += eq.resultVal;
        }
    }
    std::cout << "Part1 sum: " << valuesSum << std::endl;
}

static void part2()
{
    uint64_t valueSum = 0;
    for (Equation& eq : equations) {
        if (eq.IsSolvableP2()) {
            valueSum += eq.resultVal;
        }
    }
    std::cout << "Part2 sum: " << valueSum << std::endl;
}

int main(int argc, char* argv[])
{
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <input file>" << std::endl;
        return 1;
    }
    readEquations(argv[1]);
    
    part1();
    part2();
}
