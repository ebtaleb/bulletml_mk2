pub mod bmltree {

    #[derive(Debug, Copy, Clone)]
    pub enum Type {
        None,
        Aim,
        Absolute,
        Relative,
        Sequence,
        TypeSize,
        Horizontal,
        Vertical
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Name {
        Bullet, Action, Fire, ChangeDirection, ChangeSpeed, Accel,
        Wait, Repeat, BulletRef, ActionRef, FireRef, Vanish,
        Horizontal, Vertical, Term, Times, Direction, Speed, Param,
        Bulletml, NameSize,
        None
    }

    #[derive(Debug, Clone)]
    struct BulletMLNode {
        name : Name,
        label : Option<String>,
        type_ : Type,
        ref_id : i32,
        val : f32
    }

    #[derive(Debug, Clone)]
    pub struct TreeNode {
        data: BulletMLNode,
        children : Vec<Box<TreeNode>>,
        parent : Option<&'static TreeNode>
    }

    impl TreeNode {
        pub fn new() -> TreeNode {
            TreeNode { data: BulletMLNode { name : Name::None, label : None, type_ : Type::None, ref_id : -1, val : -1.0 },
                       children: Vec::new(),
                       parent : None }
        }

        pub fn new_box(n : &str) -> Box<TreeNode> {
            Box::new( TreeNode { data: BulletMLNode { name : string2name(n), label : None, type_ : Type::None, ref_id : -1, val : -1.0 },
                                 children: Vec::new(),
                                 parent : None } )
        }

        pub fn set_parent(&mut self, node: &'static TreeNode) {
            self.parent = Some(node);
        }

        pub fn has_parent(&self) -> bool {
                match self.parent {
                    None => false,
                    _ => true
                }
        }

        pub fn add_child(&'static mut self, mut child : Box<TreeNode>) {
            child.set_parent(self);
            self.children.push(child);
        }

        pub fn get_name(&self) -> Name { return self.data.name }

        pub fn set_value(&mut self, val: f32) { self.data.val = val }
        pub fn get_value(&self) -> f32 { return self.data.val }

        pub fn set_type(&mut self, _type: &str) { self.data.type_ = string2type(_type); }
        pub fn get_type(&self) -> Type { return self.data.type_ }

        pub fn set_ref_id(&mut self, id: i32) { self.data.ref_id = id }
        pub fn get_ref_id(&self) -> i32 { return self.data.ref_id }

        pub fn get_child(&self, name: Name) -> Option<Box<TreeNode>> {

            for i in self.children.to_vec() {
                if (*i).get_name() == name {
                    return Some(i)
                }
            }

            return None
        }

        pub fn find_node(&self, name : Name) -> bool {
            if self.get_name() == name {
                return true
            }

            for i in self.children.to_vec() {
                if (*i).find_node(name) {
                    return true
                }
            }

            return false
        }

        pub fn get_all_children_vec(&self, name : Name, outvec : &mut Vec<Box<TreeNode>>) {

            for i in self.children.to_vec() {
                if i.get_name() == name {
                    outvec.push(i)
                }
            }
        }

    }

    pub fn string2type(s : &str) -> Type {
        match s {
            "aim"        => Type::Aim,
            "absolute"   => Type::Absolute,
            "relative"   => Type::Relative,
            "sequence"   => Type::Sequence,
            "horizontal" => Type::Horizontal,
            "vertical"   => Type::Vertical,
            "none"       => Type::None,
            _            => { assert!(false, "BulletML parser: unknown type {}.", s); Type::None }
        }
    }

    pub fn string2name(s : &str) -> Name {
        match s {
            "bulletml"          => Name::Bulletml,
            "bullet"            => Name::Bullet,
            "action"            => Name::Action,
            "fire"              => Name::Fire,
            "changeDirection"   => Name::ChangeDirection,
            "changeSpeed"       => Name::ChangeSpeed,
            "accel"             => Name::Accel,
            "vanish"            => Name::Vanish,
            "wait"              => Name::Wait,
            "repeat"            => Name::Repeat,
            "direction"         => Name::Direction,
            "speed"             => Name::Speed,
            "horizontal"        => Name::Horizontal,
            "vertical"          => Name::Vertical,
            "term"              => Name::Term,
            "bulletRef"         => Name::BulletRef,
            "actionRef"         => Name::ActionRef,
            "fireRef"           => Name::FireRef,
            "param"             => Name::Param,
            "times"             => Name::Times,
            "none"              => Name::None,
            _                   => { assert!(false, "BulletML parser: unknown tag {}.", s); Name::None }
        }
    }

}

