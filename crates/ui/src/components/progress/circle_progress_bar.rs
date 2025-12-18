use documented::Documented;
use gpui::{Hsla, PathBuilder, canvas, point};
use std::f32::consts::PI;

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CircleSize {
    XSmall,
    Small,
    Medium,
    Large,
    Custom(Pixels),
}

impl CircleSize {
    fn diameter(self) -> Pixels {
        match self {
            CircleSize::XSmall => px(12.),
            CircleSize::Small => px(14.),
            CircleSize::Medium => px(16.),
            CircleSize::Large => px(20.),
            CircleSize::Custom(size) => size,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CircleDirection {
    Clockwise,
    CounterClockwise,
}

/// A circular progress bar that displays progress as an arc around a circle.
///
/// The progress arc can be customized with different sizes, stroke widths,
/// colors, start angles, and directions. The center is empty by design.
#[derive(IntoElement, RegisterComponent, Documented)]
pub struct CircleProgressBar {
    id: ElementId,
    value: f32,
    max_value: f32,
    size: CircleSize,
    stroke_width: Pixels,
    bg_color: Hsla,
    fg_color: Hsla,
    over_color: Hsla,
    start_angle: f32,
    direction: CircleDirection,
}

impl CircleProgressBar {
    pub fn new(id: impl Into<ElementId>, value: f32, max_value: f32, cx: &App) -> Self {
        Self {
            id: id.into(),
            value,
            max_value,
            size: CircleSize::Medium,
            stroke_width: px(4.),
            bg_color: cx.theme().colors().border_variant,
            fg_color: cx.theme().status().info,
            over_color: cx.theme().status().error,
            start_angle: -90.,
            direction: CircleDirection::Clockwise,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;

        self
    }

    pub fn max_value(mut self, max_value: f32) -> Self {
        self.max_value = max_value;

        self
    }

    pub fn size(mut self, size: CircleSize) -> Self {
        self.size = size;

        self
    }

    pub fn stroke_width(mut self, width: Pixels) -> Self {
        self.stroke_width = width;

        self
    }

    pub fn bg_color(mut self, color: Hsla) -> Self {
        self.bg_color = color;

        self
    }

    pub fn fg_color(mut self, color: Hsla) -> Self {
        self.fg_color = color;

        self
    }

    pub fn over_color(mut self, color: Hsla) -> Self {
        self.over_color = color;

        self
    }

    pub fn start_angle(mut self, angle: f32) -> Self {
        self.start_angle = angle;

        self
    }

    pub fn direction(mut self, direction: CircleDirection) -> Self {
        self.direction = direction;

        self
    }

    fn angle_to_point(
        center: gpui::Point<Pixels>,
        radius: Pixels,
        angle_degrees: f32,
    ) -> gpui::Point<Pixels> {
        let angle_radians = angle_degrees * PI / 180.;

        let x = center.x + radius * angle_radians.cos();
        let y = center.y + radius * angle_radians.sin();

        point(x, y)
    }

    fn normalized_progress(&self) -> f32 {
        (self.value / self.max_value).clamp(0.02, 1.0)
    }
}

impl RenderOnce for CircleProgressBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let diameter = self.size.diameter();
        let radius = diameter / 2.;
        let stroke_width = self.stroke_width;
        let bg_color = self.bg_color;
        let fg_color = self.fg_color;
        let over_color = self.over_color;
        let value = self.value;
        let max_value = self.max_value;
        let start_angle = self.start_angle;
        let direction = self.direction;
        let progress = self.normalized_progress();

        div()
            .id(self.id)
            .flex_none()
            .size(diameter)
            .relative()
            .child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, _cx| {
                        let center = point(
                            bounds.origin.x + bounds.size.width / 2.,
                            bounds.origin.y + bounds.size.height / 2.,
                        );
                        let effective_radius = radius - stroke_width / 2.;

                        {
                            let mut builder = PathBuilder::stroke(stroke_width);
                            let start_point = point(center.x + effective_radius, center.y);
                            builder.move_to(start_point);

                            builder.arc_to(
                                point(effective_radius, effective_radius),
                                px(0.),
                                false,
                                true,
                                point(center.x - effective_radius, center.y),
                            );

                            builder.arc_to(
                                point(effective_radius, effective_radius),
                                px(0.),
                                false,
                                true,
                                point(center.x + effective_radius, center.y),
                            );

                            if let Ok(path) = builder.build() {
                                window.paint_path(path, bg_color);
                            }
                        }

                        {
                            let color = if value > max_value {
                                over_color
                            } else {
                                fg_color
                            };

                            if progress >= 0.98 {
                                let mut builder = PathBuilder::stroke(stroke_width);
                                let start_point = point(center.x + effective_radius, center.y);
                                builder.move_to(start_point);

                                builder.arc_to(
                                    point(effective_radius, effective_radius),
                                    px(0.),
                                    false,
                                    true,
                                    point(center.x - effective_radius, center.y),
                                );

                                builder.arc_to(
                                    point(effective_radius, effective_radius),
                                    px(0.),
                                    false,
                                    true,
                                    point(center.x + effective_radius, center.y),
                                );

                                if let Ok(path) = builder.build() {
                                    window.paint_path(path, color);
                                }
                            } else {
                                let mut builder = PathBuilder::stroke(stroke_width);
                                let start_point =
                                    Self::angle_to_point(center, effective_radius, start_angle);
                                builder.move_to(start_point);

                                let end_angle = start_angle
                                    + (progress
                                        * 360.0
                                        * match direction {
                                            CircleDirection::Clockwise => 1.0,
                                            CircleDirection::CounterClockwise => -1.0,
                                        });
                                let end_point =
                                    Self::angle_to_point(center, effective_radius, end_angle);

                                let angle_span = (end_angle - start_angle).abs();
                                let large_arc = angle_span > 180.;
                                let sweep = matches!(direction, CircleDirection::Clockwise);

                                builder.arc_to(
                                    point(effective_radius, effective_radius),
                                    px(0.),
                                    large_arc,
                                    sweep,
                                    end_point,
                                );

                                if let Ok(path) = builder.build() {
                                    window.paint_path(path, color);
                                }
                            }
                        }
                    },
                )
                .absolute()
                .size_full(),
            )
    }
}

