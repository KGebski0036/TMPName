#include <cpr/cpr.h>
#include <downloader.hpp>

void downloadFile(const std::string& url, const std::string& outputFile) {
    const cpr::Response r = cpr::Get(cpr::Url{url});

    if (r.status_code == 200) {
        std::ofstream file(outputFile, std::ios::binary);
        if (file.is_open()) {
            file << r.text;
            file.close();
            std::cout << "\033[48;5;39m[INFORMATION]\033[0m \033[38;5;123m" << "Backdoor is not enabled\n" << "\033[0m" ;
        } else {
            std::cerr << "Failed to create a file " << outputFile << '\n';
        }
    } else {
        std::cerr << "Coudn't connect to server. Offline mode\n";
    }
}
