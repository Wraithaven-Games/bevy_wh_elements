use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::prelude::{
    AssetReference,
    BoxedElement,
    ElementAlignment,
    ElementDirection,
    NodeInteraction,
    ScrollDirection,
    ScrollPane,
    WhElement,
    WhNode,
};
use crate::{build_children_field, build_node_field};

pub struct WhScrollPane<ContainerFlags: Bundle, PanelFlags: Bundle> {
    pub container_flags: ContainerFlags,
    pub panel_flags: PanelFlags,
    pub node: WhNode,
    pub children: Vec<BoxedElement>,
    pub scroll_direction: ScrollDirection,
}

impl WhScrollPane<(), ()> {
    pub fn new() -> Box<Self> {
        Self::with_flags((), ())
    }
}

impl<ContainerFlags: Bundle, PanelFlags: Bundle> WhScrollPane<ContainerFlags, PanelFlags> {
    build_node_field!(node);
    build_children_field!(children);

    pub fn with_flags(container_flags: ContainerFlags, panel_flags: PanelFlags) -> Box<Self> {
        Box::new(Self {
            container_flags,
            panel_flags,
            node: WhNode {
                no_wrap: true,
                ..default()
            },
            children: Default::default(),
            scroll_direction: Default::default(),
        })
    }

    pub fn scroll_direction(mut self: Box<Self>, scroll_direction: ScrollDirection) -> Box<Self> {
        self.scroll_direction = scroll_direction;
        self
    }

    fn container_style(&self) -> Style {
        Style {
            flex_direction: match self.scroll_direction {
                ScrollDirection::Vertical => FlexDirection::Column,
                ScrollDirection::Horizontal => FlexDirection::Row,
                ScrollDirection::Both => FlexDirection::Column,
            },
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            flex_grow: self.node.flex_grow,
            width: self.node.width,
            height: self.node.height,
            margin: self.node.margin,
            aspect_ratio: self.node.aspect_ratio,
            overflow: Overflow::clip(),
            border: UiRect::all(self.node.border_thickness),
            ..default()
        }
    }

    fn panel_style(&self) -> Style {
        Style {
            flex_direction: match self.node.direction {
                ElementDirection::Row => FlexDirection::Row,
                ElementDirection::Column => FlexDirection::Column,
            },
            justify_content: match self.node.justify {
                ElementAlignment::Left => JustifyContent::FlexStart,
                ElementAlignment::Center => JustifyContent::Center,
                ElementAlignment::Right => JustifyContent::FlexEnd,
            },
            align_content: match self.node.alignment {
                ElementAlignment::Left => AlignContent::FlexStart,
                ElementAlignment::Center => AlignContent::Center,
                ElementAlignment::Right => AlignContent::FlexEnd,
            },
            flex_wrap: match self.node.no_wrap {
                true => FlexWrap::NoWrap,
                false => FlexWrap::Wrap,
            },
            row_gap: self.node.gap,
            column_gap: self.node.gap,
            padding: self.node.padding,
            width: match self.scroll_direction {
                ScrollDirection::Vertical => Val::Percent(100.0),
                ScrollDirection::Horizontal => Val::Auto,
                ScrollDirection::Both => Val::Auto,
            },
            height: match self.scroll_direction {
                ScrollDirection::Vertical => Val::Auto,
                ScrollDirection::Horizontal => Val::Percent(100.0),
                ScrollDirection::Both => Val::Auto,
            },
            align_self: AlignSelf::Stretch,
            ..default()
        }
    }
}

impl<ContainerFlags: Bundle, PanelFlags: Bundle> WhElement
    for WhScrollPane<ContainerFlags, PanelFlags>
{
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetReference,
        parent: Option<Entity>,
    ) {
        let interaction_clone = self.node.clone();
        let container_style = self.container_style();
        let panel_style = self.panel_style();

        let mut container_cmd = match self.node.bg_img {
            None => commands.spawn(NodeBundle {
                style: container_style,
                background_color: self.node.bg_color.into(),
                border_color: self.node.border_color.into(),
                ..default()
            }),
            Some(bg_path) => commands.spawn(ImageBundle {
                style: container_style,
                background_color: self.node.bg_color.into(),
                image: loader.load(bg_path).into(),
                ..default()
            }),
        };
        let container_id = container_cmd.id();
        if let Some(parent) = parent {
            container_cmd.set_parent(parent);
        }

        let mut panel_cmd = commands.spawn((
            self.panel_flags,
            ScrollPane::default(),
            NodeBundle {
                style: panel_style,
                focus_policy: match self.node.interaction {
                    NodeInteraction::None => FocusPolicy::Pass,
                    _ => FocusPolicy::Block,
                },
                ..default()
            },
        ));
        interaction_clone.insert_interaction(&mut panel_cmd);
        panel_cmd.set_parent(container_id);

        let panel_id = panel_cmd.id();
        for child in self.children {
            child.build_child(commands, loader, Some(panel_id));
        }
    }
}
