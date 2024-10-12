#include "gui.hpp"
#include "ftxui/component/captured_mouse.hpp"
#include "ftxui/component/component.hpp"
#include "ftxui/component/component_options.hpp"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/dom/elements.hpp"

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


void draw(const nlohmann::json &json) {
    auto screen = ScreenInteractive::TerminalOutput();

    struct entry{
        std::string name;
        std::string path;
        std::string ver;
        std::string curse_id;
    };
    std::optional<entry> focused;
    std::vector<std::string> ids;
    std::vector<entry> entries;
    for (auto x: json.at("list")) {
        ids.emplace_back(x.at("name"));
        entries.emplace_back(x.at("name"),x.at("path"),x.at("ver"),x.at("curseId"));
    }
    int selected = 0;
    MenuOption option;
    if(!ids.empty()){
        focused = entries.at(0);
    }
    option.on_change =[&]() {
        focused = entries.at(static_cast<unsigned long>(selected));
    };
    auto menu = Menu(&ids, &selected, option);
    auto test = Renderer(menu, [&] {
        return window(text("essa"), menu->Render()) | flex;
    });
    auto details_info = Renderer([&] {
        return window(text("Details about deck"), vbox({
          vbox({
            center(text(focused->name)),
            hbox({
              center(text("Author: Anonymus")) | flex,
              center(text("Number of cards: 69")) | flex,
              center(text("Course: CBE2024")) | flex,
              center(text("Update: 10.12.2023")) | flex,
            })
          }) | border,
          separator(),
          paragraph(focused->name + " baza pytań."),
        })) | flex;
    });
    auto details = Container::Vertical({
      details_info,
      Container::Horizontal({
         Button("Download/Update", [&] { return; }, Style(Color::Green)),
         Button("Delete", [&] { return; }, Style(Color::Red)),
      })
    });
    auto all = Container::Horizontal({
                                             test, details | flex
                                     });

    screen.Loop(all);
}
