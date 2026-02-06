pub struct XorList<T> {
    head: LINK<T>,
    tail: LINK<T>,
    size: usize,
}

type LINK<T> = *mut Node<T>;
pub struct Node<T> {
    xor: LINK<T>,
    val: Option<T>,
}

impl<T> XorList<T> {
    pub fn new() -> Self {
        let ht = unsafe {
            let h = Self::new_node();
            let t = Self::new_node();
            (*h).xor = Self::xorptr(std::ptr::null_mut(), t);
            (*t).xor = Self::xorptr(h, std::ptr::null_mut());
            (h, t)
        };
        Self {
            head: ht.0,
            tail: ht.1,
            size: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let fb = unsafe {
            let f = self.head;
            let b = Self::xorptr(std::ptr::null_mut(), (*f).xor);
            (f, b)
        };
        self.add(fb.0, fb.1, val);
    }

    pub fn push_back(&mut self, val: T) {
        let fb = unsafe {
            let b = self.tail;
            let f = Self::xorptr((*b).xor, std::ptr::null_mut());
            (f, b)
        };
        self.add(fb.0, fb.1, val);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            _ => unsafe {
                self.size -= 1;
                let f = self.head;
                let c = Self::xorptr(std::ptr::null_mut(), (*f).xor);
                let b = Self::xorptr(f, (*c).xor);
                (*b).xor = Self::xorptr(Self::xorptr(f, c), (*b).xor);
                (*f).xor = Self::xorptr(Self::xorptr(c, b), (*f).xor);
                (*Box::from_raw(c)).val
            },
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            _ => {
                self.size -= 1;
                unsafe {
                    let b = self.tail;
                    let c = Self::xorptr((*b).xor, std::ptr::null_mut());
                    let f = Self::xorptr((*c).xor, b);
                    (*b).xor = Self::xorptr(Self::xorptr(f, c), (*b).xor);
                    (*f).xor = Self::xorptr(Self::xorptr(c, b), (*f).xor);
                    (*Box::from_raw(c)).val
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        match self.size {
            0 => true,
            _ => false,
        }
    }

    fn new_node() -> LINK<T> {
        Box::into_raw(Box::new(Node {
            xor: 0 as LINK<T>,
            val: None,
        }))
    }

    fn xorptr(a: LINK<T>, b: LINK<T>) -> LINK<T> {
        ((a as usize) ^ (b as usize)) as LINK<T>
    }

    fn add(&mut self, f: LINK<T>, b: LINK<T>, val: T) {
        let n = unsafe {
            let n = Self::new_node();
            (*n).val = Some(val);
            (*n).xor = Self::xorptr(f, b);
            n
        };

        unsafe {
            (*f).xor = Self::xorptr(Self::xorptr((*f).xor, b), n);
            (*b).xor = Self::xorptr(Self::xorptr(f, (*b).xor), n);
        };

        self.size += 1;
    }

    /*
    fn ftrack(&mut self)
    {
        unsafe {
            let mut pre = self.head;
            let mut cur = Self::xorptr(std::ptr::null_mut(), (*pre).xor);
            for _i in 0..self.size {
                let tmp = &mut *Self::xorptr(pre, (*cur).xor);
                pre = cur;
                cur = tmp;
            }
        };
    }

    fn btrack(&mut self)
    {
        unsafe {
            let mut nxt = self.tail;
            let mut cur = Self::xorptr((*nxt).xor, std::ptr::null_mut());
            for _i in 0..self.size {
                let tmp = &mut *Self::xorptr((*cur).xor, nxt);
                nxt = cur;
                cur = tmp;
            }
        };
    }
    */

    /*
    // from reference
    // https://doc.rust-lang.org/std/iter/#the-three-forms-of-iteration
    iter(), which iterates over &T.
    iter_mut(), which iterates over &mut T.
    into_iter(), which iterates over T.
    */
    pub fn iter(&self) -> XorListIter<'_, T> {
        let pre = self.head;
        let cur = unsafe { Self::xorptr(std::ptr::null_mut(), (*pre).xor) };
        XorListIter {
            cur: Some((pre, cur)),
            _ph: std::marker::PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> XorListIterMut<'_, T> {
        let pre = self.head;
        let cur = unsafe { Self::xorptr(std::ptr::null_mut(), (*pre).xor) };
        XorListIterMut {
            cur: Some((pre, cur)),
            _ph: std::marker::PhantomData,
        }
    }

    pub fn into_iter(self) -> XorListIterRaw<T> {
        XorListIterRaw { xorlist: self }
    }
}

pub struct XorListIterRaw<T> {
    xorlist: XorList<T>,
}

impl<T> std::iter::IntoIterator for XorList<T> {
    type Item = T;
    type IntoIter = XorListIterRaw<T>;
    fn into_iter(self) -> XorListIterRaw<T> {
        XorListIterRaw { xorlist: self }
    }
}

impl<T> std::iter::Iterator for XorListIterRaw<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.xorlist.pop_front()
    }
}

pub struct XorListIter<'a, T> {
    cur: Option<(LINK<T>, LINK<T>)>,
    _ph: std::marker::PhantomData<&'a T>,
}

