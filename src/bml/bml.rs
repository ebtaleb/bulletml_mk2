pub mod bml {

    use std::cell::RefCell;

    #[derive(Debug, PartialEq, Copy, Clone)]
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

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Name {
        Bullet, Action, Fire, ChangeDirection, ChangeSpeed, Accel,
        Wait, Repeat, BulletRef, ActionRef, FireRef, Vanish,
        Horizontal, Vertical, Term, Times, Direction, Speed, Param,
        Bulletml, NameSize,
        None
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct BulletMLData {
        name : Name,
        label : Option<String>,
        type_ : Type,
        ref_id : i32,
        val : f32
    }

    impl BulletMLData {
        pub fn new() -> BulletMLData {
            BulletMLData { name : Name::None, label: None, type_ : Type::None, ref_id : -1, val : -1.0 }
        }

        pub fn new_cell(n : &str) -> RefCell<BulletMLData> {
            RefCell::new(BulletMLData { name : string2name(n), label : None, type_ : Type::None, ref_id : -1, val : -1.0 })
        }

        pub fn get_name(&self) -> Name { return self.name.clone() }

        pub fn set_value(&mut self, val: f32) { self.val = val }
        pub fn get_value(&self) -> f32 { return self.val }

        pub fn set_type(&mut self, _type: &str) { self.type_ = string2type(_type); }
        pub fn get_type(&self) -> Type { return self.type_.clone() }

        pub fn set_label(&mut self, lab: &str) { if lab != "" { self.label = Some(lab.to_string()); } }
        pub fn get_label(&self) -> Option<String> { return self.label.clone() }

        pub fn set_ref_id(&mut self, id: i32) { self.ref_id = id }
        pub fn get_ref_id(&self) -> i32 { return self.ref_id }
    }

    #[derive(Debug)]
    pub struct Node<'a, T: 'a> {
        parent: RefCell<Option<&'a Node<'a, T>>>,
        children: RefCell<Vec<RefCell<Option<&'a Node<'a, T>>>>>,
        pub data: T,
    }

    fn same_ref<T>(a: &T, b: &T) -> bool {
        a as *const T == b as *const T
    }

    impl<'a, T> Node<'a, T> {

        pub fn new(data: T) -> Node<'a, T> {
            Node {
                parent: RefCell::new(None),
                children : RefCell::new(Vec::new()),
                data: data,
            }
        }

        pub fn parent(&self) -> Option<&'a Node<'a, T>> {
            *self.parent.borrow_mut()
        }

        pub fn same_node(&self, other: &Node<'a, T>) -> bool {
            same_ref(self, other)
        }

        pub fn add_children(&'a self, new_node: &'a Node<'a, T>) {
            self.children.borrow_mut().push(RefCell::new(Some(new_node)));
        }

        pub fn insert(&'a self, new_node: &'a Node<'a, T>, indent : i32) {
            if indent <= 1 {
                self.add_children(new_node);
            } else {
                self.insert(new_node, indent - 1)
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
            "none" | ""  => Type::None,
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
            "none" | ""         => Name::None,
            _                   => { assert!(false, "BulletML parser: unknown tag {}.", s); Name::None }
        }
    }

}

