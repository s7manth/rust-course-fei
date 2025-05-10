#include <vector>
#include <iostream>

// after the first iteration, the value at the start of the array changes to 4
// which is then used as the incrementing factor for the other elements
void add_numbers(std::vector<int> &items, const int &value) {
    for (auto &item: items) {
        item += value;
    }
}

int main() {
    std::vector<int> items{2, 2, 2, 2, 2};
    add_numbers(items, items[0]);

    for (auto item: items) {
        std::cout << item << " ";
    }
    std::cout << std::endl;
    return 0;
}
