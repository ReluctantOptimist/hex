//! custom (Hex 2.0): the Themes pane. The window's theme and everything
//! about the dictation pill (where it shows, its style, its color) in one
//! place, picked from the look catalog.

use rune::El;

use super::controls::{Paint, page, section};
use super::general::pick_row;
use super::icons::PANEL;
use super::{Ui, scroll_body};
use crate::app_settings::IndicatorPosition;
use crate::desktop::i18n::tr;

pub(super) fn pane(ui: &Ui, p: Paint, layer: &mut Vec<El>) -> El {
    let settings = &ui.settings;
    let app = vec![pick_row(
        ui,
        p,
        "app-theme",
        &PANEL,
        "App theme",
        vec!["Hex standard", "Arcade (Hex 2.0 logo)", "Comic (DSF logo)"],
        usize::from(settings.app_theme).min(2),
        |ui, index| {
            let theme = index as u8;
            ui.update(|settings| settings.app_theme = theme);
        },
        layer,
    )];
    let pill = vec![
        pick_row(
            ui,
            p,
            "indicator",
            &PANEL,
            tr("Dictation HUD"),
            vec![tr("Top"), tr("Bottom"), tr("Off")],
            match settings.indicator_position {
                IndicatorPosition::Top => 0,
                IndicatorPosition::Bottom => 1,
                IndicatorPosition::Hidden => 2,
            },
            |ui, index| {
                let position = [
                    IndicatorPosition::Top,
                    IndicatorPosition::Bottom,
                    IndicatorPosition::Hidden,
                ][index];
                if ui.settings.indicator_position != position {
                    ui.update(|settings| settings.indicator_position = position);
                }
            },
            layer,
        ),
        pick_row(
            ui,
            p,
            "pill-style",
            &PANEL,
            "Pill style",
            crate::rune_hud::PILL_STYLE_LABELS.to_vec(),
            crate::rune_hud::PILL_STYLES
                .iter()
                .position(|style| *style == settings.pill_style)
                .unwrap_or(1),
            |ui, index| {
                let style = crate::rune_hud::PILL_STYLES[index];
                ui.update(|settings| settings.pill_style = style);
            },
            layer,
        ),
        pick_row(
            ui,
            p,
            "pill-color",
            &PANEL,
            "Pill color",
            crate::rune_hud::PILL_COLOR_LABELS.to_vec(),
            usize::from(settings.pill_color).min(41),
            |ui, index| {
                let color = index as u8;
                ui.update(|settings| settings.pill_color = color);
            },
            layer,
        ),
    ];
    let content = page("content")
        .child(section(p, "theme-app", "App", app))
        .child(section(p, "theme-pill", "Dictation pill", pill));
    scroll_body(p.k, "scroll", content)
}