impl Component for CircleProgressBar {
    fn scope() -> ComponentScope {
        ComponentScope::Loading
    }

    fn description() -> Option<&'static str> {
        Some(Self::DOCS)
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let max_value = 100.0;

        Some(
            v_flex()
                .gap_6()
                .p_4()
                .child(div().child("Circular Progress Bars"))
                .child(
                    h_flex()
                        .gap_8()
                        .items_center()
                        .child(
                            v_flex()
                                .gap_2()
                                .items_center()
                                .child(Label::new("0% - Empty"))
                                .child(CircleProgressBar::new("empty", 0.0, max_value, cx)),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .items_center()
                                .child(Label::new("50% - Partial"))
                                .child(CircleProgressBar::new(
                                    "partial",
                                    max_value * 0.5,
                                    max_value,
                                    cx,
                                )),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .items_center()
                                .child(Label::new("100% - Complete"))
                                .child(CircleProgressBar::new("filled", max_value, max_value, cx)),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .items_center()
                                .child(Label::new("120% - Over-limit"))
                                .child(CircleProgressBar::new(
                                    "over",
                                    max_value * 1.2,
                                    max_value,
                                    cx,
                                )),
                        ),
                )
                .child(
                    h_flex()
                        .gap_8()
                        .items_center()
                        .child(Label::new("Size Variants"))
                        .child(
                            CircleProgressBar::new("xsmall", max_value * 0.6, max_value, cx)
                                .size(CircleSize::XSmall)
                                .stroke_width(px(2.)),
                        )
                        .child(
                            CircleProgressBar::new("small", max_value * 0.6, max_value, cx)
                                .size(CircleSize::Small),
                        )
                        .child(
                            CircleProgressBar::new("medium", max_value * 0.6, max_value, cx)
                                .size(CircleSize::Medium),
                        )
                        .child(
                            CircleProgressBar::new("large", max_value * 0.6, max_value, cx)
                                .size(CircleSize::Large),
                        ),
                )
                .child(
                    h_flex()
                        .gap_8()
                        .items_center()
                        .child(Label::new("Counter-clockwise"))
                        .child(
                            CircleProgressBar::new("ccw", max_value * 0.6, max_value, cx)
                                .direction(CircleDirection::CounterClockwise),
                        ),
                )
                .child(
                    h_flex()
                        .gap_8()
                        .items_center()
                        .child(Label::new("Custom stroke"))
                        .child(
                            CircleProgressBar::new("thick", max_value * 0.6, max_value, cx)
                                .stroke_width(px(8.)),
                        ),
                )
                .into_any_element(),
        )
    }
}
