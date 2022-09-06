pub enum Justify {
    HCentre(u32),
    VCentre(u32),
    Left(u32),
    Right(u32),
    Top(u32),
    Bottom(u32),
    TopLeft,
    TopCentre,
    TopRight,
    CentreLeft,
    Centre,
    CentreRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

pub enum Align {
    TopLeft,
    TopCentre,
    TopRight,
    CentreLeft,
    Centre,
    CentreRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

pub fn align(
    a: Align,
    follower_height: usize,
    follower_width: usize,
    anchor_y: u32,
    anchor_x: u32,
    anchor_height: usize,
    anchor_width: usize
) -> (u32, u32)
{
    let y: u32;
    let x: u32;

    match a {
        Align::TopLeft => {
            x = anchor_x;
            y = anchor_y;
        },
        Align::TopCentre => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32 / 2;
            }

            y = anchor_y;
        },
        Align::TopRight => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32;
            }

            y = anchor_y;
        },
        Align::CentreLeft => {
            x = anchor_x;

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32 / 2;
            }
        },
        Align::Centre => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32 / 2;
            }

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32 / 2;
            }
        },
        Align::CentreRight => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32;
            }

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32 / 2;
            }
        },
        Align::BottomLeft => {
            x = anchor_x;

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32;
            }
        },
        Align::BottomCentre => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32 / 2;
            }

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32;
            }
        },
        Align::BottomRight => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32;
            }

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32;
            }
        },
    }

    (y, x)
}

pub trait Aligned {
    fn inner_width(&self) -> usize;
    fn inner_height(&self) -> usize;
    fn inner_start_yx(&self) -> (u32, u32);
    fn outer_width(&self) -> usize;
    fn outer_height(&self) -> usize;
    fn outer_start_yx(&self) -> (u32, u32);
    fn centre(&self) -> (u32, u32);
    fn align_centres<T: Aligned>(&mut self, anchor: &T);
    fn align_to_inner<T: Aligned>(&mut self, anchor: &T, a: Align);
    fn align_to_outer<T: Aligned>(&mut self, anchor: &T, a: Align);
    fn adjust_pos(&mut self, y: i32, x: i32);
    fn change_pos(&mut self, y: u32, x: u32);
}

#[macro_export]
macro_rules! sub_impl_aligned {
    ($data_type:ty, $sub_impl:ident) => {
        impl Aligned for $data_type {
            fn inner_width(&self) -> usize { self.$sub_impl.inner_width() }
            fn inner_height(&self) -> usize { self.$sub_impl.inner_height() }
            fn inner_start_yx(&self) -> (u32, u32) { self.$sub_impl.inner_start_yx() }
            fn outer_width(&self) -> usize { self.$sub_impl.outer_width() }
            fn outer_height(&self) -> usize { self.$sub_impl.outer_height() }
            fn outer_start_yx(&self) -> (u32, u32) { self.$sub_impl.outer_start_yx() }
            fn centre(&self) -> (u32, u32) { self.$sub_impl.centre() }
            fn align_centres<T: Aligned>(&mut self, anchor: &T) { self.$sub_impl.align_centres(anchor); }
            fn align_to_inner<T: Aligned>(&mut self, anchor: &T, a: Align) { self.$sub_impl.align_to_inner(anchor, a); }
            fn align_to_outer<T: Aligned>(&mut self, anchor: &T, a: Align) { self.$sub_impl.align_to_outer(anchor, a); }
            fn adjust_pos(&mut self, y: i32, x: i32) { self.$sub_impl.adjust_pos(y, x); }
            fn change_pos(&mut self, y: u32, x: u32) { self.$sub_impl.change_pos(y, x); }
        }
    };

    ($data_type:ty, $sub_impl:ident, [$($sub_item:ident),+]) => {
        impl Aligned for $data_type {
            fn inner_width(&self) -> usize { self.$sub_impl.inner_width() }
            fn inner_height(&self) -> usize { self.$sub_impl.inner_height() }
            fn inner_start_yx(&self) -> (u32, u32) { self.$sub_impl.inner_start_yx() }
            fn outer_width(&self) -> usize { self.$sub_impl.outer_width() }
            fn outer_height(&self) -> usize { self.$sub_impl.outer_height() }
            fn outer_start_yx(&self) -> (u32, u32) { self.$sub_impl.outer_start_yx() }
            fn centre(&self) -> (u32, u32) { self.$sub_impl.centre() }

            fn align_centres<T: Aligned>(&mut self, anchor: &T)
            {
                let (wy, wx) = self.$sub_impl.outer_start_yx();
                self.$sub_impl.align_centres(anchor);
                let (new_wy, new_wx) = self.$sub_impl.outer_start_yx();
                let dy: i32 = new_wy as i32 - wy as i32;
                let dx: i32 = new_wx as i32 - wx as i32;
                $(
                    self.$sub_item.adjust_pos(dy, dx);
                )*
            }

            fn align_to_inner<T: Aligned>(&mut self, anchor: &T, a: Align)
            {
                let (wy, wx) = self.$sub_impl.outer_start_yx();
                self.$sub_impl.align_to_inner(anchor, a);
                let (new_wy, new_wx) = self.$sub_impl.outer_start_yx();
                let dy: i32 = new_wy as i32 - wy as i32;
                let dx: i32 = new_wx as i32 - wx as i32;
                $(
                    self.$sub_item.adjust_pos(dy, dx);
                )*
            }

            fn align_to_outer<T: Aligned>(&mut self, anchor: &T, a: Align)
            {
                let (wy, wx) = self.$sub_impl.outer_start_yx();
                self.$sub_impl.align_to_outer(anchor, a);
                let (new_wy, new_wx) = self.$sub_impl.outer_start_yx();
                let dy: i32 = new_wy as i32 - wy as i32;
                let dx: i32 = new_wx as i32 - wx as i32;
                $(
                    self.$sub_item.adjust_pos(dy, dx);
                )*
            }

            fn adjust_pos(&mut self, y: i32, x: i32)
            {
                self.$sub_impl.adjust_pos(y, x);
                $(
                    self.$sub_item.adjust_pos(y, x);
                )*
            }

            fn change_pos(&mut self, y: u32, x: u32)
            {
                let (wy, wx) = self.$sub_impl.outer_start_yx();
                self.$sub_impl.change_pos(y, x);
                let (new_wy, new_wx) = self.$sub_impl.outer_start_yx();
                let dy: i32 = new_wy as i32 - wy as i32;
                let dx: i32 = new_wx as i32 - wx as i32;
                $(
                    self.$sub_item.adjust_pos(dy, dx);
                )*
            }
        }
    }
}
