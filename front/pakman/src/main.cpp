#include <iostream>
#include <print>

auto main() -> int {
    std::print("Hello, World!");
}
namespace {
    auto main() -> int {
        std::cout << "Hello, World!\n";
        return 0;
    }
}