#include <iostream>
#include <string>
#include <string_view>

std::string make_string(const std::string_view string) {
    static constexpr std::size_t SIZE = 4096;

    std::string yes;
    yes.reserve((string.size() + 1) * SIZE);

    for (std::size_t i = 0; i < SIZE; ++i) {
        yes += string;
        yes += '\n';
    }

    return yes;
}

int main(const int argc, const char *const argv[]) {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);

    const std::string_view string = [argc, &argv] {
        if (argc > 1) {
            return argv[1];
        }

        return "y";
    }();

    const std::string yes_string = make_string(string);

    const char *const data = yes_string.data();
    const std::size_t size = yes_string.size();

    std::streambuf &buffer = *std::cout.rdbuf();

    while (true) {
        buffer.sputn(data, size);
    }
}
