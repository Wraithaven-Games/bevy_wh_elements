use bevy::prelude::*;

use crate::prelude::{BoxedElement, WhElement, WhNode};
use crate::{build_children_field, build_node_field};

pub struct WhScreen<Flags: Bundle> {
    pub flags: Flags,
    pub node: WhNode,
    pub children: Vec<BoxedElement>,
}

impl<Flags: Bundle> WhScreen<Flags> {
    build_node_field!();
    build_children_field!();

    pub fn new(flags: Flags) -> Box<Self> {
        Box::new(Self {
            flags,
            node: WhNode {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            children: Default::default(),
        })
    }
}

impl<Flags: Bundle> WhElement for WhScreen<Flags> {
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &AssetServer,
        parent: Option<Entity>,
    ) {
        let mut cmd = self.node.build_entity(commands, loader);
        cmd.insert(self.flags);

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let entity = cmd.id();
        for child in self.children {
            child.build_child(commands, loader, Some(entity));
        }
    }
}
