const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.30.2-beta2");
#[allow(non_camel_case_types)]
pub struct GetLicenseRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetLicenseRequest {}

impl ::std::default::Default for GetLicenseRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetLicenseRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetLicenseRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetLicenseRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetLicenseRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetLicenseRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetLicenseRequest {
  fn clear_and_parse(&mut self, data: &[u8]) -> ::std::result::Result<(), ::protobuf::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::protobuf::__internal::runtime::SerializedData::from_raw_parts(
        ::std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      ::protobuf::__internal::runtime::proto2_rust_Message_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::protobuf::ParseError)
  }
}

// SAFETY:
// - `GetLicenseRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetLicenseRequestMut`.
unsafe impl Sync for GetLicenseRequest {}

// SAFETY:
// - `GetLicenseRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetLicenseRequest {}

impl ::protobuf::Proxied for GetLicenseRequest {
  type View<'msg> = GetLicenseRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetLicenseRequest {}

impl ::protobuf::MutProxied for GetLicenseRequest {
  type Mut<'msg> = GetLicenseRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetLicenseRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetLicenseRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetLicenseRequestView<'msg> {
  type Message = GetLicenseRequest;
}

impl ::std::fmt::Debug for GetLicenseRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetLicenseRequestView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for GetLicenseRequestView<'_> {
  fn default() -> GetLicenseRequestView<'static> {
    GetLicenseRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szproduct_GetLicenseRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetLicenseRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetLicenseRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GetLicenseRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetLicenseRequestView<'_> {}

// SAFETY:
// - `GetLicenseRequestView` is `Send` because while its alive a `GetLicenseRequestMut` cannot.
// - `GetLicenseRequestView` does not use thread-local data.
unsafe impl Send for GetLicenseRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetLicenseRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetLicenseRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetLicenseRequestView<'msg> {
  type Proxied = GetLicenseRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetLicenseRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetLicenseRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetLicenseRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetLicenseRequest> for GetLicenseRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetLicenseRequest {
    let dst = GetLicenseRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetLicenseRequest> for GetLicenseRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetLicenseRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetLicenseRequest {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <GetLicenseRequestView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetLicenseRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetLicenseRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetLicenseRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetLicenseRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetLicenseRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetLicenseRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetLicenseRequestMut<'msg> {
  type Message = GetLicenseRequest;
}

impl ::std::fmt::Debug for GetLicenseRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetLicenseRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetLicenseRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetLicenseRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetLicenseRequest>) {
    // SAFETY: self and src are both valid `GetLicenseRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetLicenseRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: &'msg mut ::protobuf::__internal::runtime::MessageInner) -> Self {
    Self{ inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg) }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg()
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn to_owned(&self) -> GetLicenseRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `GetLicenseRequestMut` does not perform any shared mutation.
// - `GetLicenseRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetLicenseRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetLicenseRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetLicenseRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetLicenseRequestMut<'msg> {
  type Proxied = GetLicenseRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetLicenseRequest> {
    GetLicenseRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetLicenseRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetLicenseRequest>
  where
      'msg: 'shorter {
    GetLicenseRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetLicenseRequestMut<'msg> {
  type MutProxied = GetLicenseRequest;
  fn as_mut(&mut self) -> GetLicenseRequestMut<'msg> {
    GetLicenseRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetLicenseRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetLicenseRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetLicenseRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szproduct_GetLicenseRequest_new() } } }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MutatorMessageRef {
    ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetLicenseRequestView {
    GetLicenseRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetLicenseRequestMut {
    GetLicenseRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl GetLicenseRequest

impl ::std::ops::Drop for GetLicenseRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetLicenseRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetLicenseRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetLicenseRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetLicenseRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetLicenseRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szproduct_GetLicenseRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szproduct_GetLicenseRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> GetLicenseRequestMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> GetLicenseRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetLicenseRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetLicenseRequestMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
    }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
    }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for GetLicenseRequestView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::__internal::MatcherEq for GetLicenseRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetLicenseRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetLicenseRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetLicenseResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetLicenseResponse {}

impl ::std::default::Default for GetLicenseResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetLicenseResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetLicenseResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetLicenseResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetLicenseResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetLicenseResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetLicenseResponse {
  fn clear_and_parse(&mut self, data: &[u8]) -> ::std::result::Result<(), ::protobuf::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::protobuf::__internal::runtime::SerializedData::from_raw_parts(
        ::std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      ::protobuf::__internal::runtime::proto2_rust_Message_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::protobuf::ParseError)
  }
}

// SAFETY:
// - `GetLicenseResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetLicenseResponseMut`.
unsafe impl Sync for GetLicenseResponse {}

// SAFETY:
// - `GetLicenseResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetLicenseResponse {}

impl ::protobuf::Proxied for GetLicenseResponse {
  type View<'msg> = GetLicenseResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetLicenseResponse {}

impl ::protobuf::MutProxied for GetLicenseResponse {
  type Mut<'msg> = GetLicenseResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetLicenseResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetLicenseResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetLicenseResponseView<'msg> {
  type Message = GetLicenseResponse;
}

impl ::std::fmt::Debug for GetLicenseResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetLicenseResponseView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for GetLicenseResponseView<'_> {
  fn default() -> GetLicenseResponseView<'static> {
    GetLicenseResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szproduct_GetLicenseResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetLicenseResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetLicenseResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szproduct_GetLicenseResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetLicenseResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetLicenseResponseView<'_> {}

// SAFETY:
// - `GetLicenseResponseView` is `Send` because while its alive a `GetLicenseResponseMut` cannot.
// - `GetLicenseResponseView` does not use thread-local data.
unsafe impl Send for GetLicenseResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetLicenseResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetLicenseResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetLicenseResponseView<'msg> {
  type Proxied = GetLicenseResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetLicenseResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetLicenseResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetLicenseResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetLicenseResponse> for GetLicenseResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetLicenseResponse {
    let dst = GetLicenseResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetLicenseResponse> for GetLicenseResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetLicenseResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetLicenseResponse {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <GetLicenseResponseView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetLicenseResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetLicenseResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetLicenseResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetLicenseResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetLicenseResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetLicenseResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetLicenseResponseMut<'msg> {
  type Message = GetLicenseResponse;
}

impl ::std::fmt::Debug for GetLicenseResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetLicenseResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetLicenseResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetLicenseResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetLicenseResponse>) {
    // SAFETY: self and src are both valid `GetLicenseResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetLicenseResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: &'msg mut ::protobuf::__internal::runtime::MessageInner) -> Self {
    Self{ inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg) }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg()
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn to_owned(&self) -> GetLicenseResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szproduct_GetLicenseResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szproduct_GetLicenseResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetLicenseResponseMut` does not perform any shared mutation.
// - `GetLicenseResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetLicenseResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetLicenseResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetLicenseResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetLicenseResponseMut<'msg> {
  type Proxied = GetLicenseResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetLicenseResponse> {
    GetLicenseResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetLicenseResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetLicenseResponse>
  where
      'msg: 'shorter {
    GetLicenseResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetLicenseResponseMut<'msg> {
  type MutProxied = GetLicenseResponse;
  fn as_mut(&mut self) -> GetLicenseResponseMut<'msg> {
    GetLicenseResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetLicenseResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetLicenseResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetLicenseResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szproduct_GetLicenseResponse_new() } } }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MutatorMessageRef {
    ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetLicenseResponseView {
    GetLicenseResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetLicenseResponseMut {
    GetLicenseResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szproduct_GetLicenseResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szproduct_GetLicenseResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetLicenseResponse

impl ::std::ops::Drop for GetLicenseResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetLicenseResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetLicenseResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetLicenseResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetLicenseResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetLicenseResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szproduct_GetLicenseResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szproduct_GetLicenseResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szproduct_GetLicenseResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szproduct_GetLicenseResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetLicenseResponseMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> GetLicenseResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetLicenseResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetLicenseResponseMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
    }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
    }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for GetLicenseResponseView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::__internal::MatcherEq for GetLicenseResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetLicenseResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetLicenseResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetVersionRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetVersionRequest {}

impl ::std::default::Default for GetVersionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetVersionRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetVersionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetVersionRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetVersionRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetVersionRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetVersionRequest {
  fn clear_and_parse(&mut self, data: &[u8]) -> ::std::result::Result<(), ::protobuf::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::protobuf::__internal::runtime::SerializedData::from_raw_parts(
        ::std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      ::protobuf::__internal::runtime::proto2_rust_Message_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::protobuf::ParseError)
  }
}

// SAFETY:
// - `GetVersionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetVersionRequestMut`.
unsafe impl Sync for GetVersionRequest {}

// SAFETY:
// - `GetVersionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetVersionRequest {}

impl ::protobuf::Proxied for GetVersionRequest {
  type View<'msg> = GetVersionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetVersionRequest {}

impl ::protobuf::MutProxied for GetVersionRequest {
  type Mut<'msg> = GetVersionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetVersionRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetVersionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetVersionRequestView<'msg> {
  type Message = GetVersionRequest;
}

impl ::std::fmt::Debug for GetVersionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetVersionRequestView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for GetVersionRequestView<'_> {
  fn default() -> GetVersionRequestView<'static> {
    GetVersionRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szproduct_GetVersionRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetVersionRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetVersionRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GetVersionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetVersionRequestView<'_> {}

// SAFETY:
// - `GetVersionRequestView` is `Send` because while its alive a `GetVersionRequestMut` cannot.
// - `GetVersionRequestView` does not use thread-local data.
unsafe impl Send for GetVersionRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetVersionRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetVersionRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetVersionRequestView<'msg> {
  type Proxied = GetVersionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetVersionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetVersionRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetVersionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetVersionRequest> for GetVersionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetVersionRequest {
    let dst = GetVersionRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetVersionRequest> for GetVersionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetVersionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetVersionRequest {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <GetVersionRequestView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetVersionRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetVersionRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetVersionRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetVersionRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetVersionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetVersionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetVersionRequestMut<'msg> {
  type Message = GetVersionRequest;
}

impl ::std::fmt::Debug for GetVersionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetVersionRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetVersionRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetVersionRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetVersionRequest>) {
    // SAFETY: self and src are both valid `GetVersionRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetVersionRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: &'msg mut ::protobuf::__internal::runtime::MessageInner) -> Self {
    Self{ inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg) }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg()
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn to_owned(&self) -> GetVersionRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `GetVersionRequestMut` does not perform any shared mutation.
// - `GetVersionRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetVersionRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetVersionRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetVersionRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetVersionRequestMut<'msg> {
  type Proxied = GetVersionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetVersionRequest> {
    GetVersionRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetVersionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetVersionRequest>
  where
      'msg: 'shorter {
    GetVersionRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetVersionRequestMut<'msg> {
  type MutProxied = GetVersionRequest;
  fn as_mut(&mut self) -> GetVersionRequestMut<'msg> {
    GetVersionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetVersionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetVersionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetVersionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szproduct_GetVersionRequest_new() } } }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MutatorMessageRef {
    ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetVersionRequestView {
    GetVersionRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetVersionRequestMut {
    GetVersionRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl GetVersionRequest

impl ::std::ops::Drop for GetVersionRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetVersionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetVersionRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetVersionRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetVersionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetVersionRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szproduct_GetVersionRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szproduct_GetVersionRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> GetVersionRequestMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> GetVersionRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetVersionRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetVersionRequestMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
    }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
    }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for GetVersionRequestView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::__internal::MatcherEq for GetVersionRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetVersionRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetVersionRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetVersionResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetVersionResponse {}

impl ::std::default::Default for GetVersionResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetVersionResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetVersionResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetVersionResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetVersionResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetVersionResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetVersionResponse {
  fn clear_and_parse(&mut self, data: &[u8]) -> ::std::result::Result<(), ::protobuf::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::protobuf::__internal::runtime::SerializedData::from_raw_parts(
        ::std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      ::protobuf::__internal::runtime::proto2_rust_Message_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::protobuf::ParseError)
  }
}

// SAFETY:
// - `GetVersionResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetVersionResponseMut`.
unsafe impl Sync for GetVersionResponse {}

// SAFETY:
// - `GetVersionResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetVersionResponse {}

impl ::protobuf::Proxied for GetVersionResponse {
  type View<'msg> = GetVersionResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetVersionResponse {}

impl ::protobuf::MutProxied for GetVersionResponse {
  type Mut<'msg> = GetVersionResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetVersionResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetVersionResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetVersionResponseView<'msg> {
  type Message = GetVersionResponse;
}

impl ::std::fmt::Debug for GetVersionResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetVersionResponseView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for GetVersionResponseView<'_> {
  fn default() -> GetVersionResponseView<'static> {
    GetVersionResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szproduct_GetVersionResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetVersionResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetVersionResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szproduct_GetVersionResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetVersionResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetVersionResponseView<'_> {}

// SAFETY:
// - `GetVersionResponseView` is `Send` because while its alive a `GetVersionResponseMut` cannot.
// - `GetVersionResponseView` does not use thread-local data.
unsafe impl Send for GetVersionResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetVersionResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetVersionResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetVersionResponseView<'msg> {
  type Proxied = GetVersionResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetVersionResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetVersionResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetVersionResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetVersionResponse> for GetVersionResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetVersionResponse {
    let dst = GetVersionResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetVersionResponse> for GetVersionResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetVersionResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetVersionResponse {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <GetVersionResponseView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetVersionResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetVersionResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetVersionResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetVersionResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetVersionResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetVersionResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetVersionResponseMut<'msg> {
  type Message = GetVersionResponse;
}

impl ::std::fmt::Debug for GetVersionResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetVersionResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetVersionResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetVersionResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetVersionResponse>) {
    // SAFETY: self and src are both valid `GetVersionResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetVersionResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: &'msg mut ::protobuf::__internal::runtime::MessageInner) -> Self {
    Self{ inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg) }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg()
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn to_owned(&self) -> GetVersionResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szproduct_GetVersionResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szproduct_GetVersionResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetVersionResponseMut` does not perform any shared mutation.
// - `GetVersionResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetVersionResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetVersionResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetVersionResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetVersionResponseMut<'msg> {
  type Proxied = GetVersionResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetVersionResponse> {
    GetVersionResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetVersionResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetVersionResponse>
  where
      'msg: 'shorter {
    GetVersionResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetVersionResponseMut<'msg> {
  type MutProxied = GetVersionResponse;
  fn as_mut(&mut self) -> GetVersionResponseMut<'msg> {
    GetVersionResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetVersionResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetVersionResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetVersionResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szproduct_GetVersionResponse_new() } } }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MutatorMessageRef {
    ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetVersionResponseView {
    GetVersionResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetVersionResponseMut {
    GetVersionResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szproduct_GetVersionResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szproduct_GetVersionResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetVersionResponse

impl ::std::ops::Drop for GetVersionResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetVersionResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetVersionResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetVersionResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetVersionResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetVersionResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szproduct_GetVersionResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szproduct_GetVersionResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szproduct_GetVersionResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szproduct_GetVersionResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetVersionResponseMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> GetVersionResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetVersionResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetVersionResponseMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
    }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::wrap_raw(
          ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
    }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for GetVersionResponseView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::__internal::MatcherEq for GetVersionResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetVersionResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetVersionResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

