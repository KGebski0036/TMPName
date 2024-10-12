#include "gui.hpp"
#include "ftxui/component/captured_mouse.hpp"
#include "ftxui/component/component.hpp"
#include "ftxui/component/component_options.hpp"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/dom/elements.hpp"

void draw(const nlohmann::json &json) {
    using namespace ftxui;
    auto screen = ScreenInteractive::TerminalOutput();

    std::vector<std::string> entries;
    for (auto x: json.at("list")) {
        entries.push_back(x.at("name"));
    }
    int selected = 0;
    MenuOption option;
    option.on_enter = screen.ExitLoopClosure();
    auto menu = Menu(&entries, &selected, option);
    auto test  = Renderer(menu,[&]{
        return window(text("essa"),menu->Render());
    });
    auto details = window(text("details"), vbox({
                                                        text("test") | border,
                                                        text("cos")
                                                }));
    auto details_comp = Renderer([&]{
        return details;
    });
    auto all = Container::Horizontal({
       test,details_comp
    });

    screen.Loop(all);

    std::cout << "Selected element = " << selected << std::endl;
}