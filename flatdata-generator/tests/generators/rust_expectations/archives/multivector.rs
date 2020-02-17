// Do not edit: This code was generated by flatdata's generator.
pub mod n {

pub mod schema {
pub mod structs {
pub const S: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

"#;
pub const T: &str = r#"namespace n {
struct T
{
    x : u64 : 64;
}
}

"#;
}

pub mod a {

pub const A: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : multivector< 8, .n.S, .n.T >;
    @optional
    optional_data : multivector< 16, .n.S, .n.T >;
    data_u64_index : multivector< 64, .n.S, .n.T >;
}
}

"#;

pub mod resources {
pub const DATA: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : multivector< 8, .n.S, .n.T >;
}
}

"#;
pub const OPTIONAL_DATA: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    @optional
    optional_data : multivector< 16, .n.S, .n.T >;
}
}

"#;
pub const DATA_U64_INDEX: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data_u64_index : multivector< 64, .n.S, .n.T >;
}
}

"#;
}
}
}#[derive(Clone, Debug)]
pub struct S {}

#[derive(Clone, Copy)]
pub struct SRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for S
{
    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = SRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item{ data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = SMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut{ data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for S {}

impl<'a> SRef<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for SRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("S")
            .field("x", &self.x())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for SRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()     }
}

impl<'a> flatdata::Ref for SRef<'a> {}

pub struct SMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> SMut<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_x(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }


    #[inline]
    pub fn fill_from(&mut self, other: &SRef) {
        self.set_x(other.x());
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

impl<'a> std::fmt::Debug for SMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        SRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for SMut<'a> {}
#[derive(Clone, Debug)]
pub struct T {}

#[derive(Clone, Copy)]
pub struct TRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for T
{
    const SCHEMA: &'static str = schema::structs::T;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = TRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item{ data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = TMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut{ data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for T {}

impl<'a> TRef<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for TRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("T")
            .field("x", &self.x())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for TRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()     }
}

impl<'a> flatdata::Ref for TRef<'a> {}

pub struct TMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> TMut<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_x(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }


    #[inline]
    pub fn fill_from(&mut self, other: &TRef) {
        self.set_x(other.x());
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

impl<'a> std::fmt::Debug for TMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        TRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for TMut<'a> {}


/// Builtin union type of .n.S, .n.T.
#[derive(Clone, PartialEq)]
pub enum DataRef<'a> {
    S(<super::n::S as flatdata::Struct<'a>>::Item),    T(<super::n::T as flatdata::Struct<'a>>::Item),}

impl<'a> ::std::fmt::Debug for DataRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DataRef::S(ref inner) => write!(f, "{:?}", inner),
            DataRef::T(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for DataRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            DataRef::S(_) => <super::n::S as flatdata::Struct<'a>>::SIZE_IN_BYTES,
            DataRef::T(_) => <super::n::T as flatdata::Struct<'a>>::SIZE_IN_BYTES,
        }
    }
}

pub struct DataBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> DataBuilder<'a> {
    #[inline]
    pub fn add_s<'b>(&'b mut self) -> <super::n::S as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::S as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> <super::n::T as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 1;
        <super::n::T as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
}

#[derive(Clone)]
pub struct Data {}

impl<'a> flatdata::VariadicStruct<'a> for Data {
    type Index = super::_builtin::multivector::IndexType8;

    type Item = DataRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => DataRef::S(<super::n::S as flatdata::Struct<'a>>::create(data)),
                1 => DataRef::T(<super::n::T as flatdata::Struct<'a>>::create(data)),
            _ => panic!("invalid type index {} for variadic type DataRef", index),
        }
    }

    type ItemMut = DataBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}
/// Builtin union type of .n.S, .n.T.
#[derive(Clone, PartialEq)]
pub enum OptionalDataRef<'a> {
    S(<super::n::S as flatdata::Struct<'a>>::Item),    T(<super::n::T as flatdata::Struct<'a>>::Item),}

impl<'a> ::std::fmt::Debug for OptionalDataRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            OptionalDataRef::S(ref inner) => write!(f, "{:?}", inner),
            OptionalDataRef::T(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for OptionalDataRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            OptionalDataRef::S(_) => <super::n::S as flatdata::Struct<'a>>::SIZE_IN_BYTES,
            OptionalDataRef::T(_) => <super::n::T as flatdata::Struct<'a>>::SIZE_IN_BYTES,
        }
    }
}

pub struct OptionalDataBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> OptionalDataBuilder<'a> {
    #[inline]
    pub fn add_s<'b>(&'b mut self) -> <super::n::S as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::S as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> <super::n::T as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 1;
        <super::n::T as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
}

#[derive(Clone)]
pub struct OptionalData {}

impl<'a> flatdata::VariadicStruct<'a> for OptionalData {
    type Index = super::_builtin::multivector::IndexType16;

