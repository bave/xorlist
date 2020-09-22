use std::iter::Iterator;
use std::marker::PhantomData;

pub struct XorList<T>
{
    head: LINK<T>,
    tail: LINK<T>,
    size: usize
}

type LINK<T> = *mut Node<T>;
pub struct Node<T> {
    xor: LINK<T>,
    val: Option<T>
}

impl<T> XorList<T>
{
    pub fn new() -> Self
    {
        let ht = unsafe {
            let mut h = Self::new_node();
            let mut t = Self::new_node();
            (*h).xor = Self::xorptr(std::ptr::null_mut(), t);
            (*t).xor = Self::xorptr(h, std::ptr::null_mut());
            (h, t)
        };
        Self{
            head: ht.0,
            tail: ht.1,
            size: 0
        }
    }

    pub fn push_front(&mut self, val: T)
    {
        let fb = unsafe {
            let f = self.head;
            let b = Self::xorptr(std::ptr::null_mut(), (*f).xor);
            (f, b)
        };
        self.add(fb.0, fb.1, val);
    }

    pub fn push_back(&mut self, val: T)
    {
        let fb = unsafe {
            let b = self.tail;
            let f = Self::xorptr((*b).xor, std::ptr::null_mut());
            (f, b)
        };
        self.add(fb.0, fb.1, val);
    }

    pub fn pop_front(&mut self) -> Option<T>
    {
        match self.size {
            0 => {
                None
            },
            _ => unsafe {
                self.size -= 1;
                let mut f = self.head;
                let c = Self::xorptr(std::ptr::null_mut(), (*f).xor);
                let mut b = Self::xorptr(f, (*c).xor);
                (*b).xor = Self::xorptr(Self::xorptr(f, c), (*b).xor);
                (*f).xor = Self::xorptr(Self::xorptr(c, b), (*f).xor);
                (*Box::from_raw(c)).val
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T>
    {
        match self.size {
            0 => {
                None
            },
            _ => {
                self.size -= 1;
                unsafe {
                    let mut b = self.tail;
                    let c = Self::xorptr((*b).xor, std::ptr::null_mut());
                    let mut f = Self::xorptr((*c).xor, b);
                    (*b).xor = Self::xorptr(Self::xorptr(f, c), (*b).xor);
                    (*f).xor = Self::xorptr(Self::xorptr(c, b), (*f).xor);
                    (*Box::from_raw(c)).val
                }
            }
        }
    }

    fn new_node() -> LINK<T>
    {
        Box::into_raw(Box::new(Node{xor: 0 as LINK<T>, val: None}))
    }

    fn xorptr(a: LINK<T>, b: LINK<T>) -> LINK<T>
    {
        ((a as usize) ^ (b as usize)) as LINK<T>
    }

    fn add(&mut self, f: LINK<T>, b: LINK<T>, val: T)
    {
        let n = unsafe {
            let mut n = Self::new_node();
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

    pub fn iter(&self) -> XorListIter<T>
    {
        let pre = self.head;
        let cur = unsafe {Self::xorptr(std::ptr::null_mut(), (*pre).xor)};
        XorListIter {
            cur: Some((pre, cur)),
            _ph : std::marker::PhantomData
        }
    }
}

pub struct XorListIter<'a, T>
{
    cur : Option<(LINK<T>, LINK<T>)>,
    _ph : PhantomData<&'a T>
}

impl<'a, T> Iterator for XorListIter<'a, T>
{
    type Item = &'a T;
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
                    match (*n.1).val {
                        None => { self.cur = None; None }
                        _ => { (*n.1).val.as_ref() }
                    }
                }
            }
        }
    }
}

impl<T> Drop for XorList<T>
{
    fn drop(&mut self)
    {
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

impl<T: Clone> Iterator for XorListIterator<T>
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
    fn init() -> Result<(), String>
    {
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
    fn iter() -> Result<(), String>
    {
        let mut xl = XorList::new() as XorList<u64>;
        xl.push_back(10);
        xl.push_back(20);
        xl.push_back(30);
        let mut it = xl.iter();
        println!("{}", it.next().unwrap());
        println!("{}", it.next().unwrap());
        println!("{}", it.next().unwrap());
        let mut it = xl.iter();
        println!("{:?}", it.next());
        println!("{:?}", it.next());
        println!("{:?}", it.next());
        println!("{:?}", it.next());
        println!("{:?}", it.next());
        Ok(())
    }
    /*
    #[test]
    fn iter() -> Result<(), String>
    {
        let mut xl = XorList::new() as XorList<u64>;
        xl.push_back(10);
        xl.push_back(20);
        xl.push_back(30);
        let mut it = xl.iter();
        let mut n = it.next().unwrap();
        let mut v = unsafe {(*n).val.unwrap()};
        println!("{}", v);
        n = it.next().unwrap();
        v = unsafe {(*n).val.unwrap()};
        println!("{}", v);
        Ok(())
    }
    */
}

