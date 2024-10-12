#include "gui.hpp"
#include "ftxui/component/captured_mouse.hpp"
#include "ftxui/component/component.hpp"
#include "ftxui/component/component_options.hpp"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/dom/elements.hpp"
#include "downloader.hpp"
#include <ftxui/dom/table.hpp>
#include <fstream>


using namespace ftxui;

ButtonOption Style(Color color) {
    auto option = ButtonOption::Animated(color);
    option.transform = [](const EntryState& s) {
        auto element = text(s.label);
        if (s.focused) {
            element |= bold;
        }
        return element | center | borderEmpty | flex;
    };
    return option;
}

void draw(const nlohmann::json& json) {
    auto screen = ScreenInteractive::TerminalOutput();

    struct entry {
        std::string name;
        std::string path;
        std::string ver;
        std::string curse_id;
        std::string hash;
        std::string author;
        std::string description;
        int format {};
        int question_amount {};
    };

    std::vector<entry> entries;

    for (auto x: json.at("entries")) {
        entries.emplace_back(x.at("name"), x.at("path"), x.at("version"), x.at("course_id"), x.at("hash"));
    }
    int selected = 1;

    auto menu = Container::Vertical(
        [&] {
            Components comps;
            static unsigned long name_spacing = 2;
            static unsigned long curse_spacing = 2;
            name_spacing = 2;
            curse_spacing = 2;
            for (const auto& x : entries) {
                name_spacing = std::max(name_spacing, x.name.size());
                curse_spacing = std::max(curse_spacing, x.curse_id.size());
            }
            auto spacing = [](unsigned long len, unsigned long min) {
                std::string output;
                while (len - 2 < min--) {
                    output += " ";
                }
                return output;
            };
            comps.emplace_back(Renderer([&] {
                return text("  Name" + spacing(4, name_spacing) + "Curse" + spacing(5, curse_spacing) + "Version");
            }));
            for (const auto& x : entries) {
                comps.emplace_back(MenuEntry(x.name + spacing(x.name.size(), name_spacing) + x.curse_id +
                                             spacing(x.curse_id.size(), curse_spacing) + x.ver));
            }
            return comps;
        }(),
        &selected);
    auto left = Container::Vertical({menu});
    auto left_window = Renderer(left, [&] { return window(text("Library"), left->Render()) | flex; });

    auto details_info = Renderer([&] {

        auto current = entries.at(selected - 1);

        if (current.question_amount == 0) {
          std::string url = "http://localhost:8080/get_package_metadata?hash=" + current.hash;
          std::string outputFile = current.hash + ".json";

          downloadFile(url, outputFile);

          std::ifstream t(outputFile);
          std::stringstream buffer;
          buffer << t.rdbuf();
          const nlohmann::json j = nlohmann::json::parse(buffer.str());

          current.author = j.at("author");
          current.description = j.at("description");
          current.format = j.at("format");
          current.question_amount = j.at("question_amount");
        }



        return window(text("Details about deck"), vbox({
          vbox({
            center(text(current.name)),
            hbox({
              center(text("Author: " + current.author)) | flex,
              center(text("Number of cards: " + std::to_string(current.question_amount))) | flex,
            }) | flex,
            hbox({
              center(text("Course: " + current.curse_id)) | flex,
              center(text("Update: 10.12.2023")) | flex,
            }) | flex
          }) | border,
          separator(),
          paragraph(current.description),
        })) | flex;

    });
    auto details = Container::Vertical({details_info, Container::Horizontal({
                                                          Button(
                                                              "Download/Update", [&] { return; }, Style(Color::Green)),
                                                          Button(
                                                              "Delete", [&] { return; }, Style(Color::Red)),
                                                      })});
    auto all = Container::Horizontal({left_window, details | flex});

    all = all | size(WIDTH, LESS_THAN, 130);

    screen.Loop(all);
}
