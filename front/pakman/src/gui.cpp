#include "gui.hpp"
#include "ftxui/component/captured_mouse.hpp"
#include "ftxui/component/component.hpp"
#include "ftxui/component/component_options.hpp"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/dom/elements.hpp"
#include <ftxui/dom/table.hpp>

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
    };

    std::vector<entry> entries;
    for (auto x : json.at("list")) {
        entries.emplace_back(x.at("name"), x.at("path"), x.at("ver"), x.at("curseId"));
    }
    int selected = 1;
    std::string filter_str;
    auto filter = Input(&filter_str, "Filter");
    MenuOption option;

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
    auto left = Container::Vertical({filter, menu});
    auto left_window = Renderer(left, [&] { return window(text("Library"), left->Render()) | flex; });
    auto details_info = Renderer([&] {
        return window(text("Deck Details"),
                      vbox({
                          vbox({center(text(entries.at(static_cast<unsigned long>(selected-1)).name)),
                                hbox({
                                    center(text("Author: Anonymus")) | flex,
                                    center(text("Number of cards: 69")) | flex,
                                    center(text("Course: CBE2024")) | flex,
                                    center(text("Update: 10.12.2023")) | flex,
                                })}) |
                              border,
                          separator(),
                          paragraph(entries.at(static_cast<unsigned long>(selected-1)).name + " baza pytań."),
                      })) |
               flex;
    });
    auto details = Container::Vertical({details_info, Container::Horizontal({
                                                          Button(
                                                              "Download/Update", [&] { return; }, Style(Color::Green)),
                                                          Button(
                                                              "Delete", [&] { return; }, Style(Color::Red)),
                                                      })});
    auto all = Container::Horizontal({left_window, details | flex});

    screen.Loop(all);
}