impl<'a, T> std::iter::IntoIterator for &'a XorList<T> {
    type Item = &'a T;
    type IntoIter = XorListIter<'a, T>;
    fn into_iter(self) -> XorListIter<'a, T> {
        let pre = self.head;
        let cur = unsafe {
            ((std::ptr::null_mut() as LINK<T>) as usize ^ (*pre).xor as usize) as LINK<T>
        };
        XorListIter {
            cur: Some((pre, cur)),
            _ph: std::marker::PhantomData,
        }
    }
}

impl<'a, T> std::iter::Iterator for XorListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            None => None,
            Some(n) => unsafe {
                let p = n.0;
                let c = n.1;
                let t = ((p as usize) ^ ((*c).xor as usize)) as LINK<T>;
                self.cur = Some((c, t));
                match (*n.1).val {
                    None => {
                        self.cur = None;
                        None
                    }
                    _ => (*n.1).val.as_ref(),
                }
            },
        }
    }
}

pub struct XorListIterMut<'a, T> {
    cur: Option<(LINK<T>, LINK<T>)>,
    _ph: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> std::iter::IntoIterator for &'a mut XorList<T> {
    type Item = &'a mut T;
    type IntoIter = XorListIterMut<'a, T>;
    fn into_iter(self) -> XorListIterMut<'a, T> {
        let pre = self.head;
        let cur = unsafe {
            ((std::ptr::null_mut() as LINK<T>) as usize ^ (*pre).xor as usize) as LINK<T>
        };
        XorListIterMut {
            cur: Some((pre, cur)),
            _ph: std::marker::PhantomData,
        }
    }
}

impl<'a, T> std::iter::Iterator for XorListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            None => None,
            Some(n) => unsafe {
                let p = n.0;
                let c = n.1;
                let t = ((p as usize) ^ ((*c).xor as usize)) as LINK<T>;
                self.cur = Some((c, t));
                match (*n.1).val {
                    None => {
                        self.cur = None;
                        None
                    }
                    _ => (*n.1).val.as_mut(),
                }
            },
        }
    }
}

impl<T> Drop for XorList<T> {
    fn drop(&mut self) {
        while self.size != 0 {
            self.pop_front();
        }
    }
}

/*
// LINK<T> iter sample
pub struct XorListIter<T>
{
    cur : Option<(LINK<T>, LINK<T>)>
}

impl<T> Iterator for XorListIter<T>
{
    type Item = LINK<T>;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.cur {
            None => {
                None
            },
            Some(n) => {
                unsafe {
                    let p = n.0;
                    let c = n.1;
                    let t = ((p as usize) ^ ((*c).xor as usize)) as LINK<T>;
                    self.cur = Some((c, t));
                    Some(&mut (*n.1))
                }
            }
        }
    }
}
*/

