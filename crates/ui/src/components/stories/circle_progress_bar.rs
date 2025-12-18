use crate::prelude::*;
use crate::{CircleDirection, CircleProgressBar, CircleSize, Label};
use std::time::Duration;

pub struct CircleProgressBarStory {
    progress: f32,
    _animation_task: Option<gpui::Task<()>>,
}

impl CircleProgressBarStory {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let animation_task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;

                let _ = this.update(cx, |this, cx| {
                    this.progress += 0.5;
                    if this.progress > 100.0 {
                        this.progress = 0.0;
                    }
                    cx.notify();
                });
            }
        });

        Self {
            progress: 0.0,
            _animation_task: Some(animation_task),
        }
    }
}

impl Render for CircleProgressBarStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let max_value = 100.0;
        let animated_progress = self.progress;

        div()
            .id("circle-progress-story")
            .flex()
            .flex_col()
            .gap_6()
            .p_4()
            .overflow_y_scroll()
            .h_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().text_sm().text_color(cx.theme().colors().text_muted).child(
                        "A circular progress indicator using SVG arc paths with customizable stroke, colors, angles, and direction.",
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_sm().text_color(cx.theme().colors().text_muted)
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("Status Bar Usage (Medium - 16px)"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .px_3()
                                    .py_1p5()
                                    .bg(cx.theme().colors().title_bar_background)
                                    .rounded_md()
                                    .child(
                                        CircleProgressBar::new(
                                            "status_bar_animated",
                                            animated_progress,
                                            max_value,
                                            cx,
                                        )
                                            .size(CircleSize::Medium)
                                            .stroke_width(px(2.)),
                                    )
                                    .child(Label::new(format!(
                                        "Downloading Zed update... {}%",
                                        animated_progress as u32
                                    ))),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .px_3()
                                    .py_1p5()
                                    .bg(cx.theme().colors().title_bar_background)
                                    .rounded_md()
                                    .child(
                                        CircleProgressBar::new(
                                            "indexing",
                                            animated_progress * 0.7,
                                            max_value,
                                            cx,
                                        )
                                            .size(CircleSize::Medium)
                                            .stroke_width(px(2.))
                                            .fg_color(cx.theme().status().info),
                                    )
                                    .child(Label::new("Indexing workspace...")),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .px_3()
                                    .py_1p5()
                                    .bg(cx.theme().colors().title_bar_background)
                                    .rounded_md()
                                    .child(
                                        CircleProgressBar::new(
                                            "syncing",
                                            animated_progress * 0.5,
                                            max_value,
                                            cx,
                                        )
                                            .size(CircleSize::Medium)
                                            .stroke_width(px(2.))
                                            .fg_color(cx.theme().status().success),
                                    )
                                    .child(Label::new("Syncing settings...")),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_sm().text_color(cx.theme().colors().text_muted)
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("Animated Progress (Various Sizes)"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_8()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .items_center()
                                    .child(CircleProgressBar::new(
                                        "animated_small",
                                        animated_progress,
                                        max_value,
                                        cx,
                                    ).size(CircleSize::Small))
                                    .child(Label::new(format!("{:.0}%", animated_progress))),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .items_center()
                                    .child(CircleProgressBar::new(
                                        "animated_medium",
                                        animated_progress,
                                        max_value,
                                        cx,
                                    ))
                                    .child(Label::new(format!("{:.0}%", animated_progress))),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .items_center()
                                    .child(CircleProgressBar::new(
                                        "animated_large",
                                        animated_progress,
                                        max_value,
                                        cx,
                                    ).size(CircleSize::Large))
                                    .child(Label::new(format!("{:.0}%", animated_progress))),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .items_center()
                                    .child(CircleProgressBar::new(
                                        "animated_ccw",
                                        animated_progress,
                                        max_value,
                                        cx,
                                    ).direction(CircleDirection::CounterClockwise))
                                    .child(Label::new("Counter-clockwise")),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_6()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(div().text_sm().text_color(cx.theme().colors().text_muted).font_weight(gpui::FontWeight::SEMIBOLD).child("Progress States"))
                            .child(
                                div()
                                    .flex()
                                    .gap_8()
                                    .items_center()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(CircleProgressBar::new(
                                                "empty",
                                                0.0,
                                                max_value,
                                                cx,
                                            ))
                                            .child(Label::new("0% - Empty")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(CircleProgressBar::new(
                                                "partial",
                                                max_value * 0.25,
                                                max_value,
                                                cx,
                                            ))
                                            .child(Label::new("25% - Quarter")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(CircleProgressBar::new(
                                                "half",
                                                max_value * 0.5,
                                                max_value,
                                                cx,
                                            ))
                                            .child(Label::new("50% - Half")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(CircleProgressBar::new(
                                                "three_quarters",
                                                max_value * 0.75,
                                                max_value,
                                                cx,
                                            ))
                                            .child(Label::new("75% - Three Quarters")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(CircleProgressBar::new(
                                                "filled",
                                                max_value,
                                                max_value,
                                                cx,
                                            ))
                                            .child(Label::new("100% - Complete")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(CircleProgressBar::new(
                                                "over",
                                                max_value * 1.2,
                                                max_value,
                                                cx,
                                            ))
                                            .child(Label::new("120% - Over-limit")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(div().text_sm().text_color(cx.theme().colors().text_muted).font_weight(gpui::FontWeight::SEMIBOLD).child("Size Variants"))
                            .child(
                                div()
                                    .flex()
                                    .gap_8()
                                    .items_center()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "xsmall",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .size(CircleSize::XSmall)
                                                    .stroke_width(px(2.)),
                                            )
                                            .child(Label::new("XSmall (12px)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "small",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .size(CircleSize::Small),
                                            )
                                            .child(Label::new("Small (14px)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "medium",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .size(CircleSize::Medium),
                                            )
                                            .child(Label::new("Medium (16px)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "large",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .size(CircleSize::Large),
                                            )
                                            .child(Label::new("Large (20px)")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(div().text_sm().text_color(cx.theme().colors().text_muted).font_weight(gpui::FontWeight::SEMIBOLD).child("Direction"))
                            .child(
                                div()
                                    .flex()
                                    .gap_8()
                                    .items_center()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "clockwise",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .direction(CircleDirection::Clockwise),
                                            )
                                            .child(Label::new("Clockwise")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "counterclockwise",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .direction(CircleDirection::CounterClockwise),
                                            )
                                            .child(Label::new("Counter-clockwise")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(div().text_sm().text_color(cx.theme().colors().text_muted).font_weight(gpui::FontWeight::SEMIBOLD).child("Stroke Width"))
                            .child(
                                div()
                                    .flex()
                                    .gap_8()
                                    .items_center()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "thin",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .stroke_width(gpui::px(2.)),
                                            )
                                            .child(Label::new("Thin (2px)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "normal",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .stroke_width(gpui::px(4.)),
                                            )
                                            .child(Label::new("Normal (4px)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "thick",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .stroke_width(gpui::px(8.)),
                                            )
                                            .child(Label::new("Thick (8px)")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(div().text_sm().text_color(cx.theme().colors().text_muted).font_weight(gpui::FontWeight::SEMIBOLD).child("Start Angle"))
                            .child(
                                div()
                                    .flex()
                                    .gap_8()
                                    .items_center()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "top",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .start_angle(-90.),
                                            )
                                            .child(Label::new("Top (-90°)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "right",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .start_angle(0.),
                                            )
                                            .child(Label::new("Right (0°)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "bottom",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .start_angle(90.),
                                            )
                                            .child(Label::new("Bottom (90°)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "left",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .start_angle(180.),
                                            )
                                            .child(Label::new("Left (180°)")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(div().text_sm().text_color(cx.theme().colors().text_muted).font_weight(gpui::FontWeight::SEMIBOLD).child("Color Customization"))
                            .child(
                                div()
                                    .flex()
                                    .gap_8()
                                    .items_center()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "success",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .fg_color(cx.theme().status().success),
                                            )
                                            .child(Label::new("Success")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "warning",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .fg_color(cx.theme().status().warning),
                                            )
                                            .child(Label::new("Warning")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                CircleProgressBar::new(
                                                    "error",
                                                    max_value * 0.6,
                                                    max_value,
                                                    cx,
                                                )
                                                    .fg_color(cx.theme().status().error),
                                            )
                                            .child(Label::new("Error")),
                                    ),
                            ),
                    ),
            )
    }
}
