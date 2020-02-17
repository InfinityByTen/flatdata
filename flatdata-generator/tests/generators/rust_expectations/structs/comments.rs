// Do not edit: This code was generated by flatdata's generator.
pub mod n {

pub mod schema {
pub mod structs {
pub const FOO: &str = r#"namespace n {
struct Foo
{
    a : u64 : 64;
    b : u64 : 64;
}
}

"#;
pub const BAR: &str = r#"namespace n {
struct Bar
{
    a : u64 : 64;
    b : u64 : 64;
}
}

"#;
}

}#[derive(Clone, Debug)]
pub struct Foo {}

// This is a comment about Foo
#[derive(Clone, Copy)]
pub struct FooRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for Foo
{
    const SCHEMA: &'static str = schema::structs::FOO;
    const SIZE_IN_BYTES: usize = 16;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = FooRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item{ data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = FooMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut{ data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for Foo {}

impl<'a> FooRef<'a> {
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for FooRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Foo")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for FooRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() &&        self.b() == other.b()     }
}

impl<'a> flatdata::Ref for FooRef<'a> {}

pub struct FooMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> FooMut<'a> {
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_a(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 16)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }

    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_b(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 16)
        };
        flatdata_write_bytes!(u64; value, buffer, 64, 64)
    }


    #[inline]
    pub fn fill_from(&mut self, other: &FooRef) {
        self.set_a(other.a());
        self.set_b(other.b());
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for FooMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        FooRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for FooMut<'a> {}
#[derive(Clone, Debug)]
pub struct Bar {}

/// This is a comment about Bar
#[derive(Clone, Copy)]
pub struct BarRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for Bar
{
    const SCHEMA: &'static str = schema::structs::BAR;
    const SIZE_IN_BYTES: usize = 16;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = BarRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item{ data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = BarMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut{ data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for Bar {}

impl<'a> BarRef<'a> {
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for BarRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Bar")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for BarRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() &&        self.b() == other.b()     }
}

impl<'a> flatdata::Ref for BarRef<'a> {}

pub struct BarMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> BarMut<'a> {
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_a(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 16)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }

    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_b(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 16)
        };
        flatdata_write_bytes!(u64; value, buffer, 64, 64)
    }


    #[inline]
    pub fn fill_from(&mut self, other: &BarRef) {
        self.set_a(other.a());
        self.set_b(other.b());
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for BarMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        BarRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for BarMut<'a> {}
}
