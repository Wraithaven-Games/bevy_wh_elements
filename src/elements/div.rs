use bevy::prelude::*;

use crate::prelude::{AssetReference, BoxedElement, WhElement, WhNode};
use crate::{build_children_field, build_node_field};

pub struct WhDiv<Flags: Bundle> {
    pub flags: Flags,
    pub node: WhNode,
    pub children: Vec<BoxedElement>,
}

impl WhDiv<()> {
    pub fn new() -> Box<Self> {
        Self::with_flags(())
    }
}

impl<Flags: Bundle> WhDiv<Flags> {
    build_node_field!(node);
    build_children_field!(children);

    pub fn with_flags(flags: Flags) -> Box<Self> {
        Box::new(Self {
            flags,
            node: Default::default(),
            children: Default::default(),
        })
    }
}

impl<Flags: Bundle> WhElement for WhDiv<Flags> {
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetReference,
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
