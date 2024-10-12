#include "gui.hpp"
#include "ftxui/component/captured_mouse.hpp"
#include "ftxui/component/component.hpp"
#include "ftxui/component/component_options.hpp"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/dom/elements.hpp"

void draw(const nlohmann::json &json) {
    using namespace ftxui;
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
    auto details_comp = Renderer([&] {
        return window(text("details"), vbox({text(focused->name) | border, text("cos"),})) | flex;
    });
    auto all = Container::Horizontal({
                                             test, details_comp
                                     });

    screen.Loop(all);

    std::cout << "Selected element = " << selected << std::endl;
}