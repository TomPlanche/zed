use gpui::{ClickEvent, Corner, CursorStyle, Entity, MouseButton};

use crate::{ContextMenu, PopoverMenu, prelude::*};

enum LabelKind {
    Text(SharedString),
    Element(AnyElement),
}

#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    label: LabelKind,
    menu: Entity<ContextMenu>,
    full_width: bool,
    disabled: bool,
}

impl DropdownMenu {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        menu: Entity<ContextMenu>,
    ) -> Self {
        Self {
            id: id.into(),
            label: LabelKind::Text(label.into()),
            menu,
            full_width: false,
            disabled: false,
        }
    }

    pub fn new_with_element(
        id: impl Into<ElementId>,
        label: AnyElement,
        menu: Entity<ContextMenu>,
    ) -> Self {
        Self {
            id: id.into(),
            label: LabelKind::Element(label),
            menu,
            full_width: false,
            disabled: false,
        }
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
}

impl Disableable for DropdownMenu {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        PopoverMenu::new(self.id)
            .full_width(self.full_width)
            .menu(move |_window, _cx| Some(self.menu.clone()))
            .trigger(
                DropdownMenuTrigger::new(self.label)
                    .full_width(self.full_width)
                    .disabled(self.disabled),
            )
            .attach(Corner::BottomLeft)
    }
}

#[derive(IntoElement)]
struct DropdownMenuTrigger {
    label: LabelKind,
    full_width: bool,
    selected: bool,
    disabled: bool,
    cursor_style: CursorStyle,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl DropdownMenuTrigger {
    pub fn new(label: LabelKind) -> Self {
        Self {
            label,
            full_width: false,
            selected: false,
            disabled: false,
            cursor_style: CursorStyle::default(),
            on_click: None,
        }
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
}

impl Disableable for DropdownMenuTrigger {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for DropdownMenuTrigger {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Clickable for DropdownMenuTrigger {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl RenderOnce for DropdownMenuTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled;

        h_flex()
            .id("dropdown-menu-trigger")
            .justify_between()
            .rounded_sm()
            .bg(cx.theme().colors().editor_background)
            .pl_2()
            .pr_1p5()
            .py_0p5()
            .gap_2()
            .min_w_20()
            .map(|el| {
                if self.full_width {
                    el.w_full()
                } else {
                    el.flex_none().w_auto()
                }
            })
            .map(|el| {
                if disabled {
                    el.cursor_not_allowed()
                } else {
                    el.cursor_pointer()
                }
            })
            .child(match self.label {
                LabelKind::Text(text) => Label::new(text)
                    .color(if disabled {
                        Color::Disabled
                    } else {
                        Color::Default
                    })
                    .into_any_element(),
                LabelKind::Element(element) => element,
            })
            .child(
                Icon::new(IconName::ChevronUpDown)
                    .size(IconSize::XSmall)
                    .color(if disabled {
                        Color::Disabled
                    } else {
                        Color::Muted
                    }),
            )
            .when_some(self.on_click.filter(|_| !disabled), |el, on_click| {
                el.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                    .on_click(move |event, window, cx| {
                        cx.stop_propagation();
                        (on_click)(event, window, cx)
                    })
            })
    }
}
