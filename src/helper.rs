pub trait ContainsSlice {
    type Item: PartialEq;
    fn contains_slice(&'_ self, slice: &'_ [Self::Item]) -> bool;
}

impl<Item: PartialEq> ContainsSlice for [Item] {
    type Item = Item;

    fn contains_slice(self: &'_ [Item], slice: &'_ [Item]) -> bool {
        let len = slice.len();
        if len == 0 {
            return true;
        }
        self.windows(len).any(move |sub_slice| sub_slice == slice)
    }
}