/*
// Clone iter sample
pub struct XorListIterator<T>
{
    cur: Option<(LINK<T>, LINK<T>)>
}

impl<T: Clone> IntoIterator for XorList<T>
{
    type Item = T;
    type IntoIter = XorListIterator<T>;
    fn into_iter(self) -> Self::IntoIter
    {
        self.iter()
    }
}

impl<T: Clone> std::iter::Iterator for XorListIterator<T>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.cur.take()
        {
            None => None,
            Some(n) => {
                unsafe { (*(n.1)).val.clone() }
            }
        }
    }
}
*/

#[cfg(test)]
mod tests {

    use super::XorList;

    #[test]
    fn push_pop() -> Result<(), String> {
        let mut xl = XorList::new() as XorList<u64>;
        xl.push_front(0);
        xl.push_front(1);
        xl.push_back(2);
        if xl.pop_front().unwrap() != 1 {
            return Err(String::from("XorList::pop_front is worng.."));
        }
        if xl.pop_back().unwrap() != 2 {
            return Err(String::from("XorList::pop_back is worng.."));
        }
        Ok(())
    }

    #[test]
    fn iter() -> Result<(), String> {
        let mut xl = XorList::new() as XorList<u64>;
        for i in 0..1000 {
            xl.push_back(i)
        }
        for _ in &xl {}
        Ok(())
    }

    #[test]
    fn iter_mut() -> Result<(), String> {
        let mut xl = XorList::new() as XorList<u64>;
        for i in 0..1000 {
            xl.push_back(i)
        }
        for i in &mut xl {
            *i = *i + 1;
        }
        Ok(())
    }

    #[test]
    fn iter_into() -> Result<(), String> {
        let mut xl = XorList::new() as XorList<u64>;
        for i in 0..1000 {
            xl.push_back(i)
        }
        for _ in xl {}
        Ok(())
    }

    #[test]
    fn iter_methods() -> Result<(), String> {
        let mut xl = XorList::new() as XorList<u64>;
        for i in 0..5 {
            xl.push_back(i);
        }

        let xl_iter_map_collect = xl.iter().map(|x| x + 1).collect::<Vec<u64>>();
        print!("{:?}\n", xl_iter_map_collect);

        let xl_iter_mut_map_collect = xl.iter_mut().map(|x| *x + 1).collect::<Vec<u64>>();
        print!("{:?}\n", xl_iter_mut_map_collect);

        //xl moved ownership
        let xl_into_iter_map_collect = xl.into_iter().map(|x| x + 1).collect::<Vec<u64>>();
        print!("{:?}\n", xl_into_iter_map_collect);
        Ok(())
    }

    #[test]
    fn len() -> Result<(), String> {
        let mut xl = XorList::new() as XorList<u64>;
        assert!(xl.len() == 0);
        xl.push_back(0);
        assert!(xl.len() == 1);
        xl.push_back(1);
        assert!(xl.len() == 2);
        xl.push_back(2);
        assert!(xl.len() == 3);
        xl.push_back(3);
        assert!(xl.len() == 4);
        xl.push_back(4);
        assert!(xl.len() == 5);
        xl.push_back(5);
        assert!(xl.len() == 6);
        xl.push_back(6);
        assert!(xl.len() == 7);
        xl.push_back(7);
        assert!(xl.len() == 8);
        xl.push_back(8);
        assert!(xl.len() == 9);
        xl.push_back(9);
        xl.pop_front();
        assert!(xl.len() == 9);
        xl.pop_front();
        assert!(xl.len() == 8);
        xl.pop_front();
        assert!(xl.len() == 7);
        xl.pop_front();
        assert!(xl.len() == 6);
        xl.pop_front();
        assert!(xl.len() == 5);
        xl.pop_front();
        assert!(xl.len() == 4);
        xl.pop_front();
        assert!(xl.len() == 3);
        xl.pop_front();
        assert!(xl.len() == 2);
        xl.pop_front();
        assert!(xl.len() == 1);
        xl.pop_front();
        assert!(xl.len() == 0);
        assert!(xl.is_empty());
        Ok(())
    }
}
