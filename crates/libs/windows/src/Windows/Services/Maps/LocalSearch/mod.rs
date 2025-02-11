#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalCategoriesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILocalCategoriesStatics {
    type Vtable = ILocalCategoriesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ILocalCategoriesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf49399f5_8261_4321_9974_ef92d49a8dca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalCategoriesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BankAndCreditUnions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EatDrink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Hospitals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HotelsAndMotels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub All: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Parking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SeeDo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Shop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILocalLocation {
    type Vtable = ILocalLocation_Vtbl;
}
unsafe impl ::windows::core::Interface for ILocalLocation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb0fe9ab_4502_4f2c_94a9_0d60de0e2163);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DataAttribution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILocalLocation2 {
    type Vtable = ILocalLocation2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILocalLocation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e9e307c_ecb5_4ffc_bb8c_ba50ba8c2dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocation2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RatingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HoursOfOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HoursOfOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationFinderResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ILocalLocationFinderResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd09b6cc6_f338_4191_9fd8_5440b9a68f52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationFinderResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalLocations: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LocalLocationFinderStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationFinderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILocalLocationFinderStatics {
    type Vtable = ILocalLocationFinderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ILocalLocationFinderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ef7344_a0de_48ca_81a8_07c7dcfd37ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationFinderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocalLocationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchterm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, searcharea: *mut ::core::ffi::c_void, localcategory: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxresults: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocalLocationsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationHoursOfOperationItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_Vtbl;
}
unsafe impl ::windows::core::Interface for ILocalLocationHoursOfOperationItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23548c72_a1c7_43f1_a4f0_1091c39ec640);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationHoursOfOperationItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Start: usize,
    #[cfg(feature = "Foundation")]
    pub Span: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Span: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationRatingInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ILocalLocationRatingInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb1dab56_3354_4311_8bc0_a2d4d5eb806e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationRatingInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AggregateRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AggregateRating: usize,
    #[cfg(feature = "Foundation")]
    pub RatingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RatingCount: usize,
    pub ProviderIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceInfoHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlaceInfoHelperStatics {
    type Vtable = IPlaceInfoHelperStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlaceInfoHelperStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd1ca9a7_a9c6_491b_bc09_e80fcea48ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromLocalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct LocalCategories;
impl LocalCategories {
    pub fn BankAndCreditUnions() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BankAndCreditUnions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EatDrink() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EatDrink)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Hospitals() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hospitals)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HotelsAndMotels() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HotelsAndMotels)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn All() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).All)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Parking() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parking)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SeeDo() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SeeDo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Shop() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Shop)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocalCategoriesStatics<R, F: FnOnce(&ILocalCategoriesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LocalCategories, ILocalCategoriesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for LocalCategories {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalCategories";
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocation(::windows::core::IUnknown);
impl LocalLocation {
    pub fn Address(&self) -> ::windows::core::Result<super::MapAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Address)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MapAddress>(result__)
        }
    }
    pub fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Identifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Point)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DataAttribution(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataAttribution)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Category)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RatingInfo(&self) -> ::windows::core::Result<LocalLocationRatingInfo> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RatingInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<LocalLocationRatingInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HoursOfOperation(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HoursOfOperation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocation {}
impl ::core::fmt::Debug for LocalLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocation;{bb0fe9ab-4502-4f2c-94a9-0d60de0e2163})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LocalLocation {
    type Vtable = ILocalLocation_Vtbl;
}
unsafe impl ::windows::core::Interface for LocalLocation {
    const IID: ::windows::core::GUID = <ILocalLocation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalLocation {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocation";
}
::windows::core::interface_hierarchy!(LocalLocation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LocalLocation {}
unsafe impl ::core::marker::Sync for LocalLocation {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct LocalLocationFinder;
impl LocalLocationFinder {
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocalLocationsAsync(searchterm: &::windows::core::HSTRING, searcharea: &super::super::super::Devices::Geolocation::Geocircle, localcategory: &::windows::core::HSTRING, maxresults: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>> {
        Self::ILocalLocationFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindLocalLocationsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(searchterm), ::core::mem::transmute_copy(searcharea), ::core::mem::transmute_copy(localcategory), maxresults, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocalLocationFinderStatics<R, F: FnOnce(&ILocalLocationFinderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LocalLocationFinder, ILocalLocationFinderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for LocalLocationFinder {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinder";
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationFinderResult(::windows::core::IUnknown);
impl LocalLocationFinderResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalLocations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<LocalLocation>>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<LocalLocationFinderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<LocalLocationFinderStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalLocationFinderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalLocationFinderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationFinderResult {}
impl ::core::fmt::Debug for LocalLocationFinderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationFinderResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationFinderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationFinderResult;{d09b6cc6-f338-4191-9fd8-5440b9a68f52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_Vtbl;
}
unsafe impl ::windows::core::Interface for LocalLocationFinderResult {
    const IID: ::windows::core::GUID = <ILocalLocationFinderResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinderResult";
}
::windows::core::interface_hierarchy!(LocalLocationFinderResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LocalLocationFinderResult {}
unsafe impl ::core::marker::Sync for LocalLocationFinderResult {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationHoursOfOperationItem(::windows::core::IUnknown);
impl LocalLocationHoursOfOperationItem {
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn Day(&self) -> ::windows::core::Result<super::super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Day)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Globalization::DayOfWeek>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Start(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Span(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Span)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalLocationHoursOfOperationItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalLocationHoursOfOperationItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationHoursOfOperationItem {}
impl ::core::fmt::Debug for LocalLocationHoursOfOperationItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationHoursOfOperationItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationHoursOfOperationItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem;{23548c72-a1c7-43f1-a4f0-1091c39ec640})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_Vtbl;
}
unsafe impl ::windows::core::Interface for LocalLocationHoursOfOperationItem {
    const IID: ::windows::core::GUID = <ILocalLocationHoursOfOperationItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalLocationHoursOfOperationItem {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem";
}
::windows::core::interface_hierarchy!(LocalLocationHoursOfOperationItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LocalLocationHoursOfOperationItem {}
unsafe impl ::core::marker::Sync for LocalLocationHoursOfOperationItem {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationRatingInfo(::windows::core::IUnknown);
impl LocalLocationRatingInfo {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AggregateRating(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AggregateRating)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RatingCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RatingCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    pub fn ProviderIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderIdentifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalLocationRatingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalLocationRatingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationRatingInfo {}
impl ::core::fmt::Debug for LocalLocationRatingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationRatingInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationRatingInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo;{cb1dab56-3354-4311-8bc0-a2d4d5eb806e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for LocalLocationRatingInfo {
    const IID: ::windows::core::GUID = <ILocalLocationRatingInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalLocationRatingInfo {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo";
}
::windows::core::interface_hierarchy!(LocalLocationRatingInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LocalLocationRatingInfo {}
unsafe impl ::core::marker::Sync for LocalLocationRatingInfo {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct PlaceInfoHelper;
impl PlaceInfoHelper {
    pub fn CreateFromLocalLocation(location: &LocalLocation) -> ::windows::core::Result<super::PlaceInfo> {
        Self::IPlaceInfoHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromLocalLocation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(location), result__.as_mut_ptr()).from_abi::<super::PlaceInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaceInfoHelperStatics<R, F: FnOnce(&IPlaceInfoHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlaceInfoHelper, IPlaceInfoHelperStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PlaceInfoHelper {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.PlaceInfoHelper";
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LocalLocationFinderStatus(pub i32);
impl LocalLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const InvalidCategory: Self = Self(3i32);
    pub const InvalidSearchTerm: Self = Self(4i32);
    pub const InvalidSearchArea: Self = Self(5i32);
    pub const NetworkFailure: Self = Self(6i32);
    pub const NotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for LocalLocationFinderStatus {}
impl ::core::clone::Clone for LocalLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LocalLocationFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LocalLocationFinderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LocalLocationFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationFinderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationFinderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.LocalSearch.LocalLocationFinderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
