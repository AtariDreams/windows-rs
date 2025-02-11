#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IChannelCredentials_Impl: Sized + super::IDispatch_Impl {
    fn SetWindowsCredential(&self, domain: &::windows::core::BSTR, username: &::windows::core::BSTR, password: &::windows::core::BSTR, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetUserNameCredential(&self, username: &::windows::core::BSTR, password: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromStore(&self, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR, findyype: &::windows::core::BSTR, findvalue: &super::VARIANT) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromStoreByName(&self, subjectname: &::windows::core::BSTR, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCertificateFromFile(&self, filename: &::windows::core::BSTR, password: &::windows::core::BSTR, keystorageflags: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(&self, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR, findtype: &::windows::core::BSTR, findvalue: &super::VARIANT) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(&self, subjectname: &::windows::core::BSTR, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(&self, filename: &::windows::core::BSTR, password: &::windows::core::BSTR, keystorageflags: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetServiceCertificateAuthentication(&self, storelocation: &::windows::core::BSTR, revocationmode: &::windows::core::BSTR, certificatevalidationmode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetIssuedToken(&self, localissueraddres: &::windows::core::BSTR, localissuerbindingtype: &::windows::core::BSTR, localissuerbinding: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IChannelCredentials {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IChannelCredentials_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>() -> IChannelCredentials_Vtbl {
        unsafe extern "system" fn SetWindowsCredential<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domain: ::core::mem::ManuallyDrop<::windows::core::BSTR>, username: ::core::mem::ManuallyDrop<::windows::core::BSTR>, password: ::core::mem::ManuallyDrop<::windows::core::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowsCredential(::core::mem::transmute(&domain), ::core::mem::transmute(&username), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&impersonationlevel), ::core::mem::transmute_copy(&allowntlm)).into()
        }
        unsafe extern "system" fn SetUserNameCredential<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::BSTR>, password: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserNameCredential(::core::mem::transmute(&username), ::core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<::windows::core::BSTR>, storename: ::core::mem::ManuallyDrop<::windows::core::BSTR>, findyype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificateFromStore(::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename), ::core::mem::transmute(&findyype), ::core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, storelocation: ::core::mem::ManuallyDrop<::windows::core::BSTR>, storename: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificateFromStoreByName(::core::mem::transmute(&subjectname), ::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::BSTR>, password: ::core::mem::ManuallyDrop<::windows::core::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificateFromFile(::core::mem::transmute(&filename), ::core::mem::transmute(&password), ::core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<::windows::core::BSTR>, storename: ::core::mem::ManuallyDrop<::windows::core::BSTR>, findtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultServiceCertificateFromStore(::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename), ::core::mem::transmute(&findtype), ::core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<::windows::core::BSTR>, storelocation: ::core::mem::ManuallyDrop<::windows::core::BSTR>, storename: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultServiceCertificateFromStoreByName(::core::mem::transmute(&subjectname), ::core::mem::transmute(&storelocation), ::core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::BSTR>, password: ::core::mem::ManuallyDrop<::windows::core::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultServiceCertificateFromFile(::core::mem::transmute(&filename), ::core::mem::transmute(&password), ::core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<::windows::core::BSTR>, revocationmode: ::core::mem::ManuallyDrop<::windows::core::BSTR>, certificatevalidationmode: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceCertificateAuthentication(::core::mem::transmute(&storelocation), ::core::mem::transmute(&revocationmode), ::core::mem::transmute(&certificatevalidationmode)).into()
        }
        unsafe extern "system" fn SetIssuedToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localissueraddres: ::core::mem::ManuallyDrop<::windows::core::BSTR>, localissuerbindingtype: ::core::mem::ManuallyDrop<::windows::core::BSTR>, localissuerbinding: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIssuedToken(::core::mem::transmute(&localissueraddres), ::core::mem::transmute(&localissuerbindingtype), ::core::mem::transmute(&localissuerbinding)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetWindowsCredential: SetWindowsCredential::<Identity, Impl, OFFSET>,
            SetUserNameCredential: SetUserNameCredential::<Identity, Impl, OFFSET>,
            SetClientCertificateFromStore: SetClientCertificateFromStore::<Identity, Impl, OFFSET>,
            SetClientCertificateFromStoreByName: SetClientCertificateFromStoreByName::<Identity, Impl, OFFSET>,
            SetClientCertificateFromFile: SetClientCertificateFromFile::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromStore: SetDefaultServiceCertificateFromStore::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromStoreByName: SetDefaultServiceCertificateFromStoreByName::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromFile: SetDefaultServiceCertificateFromFile::<Identity, Impl, OFFSET>,
            SetServiceCertificateAuthentication: SetServiceCertificateAuthentication::<Identity, Impl, OFFSET>,
            SetIssuedToken: SetIssuedToken::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChannelCredentials as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