    type Item = OptionalDataRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => OptionalDataRef::S(<super::n::S as flatdata::Struct<'a>>::create(data)),
                1 => OptionalDataRef::T(<super::n::T as flatdata::Struct<'a>>::create(data)),
            _ => panic!("invalid type index {} for variadic type OptionalDataRef", index),
        }
    }

    type ItemMut = OptionalDataBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}
/// Builtin union type of .n.S, .n.T.
#[derive(Clone, PartialEq)]
pub enum DataU64IndexRef<'a> {
    S(<super::n::S as flatdata::Struct<'a>>::Item),    T(<super::n::T as flatdata::Struct<'a>>::Item),}

impl<'a> ::std::fmt::Debug for DataU64IndexRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DataU64IndexRef::S(ref inner) => write!(f, "{:?}", inner),
            DataU64IndexRef::T(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for DataU64IndexRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            DataU64IndexRef::S(_) => <super::n::S as flatdata::Struct<'a>>::SIZE_IN_BYTES,
            DataU64IndexRef::T(_) => <super::n::T as flatdata::Struct<'a>>::SIZE_IN_BYTES,
        }
    }
}

pub struct DataU64IndexBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> DataU64IndexBuilder<'a> {
    #[inline]
    pub fn add_s<'b>(&'b mut self) -> <super::n::S as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::S as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> <super::n::T as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 1;
        <super::n::T as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
}

#[derive(Clone)]
pub struct DataU64Index {}

impl<'a> flatdata::VariadicStruct<'a> for DataU64Index {
    type Index = super::_builtin::multivector::IndexType64;

    type Item = DataU64IndexRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => DataU64IndexRef::S(<super::n::S as flatdata::Struct<'a>>::create(data)),
                1 => DataU64IndexRef::T(<super::n::T as flatdata::Struct<'a>>::create(data)),
            _ => panic!("invalid type index {} for variadic type DataU64IndexRef", index),
        }
    }

    type ItemMut = DataU64IndexBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}

#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    data: (flatdata::MemoryDescriptor, flatdata::MemoryDescriptor),
    optional_data: Option<(flatdata::MemoryDescriptor, flatdata::MemoryDescriptor)>,
    data_u64_index: (flatdata::MemoryDescriptor, flatdata::MemoryDescriptor),
}

impl A {
    fn read_resource(
        storage: &dyn flatdata::ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<flatdata::MemoryDescriptor, flatdata::ResourceStorageError>
    {
        storage.read(name, schema).map(|x| flatdata::MemoryDescriptor::new(&x))
    }

    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> flatdata::MultiArrayView<Data>
    {
        flatdata::MultiArrayView::new(
            flatdata::ArrayView::new(&unsafe {self.data.0.as_bytes()}),
            &unsafe {self.data.1.as_bytes()},
        )
    }

    #[inline]
    pub fn optional_data(&self) -> Option<flatdata::MultiArrayView<OptionalData>>
    {
        self.optional_data.as_ref()
            .map(|(index, data)|{
                flatdata::MultiArrayView::new(flatdata::ArrayView::new(unsafe {index.as_bytes()}), unsafe {data.as_bytes()})
            })
    }

    #[inline]
    pub fn data_u64_index(&self) -> flatdata::MultiArrayView<DataU64Index>
    {
        flatdata::MultiArrayView::new(
            flatdata::ArrayView::new(&unsafe {self.data_u64_index.0.as_bytes()}),
            &unsafe {self.data_u64_index.1.as_bytes()},
        )
    }

}

impl ::std::fmt::Debug for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("A")
            .field("data", &self.data())
            .field("optional_data", &self.optional_data())
            .field("data_u64_index", &self.data_u64_index())
            .finish()
    }
}

impl flatdata::Archive for A {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let data = {
            let index_schema = &format!("index({})", schema::a::resources::DATA);
            let index = Self::read_resource(&*storage, "data_index", &index_schema)?;
            let data = Self::read_resource(&*storage, "data", schema::a::resources::DATA)?;            (index, data)
        };
        let optional_data = {
            let index_schema = &format!("index({})", schema::a::resources::OPTIONAL_DATA);
            let index = Self::read_resource(&*storage, "optional_data_index", &index_schema).ok();
            let data = Self::read_resource(&*storage, "optional_data", schema::a::resources::OPTIONAL_DATA).ok();            match (index, data) {
                (Some(index), Some(data)) => Some((index, data)),
                _ => None,
            }        };
        let data_u64_index = {
            let index_schema = &format!("index({})", schema::a::resources::DATA_U64_INDEX);
            let index = Self::read_resource(&*storage, "data_u64_index_index", &index_schema)?;
            let data = Self::read_resource(&*storage, "data_u64_index", schema::a::resources::DATA_U64_INDEX)?;            (index, data)
        };

