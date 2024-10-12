#include <cpr/cpr.h>
#include <iostream>
#include <downloader.hpp>

void downloadFile(const std::string& url, const std::string& outputFile) {
    // Perform the GET request
    cpr::Response r = cpr::Get(cpr::Url{url});

    // Check if the request was successful
    if (r.status_code == 200) {
        std::ofstream file(outputFile, std::ios::binary);
        if (file.is_open()) {
            file << r.text;  // Write the content of the response to the file
            file.close();
            std::cout << "File downloaded successfully and saved as " << outputFile << std::endl;
        } else {
            std::cerr << "Failed to open output file: " << outputFile << std::endl;
        }
    } else {
        std::cerr << "Failed to download file. HTTP Status code: " << r.status_code << std::endl;
    }
}
