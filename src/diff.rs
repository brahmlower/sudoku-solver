
pub trait PatchFragment {
    type Fragment;

    fn apply_fragment(&mut self, fragment: &Self::Fragment);
    fn revert_fragment(&mut self, fragment: &Self::Fragment);
}

impl <T: Sized + Copy> PatchFragment for T {
    type Fragment = ScalarDiffFragment<T>;

    fn apply_fragment(&mut self, fragment: &Self::Fragment) {
        *self = *fragment.after();
    }

    fn revert_fragment(&mut self, fragment: &Self::Fragment) {
        *self = *fragment.before();
    }
}

pub trait PatchDiff {
    type Fragment;

    fn apply_diff(&mut self, diff: &Diff<Self::Fragment>);
    fn revert_diff(&mut self, diff: &Diff<Self::Fragment>);
}

impl<F, T: PatchFragment<Fragment = F>> PatchDiff for T {
    type Fragment = F;

    fn apply_diff(&mut self, diff: &Diff<Self::Fragment>) {
        diff
            .fragments()
            .into_iter()
            .for_each(|f| self.apply_fragment(f));
    }

    fn revert_diff(&mut self, diff: &Diff<Self::Fragment>) {
        diff
            .fragments()
            .into_iter()
            .for_each(|f| self.revert_fragment(f));
    }
}

pub struct DiffBuilder<F> {
    fragments: Vec<F>
}

impl <F> DiffBuilder<F> {
    pub fn new() -> DiffBuilder<F> {
        DiffBuilder::default()
    }

    pub fn add_fragment<C>(&mut self, callback: C) -> &mut DiffBuilder<F>
    where
        C: Fn() -> F,
    {
        self.fragments.push(callback());
        self
    }

    pub fn finalize(&mut self) -> Diff<F> {
        let s = std::mem::take(self);
        Diff::new(s.fragments)
    }
}

impl<F> Default for DiffBuilder<F> {
    fn default() -> Self {
        DiffBuilder { fragments: vec![] }
    }
}

#[derive(Debug)]
pub struct Diff<F> {
    fragments: Vec<F>
}

impl<F> Diff<F> {
    pub fn new(fragments: Vec<F>) -> Diff<F> {
        Diff { fragments }
    }

    pub fn builder() -> DiffBuilder<F> {
        DiffBuilder::new()
    }

    pub fn fragments(&self) -> &Vec<F> {
        &self.fragments
    }
}

#[derive(Debug)]
pub struct ScalarDiffFragment<T> {
    value: [T; 2]
}

impl <T> ScalarDiffFragment<T> {
    pub fn new(before: T, after: T) -> ScalarDiffFragment<T> {
        ScalarDiffFragment { value: [before, after] }
    }

    pub fn before(&self) -> &T {
        &self.value[0]
    }

    pub fn after(&self) -> &T {
        &self.value[1]
    }
}

trait ScalarDiff: Sized + PatchFragment {
    fn mut_and_diff(&mut self, value: Self) -> Diff<ScalarDiffFragment<Self>>;
}

impl <T: Copy + Sized + PatchFragment<Fragment = ScalarDiffFragment<T>>> ScalarDiff for T {
    fn mut_and_diff(&mut self, value: T) -> Diff<ScalarDiffFragment<T>> {
        let fragment: ScalarDiffFragment<T> = ScalarDiffFragment::new(*self, value);
        let diff: Diff<ScalarDiffFragment<T>> = Diff::new(vec![fragment]);
        self.apply_diff(&diff);
        diff
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::{ScalarDiff, PatchDiff};

    #[test]
    fn creates_diff_for_u8() {
        let mut start: u8 = 1;

        let diff = start.mut_and_diff(2u8);
        assert_eq!(start, 2u8);

        start.revert_diff(&diff);
        assert_eq!(start, 1u8);

        start.apply_diff(&diff);
        assert_eq!(start, 2u8);
    }

    #[test]
    fn creates_diff_for_i32() {
        let mut start: i32 = 9001;

        let diff = start.mut_and_diff(123);
        assert_eq!(start, 123);

        start.revert_diff(&diff);
        assert_eq!(start, 9001);

        start.apply_diff(&diff);
        assert_eq!(start, 123);
    }

    #[test]
    fn creates_diff_for_str() {
        let mut start: &str = "foo bar baz";

        let diff = start.mut_and_diff("qux corge");
        assert_eq!(start, "qux corge");

        start.revert_diff(&diff);
        assert_eq!(start, "foo bar baz");

        start.apply_diff(&diff);
        assert_eq!(start, "qux corge");
    }

    // #[test]
    // fn creates_diff_for_vec_i32() {
    //     let mut start: Vec<i32> = vec![1, 2, 3];

    //     let diff = start.mut_and_diff(vec![4, 5, 6]);
    //     assert_eq!(start, vec![4, 5, 6]);

    //     start.revert_diff(&diff);
    //     assert_eq!(start, "foo bar baz");

    //     start.apply_diff(&diff);
    //     assert_eq!(start, "qux corge");
    // }


}