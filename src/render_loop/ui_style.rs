use crate::config::Config;
use crate::render_loop::RenderLoop;

pub(super) struct UiStyleTokens<'ui> {
    _text_style: imgui::ColorStackToken<'ui>,
    _text_disabled_style: imgui::ColorStackToken<'ui>,
    _border_style: imgui::ColorStackToken<'ui>,
    _border_shadow_style: imgui::ColorStackToken<'ui>,
    _button_style: imgui::ColorStackToken<'ui>,
    _button_hovered_style: imgui::ColorStackToken<'ui>,
    _button_active_style: imgui::ColorStackToken<'ui>,
    _header_style: imgui::ColorStackToken<'ui>,
    _header_hovered_style: imgui::ColorStackToken<'ui>,
    _header_active_style: imgui::ColorStackToken<'ui>,
    _title_style: imgui::ColorStackToken<'ui>,
    _title_active_style: imgui::ColorStackToken<'ui>,
    _title_collapsed_style: imgui::ColorStackToken<'ui>,
    _frame_bg_style: imgui::ColorStackToken<'ui>,
    _frame_bg_hovered_style: imgui::ColorStackToken<'ui>,
    _frame_bg_active_style: imgui::ColorStackToken<'ui>,
    _check_mark_style: imgui::ColorStackToken<'ui>,
    _text_input_style: imgui::ColorStackToken<'ui>,
}

impl RenderLoop {
    pub(super) fn push_ui_styles<'ui>(
        &self,
        ui: &'ui imgui::Ui,
        config: &Config,
    ) -> UiStyleTokens<'ui> {
        let button_color = config.colors.button.to_float4();
        let button_hovered = config.colors.button_hovered.to_float4();
        let button_active = config.colors.button_active.to_float4();
        let text_color = config.colors.text.to_float4();

        UiStyleTokens {
            _text_style: ui.push_style_color(imgui::StyleColor::Text, text_color),
            _text_disabled_style: ui.push_style_color(
                imgui::StyleColor::TextDisabled,
                [0.0, 0.0, 0.0, 1.0],
            ),
            _border_style: ui.push_style_color(imgui::StyleColor::Border, [0.0, 0.0, 0.0, 1.0]),
            _border_shadow_style: ui.push_style_color(
                imgui::StyleColor::BorderShadow,
                [0.0, 0.0, 0.0, 0.5],
            ),
            _button_style: ui.push_style_color(imgui::StyleColor::Button, button_color),
            _button_hovered_style: ui.push_style_color(
                imgui::StyleColor::ButtonHovered,
                button_hovered,
            ),
            _button_active_style: ui.push_style_color(imgui::StyleColor::ButtonActive, button_active),
            _header_style: ui.push_style_color(imgui::StyleColor::Header, button_color),
            _header_hovered_style: ui.push_style_color(
                imgui::StyleColor::HeaderHovered,
                button_hovered,
            ),
            _header_active_style: ui.push_style_color(imgui::StyleColor::HeaderActive, button_active),
            _title_style: ui.push_style_color(imgui::StyleColor::TitleBg, button_color),
            _title_active_style: ui.push_style_color(imgui::StyleColor::TitleBgActive, button_color),
            _title_collapsed_style: ui.push_style_color(
                imgui::StyleColor::TitleBgCollapsed,
                button_active,
            ),
            _frame_bg_style: ui.push_style_color(imgui::StyleColor::FrameBg, button_hovered),
            _frame_bg_hovered_style: ui.push_style_color(
                imgui::StyleColor::FrameBgHovered,
                button_hovered,
            ),
            _frame_bg_active_style: ui.push_style_color(
                imgui::StyleColor::FrameBgActive,
                button_hovered,
            ),
            _check_mark_style: ui.push_style_color(imgui::StyleColor::CheckMark, [1.0, 0.0, 0.0, 1.0]),
            _text_input_style: ui.push_style_color(imgui::StyleColor::Text, [1.0, 1.0, 1.0, 1.0]),
        }
    }
}
