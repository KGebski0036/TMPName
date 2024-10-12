#include <downloader.hpp>
#include <string>
#include "gui.hpp"
#include "nlohmann/json.hpp"
auto main() -> int {
    // std::string url = "http://localhost:8080/Master.json";  // Replace with the actual file path
    // std::string outputFile = "downloaded_file.txt";
    //
    // downloadFile(url, outputFile);
    const nlohmann::json j = nlohmann::json::parse("{\n"
                                             "    \"list\": [\n"
                                             "        {\n"
                                             "            \"id\": 1,\n"
                                             "            \"name\": \"Miernictwo\",\n"
                                             "            \"path\": \"/path/to/file\",\n"
                                             "            \"ver\": \"1.0\",\n"
                                             "            \"curseId\": \"CBE2023\"\n"
                                             "        },\n"
                                             "        {\n"
                                             "            \"id\": 2,\n"
                                             "            \"name\": \"Miernictwo2\",\n"
                                             "            \"path\": \"/path/to/file2\",\n"
                                             "            \"ver\": \"1.1\",\n"
                                             "            \"curseId\": \"CBE2023\"\n"
                                             "        },\n"
                                             "        {\n"
                                             "            \"id\": 3,\n"
                                             "            \"name\": \"Telekomuna\",\n"
                                             "            \"path\": \"/path/to/file3\",\n"
                                             "            \"ver\": \"1.0\",\n"
                                             "            \"curseId\": \"CBE2023\"\n"
                                             "        },\n"
                                             "        {\n"
                                             "            \"id\": 4,\n"
                                             "            \"name\": \"Ekonomia\",\n"
                                             "            \"path\": \"/path/to/file4\",\n"
                                             "            \"ver\": \"2.0\",\n"
                                             "            \"curseId\": \"CBD2023\"\n"
                                             "        }\n"
                                             "    ]\n"
                                             "}");
    draw(j);
}