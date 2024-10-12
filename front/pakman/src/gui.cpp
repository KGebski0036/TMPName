#include "gui.hpp"
#include "ftxui/component/captured_mouse.hpp"
#include "ftxui/component/component.hpp"
#include "ftxui/component/component_options.hpp"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/dom/elements.hpp"

void draw(const nlohmann::json &json) {
    using namespace ftxui;
    auto screen = ScreenInteractive::TerminalOutput();
    std::string focused = "";
    std::vector<std::string> entries;
    for (auto x: json.at("list")) {
        entries.push_back(x.at("name"));
    }
    int selected = 0;
    MenuOption option;
    option.on_enter = [&]() {
        focused = entries.at(static_cast<unsigned long>(selected));
    };
    auto menu = Menu(&entries, &selected, option);
    auto test = Renderer(menu, [&] {
        return window(text("essa"), menu->Render()) | flex;
    });
    auto details_comp = Renderer([&] {
        return window(text("details"), vbox({text(focused) | border, text("cos"),})) | flex;
    });
    auto all = Container::Horizontal({
                                             test, details_comp
                                     });

    screen.Loop(all);

    std::cout << "Selected element = " << selected << std::endl;
}