        Ok(Self {
            _storage: storage,
            data,
            optional_data,
            data_u64_index,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ABuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl ABuilder {
    #[inline]
    pub fn start_data(&self) -> ::std::io::Result<flatdata::MultiVector<Data>> {
        flatdata::create_multi_vector(&*self.storage, "data", schema::a::resources::DATA)
    }

    #[inline]
    pub fn start_optional_data(&self) -> ::std::io::Result<flatdata::MultiVector<OptionalData>> {
        flatdata::create_multi_vector(&*self.storage, "optional_data", schema::a::resources::OPTIONAL_DATA)
    }

    #[inline]
    pub fn start_data_u64_index(&self) -> ::std::io::Result<flatdata::MultiVector<DataU64Index>> {
        flatdata::create_multi_vector(&*self.storage, "data_u64_index", schema::a::resources::DATA_U64_INDEX)
    }

}

impl flatdata::ArchiveBuilder for ABuilder {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}

}

pub mod _builtin {

pub mod multivector {

pub mod schema {
pub mod structs {
pub const INDEX_TYPE8: &str = r#""#;
pub const INDEX_TYPE16: &str = r#""#;
pub const INDEX_TYPE64: &str = r#""#;
}

}#[derive(Clone, Debug)]
pub struct IndexType8 {}

/// Builtin type to for MultiVector index
#[derive(Clone, Copy)]
pub struct IndexType8Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for IndexType8
{
    const SCHEMA: &'static str = schema::structs::INDEX_TYPE8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = IndexType8Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item{ data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = IndexType8Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut{ data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> IndexType8Ref<'a> {
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 8);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }#[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data, 0, 8);
        let end = flatdata_read_bytes!(u64, self.data, 0 + 1 * 8, 8);
        start..end
    }


    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for IndexType8Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType8")
            .field("value", &self.value())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for IndexType8Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl<'a> flatdata::Ref for IndexType8Ref<'a> {}

pub struct IndexType8Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> IndexType8Mut<'a> {
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 8);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_value(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 8)
    }


    #[inline]
    pub fn fill_from(&mut self, other: &IndexType8Ref) {
        self.set_value(other.value());
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

impl<'a> std::fmt::Debug for IndexType8Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        IndexType8Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for IndexType8Mut<'a> {}

impl<'a> flatdata::IndexStruct<'a> for IndexType8 {
    #[inline]
    fn range(data: Self::Item) -> std::ops::Range<usize> {
        let range = data.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(mut data: Self::ItemMut, value: usize) {
        data.set_value(value as u64);
    }
}

#[derive(Clone, Debug)]
pub struct IndexType16 {}

/// Builtin type to for MultiVector index
#[derive(Clone, Copy)]
pub struct IndexType16Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for IndexType16
{
    const SCHEMA: &'static str = schema::structs::INDEX_TYPE16;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = IndexType16Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item{ data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = IndexType16Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut{ data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> IndexType16Ref<'a> {
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 16);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }#[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data, 0, 16);
        let end = flatdata_read_bytes!(u64, self.data, 0 + 2 * 8, 16);
        start..end
    }


    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for IndexType16Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType16")
            .field("value", &self.value())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for IndexType16Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl<'a> flatdata::Ref for IndexType16Ref<'a> {}

pub struct IndexType16Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> IndexType16Mut<'a> {
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 16);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_value(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 2)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 16)
    }


    #[inline]
    pub fn fill_from(&mut self, other: &IndexType16Ref) {
        self.set_value(other.value());
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

impl<'a> std::fmt::Debug for IndexType16Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        IndexType16Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for IndexType16Mut<'a> {}

impl<'a> flatdata::IndexStruct<'a> for IndexType16 {
    #[inline]
    fn range(data: Self::Item) -> std::ops::Range<usize> {
        let range = data.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(mut data: Self::ItemMut, value: usize) {
        data.set_value(value as u64);
    }
}

#[derive(Clone, Debug)]
pub struct IndexType64 {}

/// Builtin type to for MultiVector index
#[derive(Clone, Copy)]
pub struct IndexType64Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for IndexType64
{
    const SCHEMA: &'static str = schema::structs::INDEX_TYPE64;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = IndexType64Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item{ data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = IndexType64Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut{ data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> IndexType64Ref<'a> {
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }#[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data, 0, 64);
        let end = flatdata_read_bytes!(u64, self.data, 0 + 8 * 8, 64);
        start..end
    }


    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for IndexType64Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType64")
            .field("value", &self.value())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for IndexType64Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl<'a> flatdata::Ref for IndexType64Ref<'a> {}

pub struct IndexType64Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> IndexType64Mut<'a> {
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_value(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }


    #[inline]
    pub fn fill_from(&mut self, other: &IndexType64Ref) {
        self.set_value(other.value());
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

impl<'a> std::fmt::Debug for IndexType64Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        IndexType64Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for IndexType64Mut<'a> {}

impl<'a> flatdata::IndexStruct<'a> for IndexType64 {
    #[inline]
    fn range(data: Self::Item) -> std::ops::Range<usize> {
        let range = data.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(mut data: Self::ItemMut, value: usize) {
        data.set_value(value as u64);
    }
}

}

pub mod schema {
pub mod structs {
}

}}
