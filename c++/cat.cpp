#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <memory>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

template <typename CharT, typename Traits>
void stream_copy(std::basic_istream<CharT, Traits> &is,
                 std::basic_ostream<CharT, Traits> &os) {
    const std::istreambuf_iterator<CharT, Traits> begin{ is };
    const decltype(begin) end;

    std::copy(begin, end, std::ostreambuf_iterator<CharT, Traits>{ os });
}

struct StreamPair {
    std::vector<std::reference_wrapper<std::istream>> streams;
    std::vector<std::unique_ptr<std::istream>> owners;
};

StreamPair get_files(const std::vector<const char*> &args) {
    StreamPair pair;
    pair.streams.reserve(args.size());
    pair.owners.reserve(args.size());

    if (args.empty()) {
        pair.streams.emplace_back(std::cin);

        return std::move(pair);   
    }

    for (const auto &filename : args) {
        const std::string_view filename_view{ filename };

        if (filename_view == "-") {
            pair.streams.emplace_back(std::cin);
        } else {
            auto owner = std::make_unique<std::ifstream>(filename);

            pair.owners.emplace_back(std::move(owner));
            pair.streams.emplace_back(*pair.owners.back());
        }
    }

    return std::move(pair);
}

int main(const int argc, const char *const argv[]) {
    const std::vector<const char*> filenames{ argv + 1, argv + argc };

    auto [streams, owners] = get_files(filenames);

    for (std::istream &stream : streams) {
        stream_copy(stream, std::cout);
    }
}
