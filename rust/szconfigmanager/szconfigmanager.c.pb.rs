const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.30.2-beta2");
#[allow(non_camel_case_types)]
pub struct GetConfigRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetConfigRequest {}

impl ::std::default::Default for GetConfigRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetConfigRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetConfigRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetConfigRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetConfigRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetConfigRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetConfigRequest {
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
// - `GetConfigRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetConfigRequestMut`.
unsafe impl Sync for GetConfigRequest {}

// SAFETY:
// - `GetConfigRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetConfigRequest {}

impl ::protobuf::Proxied for GetConfigRequest {
  type View<'msg> = GetConfigRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetConfigRequest {}

impl ::protobuf::MutProxied for GetConfigRequest {
  type Mut<'msg> = GetConfigRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetConfigRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetConfigRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetConfigRequestView<'msg> {
  type Message = GetConfigRequest;
}

impl ::std::fmt::Debug for GetConfigRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetConfigRequestView<'_> {
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

impl ::std::default::Default for GetConfigRequestView<'_> {
  fn default() -> GetConfigRequestView<'static> {
    GetConfigRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_GetConfigRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetConfigRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetConfigRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_id: optional int64
  pub fn config_id(self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `GetConfigRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetConfigRequestView<'_> {}

// SAFETY:
// - `GetConfigRequestView` is `Send` because while its alive a `GetConfigRequestMut` cannot.
// - `GetConfigRequestView` does not use thread-local data.
unsafe impl Send for GetConfigRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetConfigRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetConfigRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetConfigRequestView<'msg> {
  type Proxied = GetConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetConfigRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetConfigRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetConfigRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetConfigRequest> for GetConfigRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetConfigRequest {
    let dst = GetConfigRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetConfigRequest> for GetConfigRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetConfigRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetConfigRequest {
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
      let prototype = <GetConfigRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetConfigRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetConfigRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetConfigRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetConfigRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetConfigRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetConfigRequestMut<'msg> {
  type Message = GetConfigRequest;
}

impl ::std::fmt::Debug for GetConfigRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetConfigRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetConfigRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetConfigRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetConfigRequest>) {
    // SAFETY: self and src are both valid `GetConfigRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetConfigRequestMut<'msg> {
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

  pub fn to_owned(&self) -> GetConfigRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_id: optional int64
  pub fn config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_get(self.raw_msg()) }
  }
  pub fn set_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `GetConfigRequestMut` does not perform any shared mutation.
// - `GetConfigRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetConfigRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetConfigRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetConfigRequestMut<'msg> {
  type Proxied = GetConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetConfigRequest> {
    GetConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetConfigRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetConfigRequest>
  where
      'msg: 'shorter {
    GetConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetConfigRequestMut<'msg> {
  type MutProxied = GetConfigRequest;
  fn as_mut(&mut self) -> GetConfigRequestMut<'msg> {
    GetConfigRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetConfigRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetConfigRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetConfigRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_GetConfigRequest_new() } } }
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

  pub fn as_view(&self) -> GetConfigRequestView {
    GetConfigRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetConfigRequestMut {
    GetConfigRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_id: optional int64
  pub fn config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_get(self.raw_msg()) }
  }
  pub fn set_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_set(self.raw_msg(), val) }
  }

}  // impl GetConfigRequest

impl ::std::ops::Drop for GetConfigRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetConfigRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetConfigRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetConfigRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetConfigRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetConfigRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_GetConfigRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_GetConfigRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szconfigmanager_GetConfigRequest_config_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

}

impl<'a> GetConfigRequestMut<'a> {
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

impl<'a> GetConfigRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetConfigRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetConfigRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetConfigRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetConfigRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetConfigRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetConfigRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetConfigResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetConfigResponse {}

impl ::std::default::Default for GetConfigResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetConfigResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetConfigResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetConfigResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetConfigResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetConfigResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetConfigResponse {
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
// - `GetConfigResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetConfigResponseMut`.
unsafe impl Sync for GetConfigResponse {}

// SAFETY:
// - `GetConfigResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetConfigResponse {}

impl ::protobuf::Proxied for GetConfigResponse {
  type View<'msg> = GetConfigResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetConfigResponse {}

impl ::protobuf::MutProxied for GetConfigResponse {
  type Mut<'msg> = GetConfigResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetConfigResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetConfigResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetConfigResponseView<'msg> {
  type Message = GetConfigResponse;
}

impl ::std::fmt::Debug for GetConfigResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetConfigResponseView<'_> {
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

impl ::std::default::Default for GetConfigResponseView<'_> {
  fn default() -> GetConfigResponseView<'static> {
    GetConfigResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_GetConfigResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetConfigResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetConfigResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetConfigResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetConfigResponseView<'_> {}

// SAFETY:
// - `GetConfigResponseView` is `Send` because while its alive a `GetConfigResponseMut` cannot.
// - `GetConfigResponseView` does not use thread-local data.
unsafe impl Send for GetConfigResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetConfigResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetConfigResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetConfigResponseView<'msg> {
  type Proxied = GetConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetConfigResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetConfigResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetConfigResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetConfigResponse> for GetConfigResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetConfigResponse {
    let dst = GetConfigResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetConfigResponse> for GetConfigResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetConfigResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetConfigResponse {
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
      let prototype = <GetConfigResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetConfigResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetConfigResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetConfigResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetConfigResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetConfigResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetConfigResponseMut<'msg> {
  type Message = GetConfigResponse;
}

impl ::std::fmt::Debug for GetConfigResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetConfigResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetConfigResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetConfigResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetConfigResponse>) {
    // SAFETY: self and src are both valid `GetConfigResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetConfigResponseMut<'msg> {
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

  pub fn to_owned(&self) -> GetConfigResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetConfigResponseMut` does not perform any shared mutation.
// - `GetConfigResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetConfigResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetConfigResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetConfigResponseMut<'msg> {
  type Proxied = GetConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetConfigResponse> {
    GetConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetConfigResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetConfigResponse>
  where
      'msg: 'shorter {
    GetConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetConfigResponseMut<'msg> {
  type MutProxied = GetConfigResponse;
  fn as_mut(&mut self) -> GetConfigResponseMut<'msg> {
    GetConfigResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetConfigResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetConfigResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetConfigResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_GetConfigResponse_new() } } }
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

  pub fn as_view(&self) -> GetConfigResponseView {
    GetConfigResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetConfigResponseMut {
    GetConfigResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetConfigResponse

impl ::std::ops::Drop for GetConfigResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetConfigResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetConfigResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetConfigResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetConfigResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetConfigResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_GetConfigResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_GetConfigResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfigmanager_GetConfigResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetConfigResponseMut<'a> {
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

impl<'a> GetConfigResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetConfigResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetConfigResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetConfigResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetConfigResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetConfigResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetConfigResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetConfigRegistryRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetConfigRegistryRequest {}

impl ::std::default::Default for GetConfigRegistryRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetConfigRegistryRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetConfigRegistryRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetConfigRegistryRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetConfigRegistryRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetConfigRegistryRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetConfigRegistryRequest {
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
// - `GetConfigRegistryRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetConfigRegistryRequestMut`.
unsafe impl Sync for GetConfigRegistryRequest {}

// SAFETY:
// - `GetConfigRegistryRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetConfigRegistryRequest {}

impl ::protobuf::Proxied for GetConfigRegistryRequest {
  type View<'msg> = GetConfigRegistryRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetConfigRegistryRequest {}

impl ::protobuf::MutProxied for GetConfigRegistryRequest {
  type Mut<'msg> = GetConfigRegistryRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetConfigRegistryRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetConfigRegistryRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetConfigRegistryRequestView<'msg> {
  type Message = GetConfigRegistryRequest;
}

impl ::std::fmt::Debug for GetConfigRegistryRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetConfigRegistryRequestView<'_> {
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

impl ::std::default::Default for GetConfigRegistryRequestView<'_> {
  fn default() -> GetConfigRegistryRequestView<'static> {
    GetConfigRegistryRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetConfigRegistryRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetConfigRegistryRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GetConfigRegistryRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetConfigRegistryRequestView<'_> {}

// SAFETY:
// - `GetConfigRegistryRequestView` is `Send` because while its alive a `GetConfigRegistryRequestMut` cannot.
// - `GetConfigRegistryRequestView` does not use thread-local data.
unsafe impl Send for GetConfigRegistryRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetConfigRegistryRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetConfigRegistryRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetConfigRegistryRequestView<'msg> {
  type Proxied = GetConfigRegistryRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetConfigRegistryRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetConfigRegistryRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetConfigRegistryRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetConfigRegistryRequest> for GetConfigRegistryRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetConfigRegistryRequest {
    let dst = GetConfigRegistryRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetConfigRegistryRequest> for GetConfigRegistryRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetConfigRegistryRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetConfigRegistryRequest {
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
      let prototype = <GetConfigRegistryRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetConfigRegistryRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetConfigRegistryRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetConfigRegistryRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetConfigRegistryRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetConfigRegistryRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetConfigRegistryRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetConfigRegistryRequestMut<'msg> {
  type Message = GetConfigRegistryRequest;
}

impl ::std::fmt::Debug for GetConfigRegistryRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetConfigRegistryRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetConfigRegistryRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetConfigRegistryRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetConfigRegistryRequest>) {
    // SAFETY: self and src are both valid `GetConfigRegistryRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetConfigRegistryRequestMut<'msg> {
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

  pub fn to_owned(&self) -> GetConfigRegistryRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `GetConfigRegistryRequestMut` does not perform any shared mutation.
// - `GetConfigRegistryRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetConfigRegistryRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetConfigRegistryRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetConfigRegistryRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetConfigRegistryRequestMut<'msg> {
  type Proxied = GetConfigRegistryRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetConfigRegistryRequest> {
    GetConfigRegistryRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetConfigRegistryRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetConfigRegistryRequest>
  where
      'msg: 'shorter {
    GetConfigRegistryRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetConfigRegistryRequestMut<'msg> {
  type MutProxied = GetConfigRegistryRequest;
  fn as_mut(&mut self) -> GetConfigRegistryRequestMut<'msg> {
    GetConfigRegistryRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetConfigRegistryRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetConfigRegistryRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetConfigRegistryRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryRequest_new() } } }
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

  pub fn as_view(&self) -> GetConfigRegistryRequestView {
    GetConfigRegistryRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetConfigRegistryRequestMut {
    GetConfigRegistryRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl GetConfigRegistryRequest

impl ::std::ops::Drop for GetConfigRegistryRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetConfigRegistryRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetConfigRegistryRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetConfigRegistryRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetConfigRegistryRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetConfigRegistryRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> GetConfigRegistryRequestMut<'a> {
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

impl<'a> GetConfigRegistryRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetConfigRegistryRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetConfigRegistryRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetConfigRegistryRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetConfigRegistryRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetConfigRegistryRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetConfigRegistryRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetConfigRegistryResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetConfigRegistryResponse {}

impl ::std::default::Default for GetConfigRegistryResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetConfigRegistryResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetConfigRegistryResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetConfigRegistryResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetConfigRegistryResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetConfigRegistryResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetConfigRegistryResponse {
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
// - `GetConfigRegistryResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetConfigRegistryResponseMut`.
unsafe impl Sync for GetConfigRegistryResponse {}

// SAFETY:
// - `GetConfigRegistryResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetConfigRegistryResponse {}

impl ::protobuf::Proxied for GetConfigRegistryResponse {
  type View<'msg> = GetConfigRegistryResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetConfigRegistryResponse {}

impl ::protobuf::MutProxied for GetConfigRegistryResponse {
  type Mut<'msg> = GetConfigRegistryResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetConfigRegistryResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetConfigRegistryResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetConfigRegistryResponseView<'msg> {
  type Message = GetConfigRegistryResponse;
}

impl ::std::fmt::Debug for GetConfigRegistryResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetConfigRegistryResponseView<'_> {
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

impl ::std::default::Default for GetConfigRegistryResponseView<'_> {
  fn default() -> GetConfigRegistryResponseView<'static> {
    GetConfigRegistryResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetConfigRegistryResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetConfigRegistryResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetConfigRegistryResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetConfigRegistryResponseView<'_> {}

// SAFETY:
// - `GetConfigRegistryResponseView` is `Send` because while its alive a `GetConfigRegistryResponseMut` cannot.
// - `GetConfigRegistryResponseView` does not use thread-local data.
unsafe impl Send for GetConfigRegistryResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetConfigRegistryResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetConfigRegistryResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetConfigRegistryResponseView<'msg> {
  type Proxied = GetConfigRegistryResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetConfigRegistryResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetConfigRegistryResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetConfigRegistryResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetConfigRegistryResponse> for GetConfigRegistryResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetConfigRegistryResponse {
    let dst = GetConfigRegistryResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetConfigRegistryResponse> for GetConfigRegistryResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetConfigRegistryResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetConfigRegistryResponse {
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
      let prototype = <GetConfigRegistryResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetConfigRegistryResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetConfigRegistryResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetConfigRegistryResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetConfigRegistryResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetConfigRegistryResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetConfigRegistryResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetConfigRegistryResponseMut<'msg> {
  type Message = GetConfigRegistryResponse;
}

impl ::std::fmt::Debug for GetConfigRegistryResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetConfigRegistryResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetConfigRegistryResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetConfigRegistryResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetConfigRegistryResponse>) {
    // SAFETY: self and src are both valid `GetConfigRegistryResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetConfigRegistryResponseMut<'msg> {
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

  pub fn to_owned(&self) -> GetConfigRegistryResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetConfigRegistryResponseMut` does not perform any shared mutation.
// - `GetConfigRegistryResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetConfigRegistryResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetConfigRegistryResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetConfigRegistryResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetConfigRegistryResponseMut<'msg> {
  type Proxied = GetConfigRegistryResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetConfigRegistryResponse> {
    GetConfigRegistryResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetConfigRegistryResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetConfigRegistryResponse>
  where
      'msg: 'shorter {
    GetConfigRegistryResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetConfigRegistryResponseMut<'msg> {
  type MutProxied = GetConfigRegistryResponse;
  fn as_mut(&mut self) -> GetConfigRegistryResponseMut<'msg> {
    GetConfigRegistryResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetConfigRegistryResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetConfigRegistryResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetConfigRegistryResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryResponse_new() } } }
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

  pub fn as_view(&self) -> GetConfigRegistryResponseView {
    GetConfigRegistryResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetConfigRegistryResponseMut {
    GetConfigRegistryResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetConfigRegistryResponse

impl ::std::ops::Drop for GetConfigRegistryResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetConfigRegistryResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetConfigRegistryResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetConfigRegistryResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetConfigRegistryResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetConfigRegistryResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_GetConfigRegistryResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfigmanager_GetConfigRegistryResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetConfigRegistryResponseMut<'a> {
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

impl<'a> GetConfigRegistryResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetConfigRegistryResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetConfigRegistryResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetConfigRegistryResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetConfigRegistryResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetConfigRegistryResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetConfigRegistryResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetDefaultConfigIdRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetDefaultConfigIdRequest {}

impl ::std::default::Default for GetDefaultConfigIdRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetDefaultConfigIdRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetDefaultConfigIdRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetDefaultConfigIdRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetDefaultConfigIdRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetDefaultConfigIdRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetDefaultConfigIdRequest {
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
// - `GetDefaultConfigIdRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetDefaultConfigIdRequestMut`.
unsafe impl Sync for GetDefaultConfigIdRequest {}

// SAFETY:
// - `GetDefaultConfigIdRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetDefaultConfigIdRequest {}

impl ::protobuf::Proxied for GetDefaultConfigIdRequest {
  type View<'msg> = GetDefaultConfigIdRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetDefaultConfigIdRequest {}

impl ::protobuf::MutProxied for GetDefaultConfigIdRequest {
  type Mut<'msg> = GetDefaultConfigIdRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetDefaultConfigIdRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetDefaultConfigIdRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetDefaultConfigIdRequestView<'msg> {
  type Message = GetDefaultConfigIdRequest;
}

impl ::std::fmt::Debug for GetDefaultConfigIdRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetDefaultConfigIdRequestView<'_> {
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

impl ::std::default::Default for GetDefaultConfigIdRequestView<'_> {
  fn default() -> GetDefaultConfigIdRequestView<'static> {
    GetDefaultConfigIdRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetDefaultConfigIdRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetDefaultConfigIdRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GetDefaultConfigIdRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetDefaultConfigIdRequestView<'_> {}

// SAFETY:
// - `GetDefaultConfigIdRequestView` is `Send` because while its alive a `GetDefaultConfigIdRequestMut` cannot.
// - `GetDefaultConfigIdRequestView` does not use thread-local data.
unsafe impl Send for GetDefaultConfigIdRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetDefaultConfigIdRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetDefaultConfigIdRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetDefaultConfigIdRequestView<'msg> {
  type Proxied = GetDefaultConfigIdRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetDefaultConfigIdRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetDefaultConfigIdRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetDefaultConfigIdRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetDefaultConfigIdRequest> for GetDefaultConfigIdRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetDefaultConfigIdRequest {
    let dst = GetDefaultConfigIdRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetDefaultConfigIdRequest> for GetDefaultConfigIdRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetDefaultConfigIdRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetDefaultConfigIdRequest {
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
      let prototype = <GetDefaultConfigIdRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetDefaultConfigIdRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetDefaultConfigIdRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetDefaultConfigIdRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetDefaultConfigIdRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetDefaultConfigIdRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetDefaultConfigIdRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetDefaultConfigIdRequestMut<'msg> {
  type Message = GetDefaultConfigIdRequest;
}

impl ::std::fmt::Debug for GetDefaultConfigIdRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetDefaultConfigIdRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetDefaultConfigIdRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetDefaultConfigIdRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetDefaultConfigIdRequest>) {
    // SAFETY: self and src are both valid `GetDefaultConfigIdRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetDefaultConfigIdRequestMut<'msg> {
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

  pub fn to_owned(&self) -> GetDefaultConfigIdRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `GetDefaultConfigIdRequestMut` does not perform any shared mutation.
// - `GetDefaultConfigIdRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetDefaultConfigIdRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetDefaultConfigIdRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetDefaultConfigIdRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetDefaultConfigIdRequestMut<'msg> {
  type Proxied = GetDefaultConfigIdRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetDefaultConfigIdRequest> {
    GetDefaultConfigIdRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetDefaultConfigIdRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetDefaultConfigIdRequest>
  where
      'msg: 'shorter {
    GetDefaultConfigIdRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetDefaultConfigIdRequestMut<'msg> {
  type MutProxied = GetDefaultConfigIdRequest;
  fn as_mut(&mut self) -> GetDefaultConfigIdRequestMut<'msg> {
    GetDefaultConfigIdRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetDefaultConfigIdRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetDefaultConfigIdRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetDefaultConfigIdRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdRequest_new() } } }
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

  pub fn as_view(&self) -> GetDefaultConfigIdRequestView {
    GetDefaultConfigIdRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetDefaultConfigIdRequestMut {
    GetDefaultConfigIdRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl GetDefaultConfigIdRequest

impl ::std::ops::Drop for GetDefaultConfigIdRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetDefaultConfigIdRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetDefaultConfigIdRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetDefaultConfigIdRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetDefaultConfigIdRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetDefaultConfigIdRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> GetDefaultConfigIdRequestMut<'a> {
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

impl<'a> GetDefaultConfigIdRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetDefaultConfigIdRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetDefaultConfigIdRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetDefaultConfigIdRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetDefaultConfigIdRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetDefaultConfigIdRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetDefaultConfigIdRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetDefaultConfigIdResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetDefaultConfigIdResponse {}

impl ::std::default::Default for GetDefaultConfigIdResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetDefaultConfigIdResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetDefaultConfigIdResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetDefaultConfigIdResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetDefaultConfigIdResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetDefaultConfigIdResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetDefaultConfigIdResponse {
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
// - `GetDefaultConfigIdResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetDefaultConfigIdResponseMut`.
unsafe impl Sync for GetDefaultConfigIdResponse {}

// SAFETY:
// - `GetDefaultConfigIdResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetDefaultConfigIdResponse {}

impl ::protobuf::Proxied for GetDefaultConfigIdResponse {
  type View<'msg> = GetDefaultConfigIdResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetDefaultConfigIdResponse {}

impl ::protobuf::MutProxied for GetDefaultConfigIdResponse {
  type Mut<'msg> = GetDefaultConfigIdResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetDefaultConfigIdResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetDefaultConfigIdResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetDefaultConfigIdResponseView<'msg> {
  type Message = GetDefaultConfigIdResponse;
}

impl ::std::fmt::Debug for GetDefaultConfigIdResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetDefaultConfigIdResponseView<'_> {
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

impl ::std::default::Default for GetDefaultConfigIdResponseView<'_> {
  fn default() -> GetDefaultConfigIdResponseView<'static> {
    GetDefaultConfigIdResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetDefaultConfigIdResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetDefaultConfigIdResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional int64
  pub fn result(self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `GetDefaultConfigIdResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetDefaultConfigIdResponseView<'_> {}

// SAFETY:
// - `GetDefaultConfigIdResponseView` is `Send` because while its alive a `GetDefaultConfigIdResponseMut` cannot.
// - `GetDefaultConfigIdResponseView` does not use thread-local data.
unsafe impl Send for GetDefaultConfigIdResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetDefaultConfigIdResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetDefaultConfigIdResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetDefaultConfigIdResponseView<'msg> {
  type Proxied = GetDefaultConfigIdResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetDefaultConfigIdResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetDefaultConfigIdResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetDefaultConfigIdResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetDefaultConfigIdResponse> for GetDefaultConfigIdResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetDefaultConfigIdResponse {
    let dst = GetDefaultConfigIdResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetDefaultConfigIdResponse> for GetDefaultConfigIdResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetDefaultConfigIdResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetDefaultConfigIdResponse {
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
      let prototype = <GetDefaultConfigIdResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetDefaultConfigIdResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetDefaultConfigIdResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetDefaultConfigIdResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetDefaultConfigIdResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetDefaultConfigIdResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetDefaultConfigIdResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetDefaultConfigIdResponseMut<'msg> {
  type Message = GetDefaultConfigIdResponse;
}

impl ::std::fmt::Debug for GetDefaultConfigIdResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetDefaultConfigIdResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetDefaultConfigIdResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetDefaultConfigIdResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetDefaultConfigIdResponse>) {
    // SAFETY: self and src are both valid `GetDefaultConfigIdResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetDefaultConfigIdResponseMut<'msg> {
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

  pub fn to_owned(&self) -> GetDefaultConfigIdResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional int64
  pub fn result(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_get(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `GetDefaultConfigIdResponseMut` does not perform any shared mutation.
// - `GetDefaultConfigIdResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetDefaultConfigIdResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetDefaultConfigIdResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetDefaultConfigIdResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetDefaultConfigIdResponseMut<'msg> {
  type Proxied = GetDefaultConfigIdResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetDefaultConfigIdResponse> {
    GetDefaultConfigIdResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetDefaultConfigIdResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetDefaultConfigIdResponse>
  where
      'msg: 'shorter {
    GetDefaultConfigIdResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetDefaultConfigIdResponseMut<'msg> {
  type MutProxied = GetDefaultConfigIdResponse;
  fn as_mut(&mut self) -> GetDefaultConfigIdResponseMut<'msg> {
    GetDefaultConfigIdResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetDefaultConfigIdResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetDefaultConfigIdResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetDefaultConfigIdResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdResponse_new() } } }
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

  pub fn as_view(&self) -> GetDefaultConfigIdResponseView {
    GetDefaultConfigIdResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetDefaultConfigIdResponseMut {
    GetDefaultConfigIdResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional int64
  pub fn result(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_get(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_set(self.raw_msg(), val) }
  }

}  // impl GetDefaultConfigIdResponse

impl ::std::ops::Drop for GetDefaultConfigIdResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetDefaultConfigIdResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetDefaultConfigIdResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetDefaultConfigIdResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetDefaultConfigIdResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetDefaultConfigIdResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_GetDefaultConfigIdResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szconfigmanager_GetDefaultConfigIdResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

}

impl<'a> GetDefaultConfigIdResponseMut<'a> {
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

impl<'a> GetDefaultConfigIdResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetDefaultConfigIdResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetDefaultConfigIdResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetDefaultConfigIdResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetDefaultConfigIdResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetDefaultConfigIdResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetDefaultConfigIdResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetTemplateConfigRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetTemplateConfigRequest {}

impl ::std::default::Default for GetTemplateConfigRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetTemplateConfigRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetTemplateConfigRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetTemplateConfigRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetTemplateConfigRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetTemplateConfigRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetTemplateConfigRequest {
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
// - `GetTemplateConfigRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetTemplateConfigRequestMut`.
unsafe impl Sync for GetTemplateConfigRequest {}

// SAFETY:
// - `GetTemplateConfigRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetTemplateConfigRequest {}

impl ::protobuf::Proxied for GetTemplateConfigRequest {
  type View<'msg> = GetTemplateConfigRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetTemplateConfigRequest {}

impl ::protobuf::MutProxied for GetTemplateConfigRequest {
  type Mut<'msg> = GetTemplateConfigRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetTemplateConfigRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetTemplateConfigRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetTemplateConfigRequestView<'msg> {
  type Message = GetTemplateConfigRequest;
}

impl ::std::fmt::Debug for GetTemplateConfigRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetTemplateConfigRequestView<'_> {
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

impl ::std::default::Default for GetTemplateConfigRequestView<'_> {
  fn default() -> GetTemplateConfigRequestView<'static> {
    GetTemplateConfigRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetTemplateConfigRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetTemplateConfigRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GetTemplateConfigRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetTemplateConfigRequestView<'_> {}

// SAFETY:
// - `GetTemplateConfigRequestView` is `Send` because while its alive a `GetTemplateConfigRequestMut` cannot.
// - `GetTemplateConfigRequestView` does not use thread-local data.
unsafe impl Send for GetTemplateConfigRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetTemplateConfigRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetTemplateConfigRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetTemplateConfigRequestView<'msg> {
  type Proxied = GetTemplateConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetTemplateConfigRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetTemplateConfigRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetTemplateConfigRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetTemplateConfigRequest> for GetTemplateConfigRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetTemplateConfigRequest {
    let dst = GetTemplateConfigRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetTemplateConfigRequest> for GetTemplateConfigRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetTemplateConfigRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetTemplateConfigRequest {
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
      let prototype = <GetTemplateConfigRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetTemplateConfigRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetTemplateConfigRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetTemplateConfigRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetTemplateConfigRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetTemplateConfigRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetTemplateConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetTemplateConfigRequestMut<'msg> {
  type Message = GetTemplateConfigRequest;
}

impl ::std::fmt::Debug for GetTemplateConfigRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetTemplateConfigRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetTemplateConfigRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetTemplateConfigRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetTemplateConfigRequest>) {
    // SAFETY: self and src are both valid `GetTemplateConfigRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetTemplateConfigRequestMut<'msg> {
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

  pub fn to_owned(&self) -> GetTemplateConfigRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `GetTemplateConfigRequestMut` does not perform any shared mutation.
// - `GetTemplateConfigRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetTemplateConfigRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetTemplateConfigRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetTemplateConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetTemplateConfigRequestMut<'msg> {
  type Proxied = GetTemplateConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetTemplateConfigRequest> {
    GetTemplateConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetTemplateConfigRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetTemplateConfigRequest>
  where
      'msg: 'shorter {
    GetTemplateConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetTemplateConfigRequestMut<'msg> {
  type MutProxied = GetTemplateConfigRequest;
  fn as_mut(&mut self) -> GetTemplateConfigRequestMut<'msg> {
    GetTemplateConfigRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetTemplateConfigRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetTemplateConfigRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetTemplateConfigRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigRequest_new() } } }
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

  pub fn as_view(&self) -> GetTemplateConfigRequestView {
    GetTemplateConfigRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetTemplateConfigRequestMut {
    GetTemplateConfigRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl GetTemplateConfigRequest

impl ::std::ops::Drop for GetTemplateConfigRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetTemplateConfigRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetTemplateConfigRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetTemplateConfigRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetTemplateConfigRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetTemplateConfigRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> GetTemplateConfigRequestMut<'a> {
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

impl<'a> GetTemplateConfigRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetTemplateConfigRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetTemplateConfigRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetTemplateConfigRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetTemplateConfigRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetTemplateConfigRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetTemplateConfigRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetTemplateConfigResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetTemplateConfigResponse {}

impl ::std::default::Default for GetTemplateConfigResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetTemplateConfigResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetTemplateConfigResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetTemplateConfigResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetTemplateConfigResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetTemplateConfigResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetTemplateConfigResponse {
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
// - `GetTemplateConfigResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetTemplateConfigResponseMut`.
unsafe impl Sync for GetTemplateConfigResponse {}

// SAFETY:
// - `GetTemplateConfigResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetTemplateConfigResponse {}

impl ::protobuf::Proxied for GetTemplateConfigResponse {
  type View<'msg> = GetTemplateConfigResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetTemplateConfigResponse {}

impl ::protobuf::MutProxied for GetTemplateConfigResponse {
  type Mut<'msg> = GetTemplateConfigResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetTemplateConfigResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetTemplateConfigResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetTemplateConfigResponseView<'msg> {
  type Message = GetTemplateConfigResponse;
}

impl ::std::fmt::Debug for GetTemplateConfigResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetTemplateConfigResponseView<'_> {
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

impl ::std::default::Default for GetTemplateConfigResponseView<'_> {
  fn default() -> GetTemplateConfigResponseView<'static> {
    GetTemplateConfigResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetTemplateConfigResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetTemplateConfigResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetTemplateConfigResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetTemplateConfigResponseView<'_> {}

// SAFETY:
// - `GetTemplateConfigResponseView` is `Send` because while its alive a `GetTemplateConfigResponseMut` cannot.
// - `GetTemplateConfigResponseView` does not use thread-local data.
unsafe impl Send for GetTemplateConfigResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetTemplateConfigResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetTemplateConfigResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetTemplateConfigResponseView<'msg> {
  type Proxied = GetTemplateConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetTemplateConfigResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetTemplateConfigResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetTemplateConfigResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetTemplateConfigResponse> for GetTemplateConfigResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetTemplateConfigResponse {
    let dst = GetTemplateConfigResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetTemplateConfigResponse> for GetTemplateConfigResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetTemplateConfigResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetTemplateConfigResponse {
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
      let prototype = <GetTemplateConfigResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetTemplateConfigResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetTemplateConfigResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetTemplateConfigResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetTemplateConfigResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetTemplateConfigResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetTemplateConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetTemplateConfigResponseMut<'msg> {
  type Message = GetTemplateConfigResponse;
}

impl ::std::fmt::Debug for GetTemplateConfigResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetTemplateConfigResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetTemplateConfigResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetTemplateConfigResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetTemplateConfigResponse>) {
    // SAFETY: self and src are both valid `GetTemplateConfigResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetTemplateConfigResponseMut<'msg> {
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

  pub fn to_owned(&self) -> GetTemplateConfigResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetTemplateConfigResponseMut` does not perform any shared mutation.
// - `GetTemplateConfigResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetTemplateConfigResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetTemplateConfigResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetTemplateConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetTemplateConfigResponseMut<'msg> {
  type Proxied = GetTemplateConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetTemplateConfigResponse> {
    GetTemplateConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetTemplateConfigResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetTemplateConfigResponse>
  where
      'msg: 'shorter {
    GetTemplateConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetTemplateConfigResponseMut<'msg> {
  type MutProxied = GetTemplateConfigResponse;
  fn as_mut(&mut self) -> GetTemplateConfigResponseMut<'msg> {
    GetTemplateConfigResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetTemplateConfigResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetTemplateConfigResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetTemplateConfigResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigResponse_new() } } }
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

  pub fn as_view(&self) -> GetTemplateConfigResponseView {
    GetTemplateConfigResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetTemplateConfigResponseMut {
    GetTemplateConfigResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetTemplateConfigResponse

impl ::std::ops::Drop for GetTemplateConfigResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetTemplateConfigResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetTemplateConfigResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetTemplateConfigResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetTemplateConfigResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetTemplateConfigResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_GetTemplateConfigResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfigmanager_GetTemplateConfigResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetTemplateConfigResponseMut<'a> {
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

impl<'a> GetTemplateConfigResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetTemplateConfigResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetTemplateConfigResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetTemplateConfigResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetTemplateConfigResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetTemplateConfigResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetTemplateConfigResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct RegisterConfigRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for RegisterConfigRequest {}

impl ::std::default::Default for RegisterConfigRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for RegisterConfigRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for RegisterConfigRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for RegisterConfigRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for RegisterConfigRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for RegisterConfigRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for RegisterConfigRequest {
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
// - `RegisterConfigRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `RegisterConfigRequestMut`.
unsafe impl Sync for RegisterConfigRequest {}

// SAFETY:
// - `RegisterConfigRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RegisterConfigRequest {}

impl ::protobuf::Proxied for RegisterConfigRequest {
  type View<'msg> = RegisterConfigRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RegisterConfigRequest {}

impl ::protobuf::MutProxied for RegisterConfigRequest {
  type Mut<'msg> = RegisterConfigRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RegisterConfigRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegisterConfigRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RegisterConfigRequestView<'msg> {
  type Message = RegisterConfigRequest;
}

impl ::std::fmt::Debug for RegisterConfigRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for RegisterConfigRequestView<'_> {
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

impl ::std::default::Default for RegisterConfigRequestView<'_> {
  fn default() -> RegisterConfigRequestView<'static> {
    RegisterConfigRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_RegisterConfigRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> RegisterConfigRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> RegisterConfigRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_definition: optional string
  pub fn config_definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // config_comment: optional string
  pub fn config_comment(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `RegisterConfigRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for RegisterConfigRequestView<'_> {}

// SAFETY:
// - `RegisterConfigRequestView` is `Send` because while its alive a `RegisterConfigRequestMut` cannot.
// - `RegisterConfigRequestView` does not use thread-local data.
unsafe impl Send for RegisterConfigRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RegisterConfigRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RegisterConfigRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for RegisterConfigRequestView<'msg> {
  type Proxied = RegisterConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, RegisterConfigRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegisterConfigRequestView<'msg> {
  fn into_view<'shorter>(self) -> RegisterConfigRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RegisterConfigRequest> for RegisterConfigRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegisterConfigRequest {
    let dst = RegisterConfigRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RegisterConfigRequest> for RegisterConfigRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegisterConfigRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for RegisterConfigRequest {
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
      let prototype = <RegisterConfigRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for RegisterConfigRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<RegisterConfigRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> RegisterConfigRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { RegisterConfigRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RegisterConfigRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegisterConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RegisterConfigRequestMut<'msg> {
  type Message = RegisterConfigRequest;
}

impl ::std::fmt::Debug for RegisterConfigRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for RegisterConfigRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for RegisterConfigRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for RegisterConfigRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = RegisterConfigRequest>) {
    // SAFETY: self and src are both valid `RegisterConfigRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> RegisterConfigRequestMut<'msg> {
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

  pub fn to_owned(&self) -> RegisterConfigRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // config_comment: optional string
  pub fn config_comment(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_comment(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `RegisterConfigRequestMut` does not perform any shared mutation.
// - `RegisterConfigRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for RegisterConfigRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RegisterConfigRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RegisterConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for RegisterConfigRequestMut<'msg> {
  type Proxied = RegisterConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'_, RegisterConfigRequest> {
    RegisterConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegisterConfigRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RegisterConfigRequest>
  where
      'msg: 'shorter {
    RegisterConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for RegisterConfigRequestMut<'msg> {
  type MutProxied = RegisterConfigRequest;
  fn as_mut(&mut self) -> RegisterConfigRequestMut<'msg> {
    RegisterConfigRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RegisterConfigRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> RegisterConfigRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RegisterConfigRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_RegisterConfigRequest_new() } } }
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

  pub fn as_view(&self) -> RegisterConfigRequestView {
    RegisterConfigRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> RegisterConfigRequestMut {
    RegisterConfigRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // config_comment: optional string
  pub fn config_comment(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_comment(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl RegisterConfigRequest

impl ::std::ops::Drop for RegisterConfigRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for RegisterConfigRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RegisterConfigRequest {
  type Proxied = Self;
  fn as_view(&self) -> RegisterConfigRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RegisterConfigRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RegisterConfigRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_RegisterConfigRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_RegisterConfigRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_definition_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfigmanager_RegisterConfigRequest_config_comment_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> RegisterConfigRequestMut<'a> {
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

impl<'a> RegisterConfigRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for RegisterConfigRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for RegisterConfigRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for RegisterConfigRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for RegisterConfigRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for RegisterConfigRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for RegisterConfigRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct RegisterConfigResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for RegisterConfigResponse {}

impl ::std::default::Default for RegisterConfigResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for RegisterConfigResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for RegisterConfigResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for RegisterConfigResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for RegisterConfigResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for RegisterConfigResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for RegisterConfigResponse {
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
// - `RegisterConfigResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `RegisterConfigResponseMut`.
unsafe impl Sync for RegisterConfigResponse {}

// SAFETY:
// - `RegisterConfigResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RegisterConfigResponse {}

impl ::protobuf::Proxied for RegisterConfigResponse {
  type View<'msg> = RegisterConfigResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RegisterConfigResponse {}

impl ::protobuf::MutProxied for RegisterConfigResponse {
  type Mut<'msg> = RegisterConfigResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RegisterConfigResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegisterConfigResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RegisterConfigResponseView<'msg> {
  type Message = RegisterConfigResponse;
}

impl ::std::fmt::Debug for RegisterConfigResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for RegisterConfigResponseView<'_> {
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

impl ::std::default::Default for RegisterConfigResponseView<'_> {
  fn default() -> RegisterConfigResponseView<'static> {
    RegisterConfigResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_RegisterConfigResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> RegisterConfigResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> RegisterConfigResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional int64
  pub fn result(self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `RegisterConfigResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for RegisterConfigResponseView<'_> {}

// SAFETY:
// - `RegisterConfigResponseView` is `Send` because while its alive a `RegisterConfigResponseMut` cannot.
// - `RegisterConfigResponseView` does not use thread-local data.
unsafe impl Send for RegisterConfigResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RegisterConfigResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RegisterConfigResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for RegisterConfigResponseView<'msg> {
  type Proxied = RegisterConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, RegisterConfigResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegisterConfigResponseView<'msg> {
  fn into_view<'shorter>(self) -> RegisterConfigResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RegisterConfigResponse> for RegisterConfigResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegisterConfigResponse {
    let dst = RegisterConfigResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RegisterConfigResponse> for RegisterConfigResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegisterConfigResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for RegisterConfigResponse {
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
      let prototype = <RegisterConfigResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for RegisterConfigResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<RegisterConfigResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> RegisterConfigResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { RegisterConfigResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RegisterConfigResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegisterConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RegisterConfigResponseMut<'msg> {
  type Message = RegisterConfigResponse;
}

impl ::std::fmt::Debug for RegisterConfigResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for RegisterConfigResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for RegisterConfigResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for RegisterConfigResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = RegisterConfigResponse>) {
    // SAFETY: self and src are both valid `RegisterConfigResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> RegisterConfigResponseMut<'msg> {
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

  pub fn to_owned(&self) -> RegisterConfigResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional int64
  pub fn result(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_get(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `RegisterConfigResponseMut` does not perform any shared mutation.
// - `RegisterConfigResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for RegisterConfigResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RegisterConfigResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RegisterConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for RegisterConfigResponseMut<'msg> {
  type Proxied = RegisterConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'_, RegisterConfigResponse> {
    RegisterConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegisterConfigResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RegisterConfigResponse>
  where
      'msg: 'shorter {
    RegisterConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for RegisterConfigResponseMut<'msg> {
  type MutProxied = RegisterConfigResponse;
  fn as_mut(&mut self) -> RegisterConfigResponseMut<'msg> {
    RegisterConfigResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RegisterConfigResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> RegisterConfigResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RegisterConfigResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_RegisterConfigResponse_new() } } }
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

  pub fn as_view(&self) -> RegisterConfigResponseView {
    RegisterConfigResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> RegisterConfigResponseMut {
    RegisterConfigResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional int64
  pub fn result(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_get(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_set(self.raw_msg(), val) }
  }

}  // impl RegisterConfigResponse

impl ::std::ops::Drop for RegisterConfigResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for RegisterConfigResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RegisterConfigResponse {
  type Proxied = Self;
  fn as_view(&self) -> RegisterConfigResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RegisterConfigResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RegisterConfigResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_RegisterConfigResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_RegisterConfigResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szconfigmanager_RegisterConfigResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

}

impl<'a> RegisterConfigResponseMut<'a> {
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

impl<'a> RegisterConfigResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for RegisterConfigResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for RegisterConfigResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for RegisterConfigResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for RegisterConfigResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for RegisterConfigResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for RegisterConfigResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct ReplaceDefaultConfigIdRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for ReplaceDefaultConfigIdRequest {}

impl ::std::default::Default for ReplaceDefaultConfigIdRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ReplaceDefaultConfigIdRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for ReplaceDefaultConfigIdRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for ReplaceDefaultConfigIdRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for ReplaceDefaultConfigIdRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for ReplaceDefaultConfigIdRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for ReplaceDefaultConfigIdRequest {
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
// - `ReplaceDefaultConfigIdRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ReplaceDefaultConfigIdRequestMut`.
unsafe impl Sync for ReplaceDefaultConfigIdRequest {}

// SAFETY:
// - `ReplaceDefaultConfigIdRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ReplaceDefaultConfigIdRequest {}

impl ::protobuf::Proxied for ReplaceDefaultConfigIdRequest {
  type View<'msg> = ReplaceDefaultConfigIdRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ReplaceDefaultConfigIdRequest {}

impl ::protobuf::MutProxied for ReplaceDefaultConfigIdRequest {
  type Mut<'msg> = ReplaceDefaultConfigIdRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReplaceDefaultConfigIdRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReplaceDefaultConfigIdRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReplaceDefaultConfigIdRequestView<'msg> {
  type Message = ReplaceDefaultConfigIdRequest;
}

impl ::std::fmt::Debug for ReplaceDefaultConfigIdRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ReplaceDefaultConfigIdRequestView<'_> {
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

impl ::std::default::Default for ReplaceDefaultConfigIdRequestView<'_> {
  fn default() -> ReplaceDefaultConfigIdRequestView<'static> {
    ReplaceDefaultConfigIdRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> ReplaceDefaultConfigIdRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> ReplaceDefaultConfigIdRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // current_default_config_id: optional int64
  pub fn current_default_config_id(self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_get(self.raw_msg()) }
  }

  // new_default_config_id: optional int64
  pub fn new_default_config_id(self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `ReplaceDefaultConfigIdRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReplaceDefaultConfigIdRequestView<'_> {}

// SAFETY:
// - `ReplaceDefaultConfigIdRequestView` is `Send` because while its alive a `ReplaceDefaultConfigIdRequestMut` cannot.
// - `ReplaceDefaultConfigIdRequestView` does not use thread-local data.
unsafe impl Send for ReplaceDefaultConfigIdRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ReplaceDefaultConfigIdRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ReplaceDefaultConfigIdRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for ReplaceDefaultConfigIdRequestView<'msg> {
  type Proxied = ReplaceDefaultConfigIdRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ReplaceDefaultConfigIdRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReplaceDefaultConfigIdRequestView<'msg> {
  fn into_view<'shorter>(self) -> ReplaceDefaultConfigIdRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ReplaceDefaultConfigIdRequest> for ReplaceDefaultConfigIdRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReplaceDefaultConfigIdRequest {
    let dst = ReplaceDefaultConfigIdRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ReplaceDefaultConfigIdRequest> for ReplaceDefaultConfigIdRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReplaceDefaultConfigIdRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ReplaceDefaultConfigIdRequest {
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
      let prototype = <ReplaceDefaultConfigIdRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ReplaceDefaultConfigIdRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ReplaceDefaultConfigIdRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ReplaceDefaultConfigIdRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ReplaceDefaultConfigIdRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReplaceDefaultConfigIdRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReplaceDefaultConfigIdRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReplaceDefaultConfigIdRequestMut<'msg> {
  type Message = ReplaceDefaultConfigIdRequest;
}

impl ::std::fmt::Debug for ReplaceDefaultConfigIdRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ReplaceDefaultConfigIdRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for ReplaceDefaultConfigIdRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for ReplaceDefaultConfigIdRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = ReplaceDefaultConfigIdRequest>) {
    // SAFETY: self and src are both valid `ReplaceDefaultConfigIdRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> ReplaceDefaultConfigIdRequestMut<'msg> {
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

  pub fn to_owned(&self) -> ReplaceDefaultConfigIdRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // current_default_config_id: optional int64
  pub fn current_default_config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_get(self.raw_msg()) }
  }
  pub fn set_current_default_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_set(self.raw_msg(), val) }
  }

  // new_default_config_id: optional int64
  pub fn new_default_config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_get(self.raw_msg()) }
  }
  pub fn set_new_default_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `ReplaceDefaultConfigIdRequestMut` does not perform any shared mutation.
// - `ReplaceDefaultConfigIdRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ReplaceDefaultConfigIdRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ReplaceDefaultConfigIdRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ReplaceDefaultConfigIdRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for ReplaceDefaultConfigIdRequestMut<'msg> {
  type Proxied = ReplaceDefaultConfigIdRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ReplaceDefaultConfigIdRequest> {
    ReplaceDefaultConfigIdRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReplaceDefaultConfigIdRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ReplaceDefaultConfigIdRequest>
  where
      'msg: 'shorter {
    ReplaceDefaultConfigIdRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for ReplaceDefaultConfigIdRequestMut<'msg> {
  type MutProxied = ReplaceDefaultConfigIdRequest;
  fn as_mut(&mut self) -> ReplaceDefaultConfigIdRequestMut<'msg> {
    ReplaceDefaultConfigIdRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReplaceDefaultConfigIdRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ReplaceDefaultConfigIdRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ReplaceDefaultConfigIdRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdRequest_new() } } }
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

  pub fn as_view(&self) -> ReplaceDefaultConfigIdRequestView {
    ReplaceDefaultConfigIdRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ReplaceDefaultConfigIdRequestMut {
    ReplaceDefaultConfigIdRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // current_default_config_id: optional int64
  pub fn current_default_config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_get(self.raw_msg()) }
  }
  pub fn set_current_default_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_set(self.raw_msg(), val) }
  }

  // new_default_config_id: optional int64
  pub fn new_default_config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_get(self.raw_msg()) }
  }
  pub fn set_new_default_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_set(self.raw_msg(), val) }
  }

}  // impl ReplaceDefaultConfigIdRequest

impl ::std::ops::Drop for ReplaceDefaultConfigIdRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ReplaceDefaultConfigIdRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ReplaceDefaultConfigIdRequest {
  type Proxied = Self;
  fn as_view(&self) -> ReplaceDefaultConfigIdRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ReplaceDefaultConfigIdRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReplaceDefaultConfigIdRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_current_default_config_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

  fn proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szconfigmanager_ReplaceDefaultConfigIdRequest_new_default_config_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

}

impl<'a> ReplaceDefaultConfigIdRequestMut<'a> {
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

impl<'a> ReplaceDefaultConfigIdRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for ReplaceDefaultConfigIdRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ReplaceDefaultConfigIdRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for ReplaceDefaultConfigIdRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for ReplaceDefaultConfigIdRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for ReplaceDefaultConfigIdRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for ReplaceDefaultConfigIdRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct ReplaceDefaultConfigIdResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for ReplaceDefaultConfigIdResponse {}

impl ::std::default::Default for ReplaceDefaultConfigIdResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ReplaceDefaultConfigIdResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for ReplaceDefaultConfigIdResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for ReplaceDefaultConfigIdResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for ReplaceDefaultConfigIdResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for ReplaceDefaultConfigIdResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for ReplaceDefaultConfigIdResponse {
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
// - `ReplaceDefaultConfigIdResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ReplaceDefaultConfigIdResponseMut`.
unsafe impl Sync for ReplaceDefaultConfigIdResponse {}

// SAFETY:
// - `ReplaceDefaultConfigIdResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ReplaceDefaultConfigIdResponse {}

impl ::protobuf::Proxied for ReplaceDefaultConfigIdResponse {
  type View<'msg> = ReplaceDefaultConfigIdResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ReplaceDefaultConfigIdResponse {}

impl ::protobuf::MutProxied for ReplaceDefaultConfigIdResponse {
  type Mut<'msg> = ReplaceDefaultConfigIdResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReplaceDefaultConfigIdResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReplaceDefaultConfigIdResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReplaceDefaultConfigIdResponseView<'msg> {
  type Message = ReplaceDefaultConfigIdResponse;
}

impl ::std::fmt::Debug for ReplaceDefaultConfigIdResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ReplaceDefaultConfigIdResponseView<'_> {
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

impl ::std::default::Default for ReplaceDefaultConfigIdResponseView<'_> {
  fn default() -> ReplaceDefaultConfigIdResponseView<'static> {
    ReplaceDefaultConfigIdResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> ReplaceDefaultConfigIdResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> ReplaceDefaultConfigIdResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ReplaceDefaultConfigIdResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReplaceDefaultConfigIdResponseView<'_> {}

// SAFETY:
// - `ReplaceDefaultConfigIdResponseView` is `Send` because while its alive a `ReplaceDefaultConfigIdResponseMut` cannot.
// - `ReplaceDefaultConfigIdResponseView` does not use thread-local data.
unsafe impl Send for ReplaceDefaultConfigIdResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ReplaceDefaultConfigIdResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ReplaceDefaultConfigIdResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for ReplaceDefaultConfigIdResponseView<'msg> {
  type Proxied = ReplaceDefaultConfigIdResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ReplaceDefaultConfigIdResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReplaceDefaultConfigIdResponseView<'msg> {
  fn into_view<'shorter>(self) -> ReplaceDefaultConfigIdResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ReplaceDefaultConfigIdResponse> for ReplaceDefaultConfigIdResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReplaceDefaultConfigIdResponse {
    let dst = ReplaceDefaultConfigIdResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ReplaceDefaultConfigIdResponse> for ReplaceDefaultConfigIdResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReplaceDefaultConfigIdResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ReplaceDefaultConfigIdResponse {
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
      let prototype = <ReplaceDefaultConfigIdResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ReplaceDefaultConfigIdResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ReplaceDefaultConfigIdResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ReplaceDefaultConfigIdResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ReplaceDefaultConfigIdResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReplaceDefaultConfigIdResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReplaceDefaultConfigIdResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReplaceDefaultConfigIdResponseMut<'msg> {
  type Message = ReplaceDefaultConfigIdResponse;
}

impl ::std::fmt::Debug for ReplaceDefaultConfigIdResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ReplaceDefaultConfigIdResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for ReplaceDefaultConfigIdResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for ReplaceDefaultConfigIdResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = ReplaceDefaultConfigIdResponse>) {
    // SAFETY: self and src are both valid `ReplaceDefaultConfigIdResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> ReplaceDefaultConfigIdResponseMut<'msg> {
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

  pub fn to_owned(&self) -> ReplaceDefaultConfigIdResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `ReplaceDefaultConfigIdResponseMut` does not perform any shared mutation.
// - `ReplaceDefaultConfigIdResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ReplaceDefaultConfigIdResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ReplaceDefaultConfigIdResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ReplaceDefaultConfigIdResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for ReplaceDefaultConfigIdResponseMut<'msg> {
  type Proxied = ReplaceDefaultConfigIdResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ReplaceDefaultConfigIdResponse> {
    ReplaceDefaultConfigIdResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReplaceDefaultConfigIdResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ReplaceDefaultConfigIdResponse>
  where
      'msg: 'shorter {
    ReplaceDefaultConfigIdResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for ReplaceDefaultConfigIdResponseMut<'msg> {
  type MutProxied = ReplaceDefaultConfigIdResponse;
  fn as_mut(&mut self) -> ReplaceDefaultConfigIdResponseMut<'msg> {
    ReplaceDefaultConfigIdResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReplaceDefaultConfigIdResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ReplaceDefaultConfigIdResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ReplaceDefaultConfigIdResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdResponse_new() } } }
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

  pub fn as_view(&self) -> ReplaceDefaultConfigIdResponseView {
    ReplaceDefaultConfigIdResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ReplaceDefaultConfigIdResponseMut {
    ReplaceDefaultConfigIdResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl ReplaceDefaultConfigIdResponse

impl ::std::ops::Drop for ReplaceDefaultConfigIdResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ReplaceDefaultConfigIdResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ReplaceDefaultConfigIdResponse {
  type Proxied = Self;
  fn as_view(&self) -> ReplaceDefaultConfigIdResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ReplaceDefaultConfigIdResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReplaceDefaultConfigIdResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_ReplaceDefaultConfigIdResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> ReplaceDefaultConfigIdResponseMut<'a> {
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

impl<'a> ReplaceDefaultConfigIdResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for ReplaceDefaultConfigIdResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ReplaceDefaultConfigIdResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for ReplaceDefaultConfigIdResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for ReplaceDefaultConfigIdResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for ReplaceDefaultConfigIdResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for ReplaceDefaultConfigIdResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct SetDefaultConfigRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for SetDefaultConfigRequest {}

impl ::std::default::Default for SetDefaultConfigRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for SetDefaultConfigRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for SetDefaultConfigRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for SetDefaultConfigRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for SetDefaultConfigRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for SetDefaultConfigRequest {
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
// - `SetDefaultConfigRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SetDefaultConfigRequestMut`.
unsafe impl Sync for SetDefaultConfigRequest {}

// SAFETY:
// - `SetDefaultConfigRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetDefaultConfigRequest {}

impl ::protobuf::Proxied for SetDefaultConfigRequest {
  type View<'msg> = SetDefaultConfigRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SetDefaultConfigRequest {}

impl ::protobuf::MutProxied for SetDefaultConfigRequest {
  type Mut<'msg> = SetDefaultConfigRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetDefaultConfigRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetDefaultConfigRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SetDefaultConfigRequestView<'msg> {
  type Message = SetDefaultConfigRequest;
}

impl ::std::fmt::Debug for SetDefaultConfigRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigRequestView<'_> {
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

impl ::std::default::Default for SetDefaultConfigRequestView<'_> {
  fn default() -> SetDefaultConfigRequestView<'static> {
    SetDefaultConfigRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> SetDefaultConfigRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> SetDefaultConfigRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_definition: optional string
  pub fn config_definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // config_comment: optional string
  pub fn config_comment(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `SetDefaultConfigRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetDefaultConfigRequestView<'_> {}

// SAFETY:
// - `SetDefaultConfigRequestView` is `Send` because while its alive a `SetDefaultConfigRequestMut` cannot.
// - `SetDefaultConfigRequestView` does not use thread-local data.
unsafe impl Send for SetDefaultConfigRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SetDefaultConfigRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SetDefaultConfigRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for SetDefaultConfigRequestView<'msg> {
  type Proxied = SetDefaultConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, SetDefaultConfigRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetDefaultConfigRequestView<'msg> {
  fn into_view<'shorter>(self) -> SetDefaultConfigRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SetDefaultConfigRequest> for SetDefaultConfigRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetDefaultConfigRequest {
    let dst = SetDefaultConfigRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SetDefaultConfigRequest> for SetDefaultConfigRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetDefaultConfigRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SetDefaultConfigRequest {
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
      let prototype = <SetDefaultConfigRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for SetDefaultConfigRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<SetDefaultConfigRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SetDefaultConfigRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { SetDefaultConfigRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetDefaultConfigRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetDefaultConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SetDefaultConfigRequestMut<'msg> {
  type Message = SetDefaultConfigRequest;
}

impl ::std::fmt::Debug for SetDefaultConfigRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for SetDefaultConfigRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for SetDefaultConfigRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = SetDefaultConfigRequest>) {
    // SAFETY: self and src are both valid `SetDefaultConfigRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> SetDefaultConfigRequestMut<'msg> {
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

  pub fn to_owned(&self) -> SetDefaultConfigRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // config_comment: optional string
  pub fn config_comment(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_comment(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `SetDefaultConfigRequestMut` does not perform any shared mutation.
// - `SetDefaultConfigRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetDefaultConfigRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SetDefaultConfigRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SetDefaultConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for SetDefaultConfigRequestMut<'msg> {
  type Proxied = SetDefaultConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'_, SetDefaultConfigRequest> {
    SetDefaultConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetDefaultConfigRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SetDefaultConfigRequest>
  where
      'msg: 'shorter {
    SetDefaultConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for SetDefaultConfigRequestMut<'msg> {
  type MutProxied = SetDefaultConfigRequest;
  fn as_mut(&mut self) -> SetDefaultConfigRequestMut<'msg> {
    SetDefaultConfigRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SetDefaultConfigRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> SetDefaultConfigRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SetDefaultConfigRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigRequest_new() } } }
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

  pub fn as_view(&self) -> SetDefaultConfigRequestView {
    SetDefaultConfigRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetDefaultConfigRequestMut {
    SetDefaultConfigRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // config_comment: optional string
  pub fn config_comment(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_comment(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl SetDefaultConfigRequest

impl ::std::ops::Drop for SetDefaultConfigRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for SetDefaultConfigRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SetDefaultConfigRequest {
  type Proxied = Self;
  fn as_view(&self) -> SetDefaultConfigRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SetDefaultConfigRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SetDefaultConfigRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_definition_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfigmanager_SetDefaultConfigRequest_config_comment_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> SetDefaultConfigRequestMut<'a> {
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

impl<'a> SetDefaultConfigRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for SetDefaultConfigRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for SetDefaultConfigRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for SetDefaultConfigRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for SetDefaultConfigRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for SetDefaultConfigRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for SetDefaultConfigRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct SetDefaultConfigResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for SetDefaultConfigResponse {}

impl ::std::default::Default for SetDefaultConfigResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for SetDefaultConfigResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for SetDefaultConfigResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for SetDefaultConfigResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for SetDefaultConfigResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for SetDefaultConfigResponse {
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
// - `SetDefaultConfigResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SetDefaultConfigResponseMut`.
unsafe impl Sync for SetDefaultConfigResponse {}

// SAFETY:
// - `SetDefaultConfigResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetDefaultConfigResponse {}

impl ::protobuf::Proxied for SetDefaultConfigResponse {
  type View<'msg> = SetDefaultConfigResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SetDefaultConfigResponse {}

impl ::protobuf::MutProxied for SetDefaultConfigResponse {
  type Mut<'msg> = SetDefaultConfigResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetDefaultConfigResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetDefaultConfigResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SetDefaultConfigResponseView<'msg> {
  type Message = SetDefaultConfigResponse;
}

impl ::std::fmt::Debug for SetDefaultConfigResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigResponseView<'_> {
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

impl ::std::default::Default for SetDefaultConfigResponseView<'_> {
  fn default() -> SetDefaultConfigResponseView<'static> {
    SetDefaultConfigResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> SetDefaultConfigResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> SetDefaultConfigResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional int64
  pub fn result(self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `SetDefaultConfigResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetDefaultConfigResponseView<'_> {}

// SAFETY:
// - `SetDefaultConfigResponseView` is `Send` because while its alive a `SetDefaultConfigResponseMut` cannot.
// - `SetDefaultConfigResponseView` does not use thread-local data.
unsafe impl Send for SetDefaultConfigResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SetDefaultConfigResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SetDefaultConfigResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for SetDefaultConfigResponseView<'msg> {
  type Proxied = SetDefaultConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, SetDefaultConfigResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetDefaultConfigResponseView<'msg> {
  fn into_view<'shorter>(self) -> SetDefaultConfigResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SetDefaultConfigResponse> for SetDefaultConfigResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetDefaultConfigResponse {
    let dst = SetDefaultConfigResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SetDefaultConfigResponse> for SetDefaultConfigResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetDefaultConfigResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SetDefaultConfigResponse {
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
      let prototype = <SetDefaultConfigResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for SetDefaultConfigResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<SetDefaultConfigResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SetDefaultConfigResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { SetDefaultConfigResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetDefaultConfigResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetDefaultConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SetDefaultConfigResponseMut<'msg> {
  type Message = SetDefaultConfigResponse;
}

impl ::std::fmt::Debug for SetDefaultConfigResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for SetDefaultConfigResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for SetDefaultConfigResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = SetDefaultConfigResponse>) {
    // SAFETY: self and src are both valid `SetDefaultConfigResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> SetDefaultConfigResponseMut<'msg> {
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

  pub fn to_owned(&self) -> SetDefaultConfigResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional int64
  pub fn result(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_get(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `SetDefaultConfigResponseMut` does not perform any shared mutation.
// - `SetDefaultConfigResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetDefaultConfigResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SetDefaultConfigResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SetDefaultConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for SetDefaultConfigResponseMut<'msg> {
  type Proxied = SetDefaultConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'_, SetDefaultConfigResponse> {
    SetDefaultConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetDefaultConfigResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SetDefaultConfigResponse>
  where
      'msg: 'shorter {
    SetDefaultConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for SetDefaultConfigResponseMut<'msg> {
  type MutProxied = SetDefaultConfigResponse;
  fn as_mut(&mut self) -> SetDefaultConfigResponseMut<'msg> {
    SetDefaultConfigResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SetDefaultConfigResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> SetDefaultConfigResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SetDefaultConfigResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigResponse_new() } } }
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

  pub fn as_view(&self) -> SetDefaultConfigResponseView {
    SetDefaultConfigResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetDefaultConfigResponseMut {
    SetDefaultConfigResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional int64
  pub fn result(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_get(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_set(self.raw_msg(), val) }
  }

}  // impl SetDefaultConfigResponse

impl ::std::ops::Drop for SetDefaultConfigResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for SetDefaultConfigResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SetDefaultConfigResponse {
  type Proxied = Self;
  fn as_view(&self) -> SetDefaultConfigResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SetDefaultConfigResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SetDefaultConfigResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szconfigmanager_SetDefaultConfigResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

}

impl<'a> SetDefaultConfigResponseMut<'a> {
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

impl<'a> SetDefaultConfigResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for SetDefaultConfigResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for SetDefaultConfigResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for SetDefaultConfigResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for SetDefaultConfigResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for SetDefaultConfigResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for SetDefaultConfigResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct SetDefaultConfigIdRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for SetDefaultConfigIdRequest {}

impl ::std::default::Default for SetDefaultConfigIdRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for SetDefaultConfigIdRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for SetDefaultConfigIdRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for SetDefaultConfigIdRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigIdRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for SetDefaultConfigIdRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for SetDefaultConfigIdRequest {
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
// - `SetDefaultConfigIdRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SetDefaultConfigIdRequestMut`.
unsafe impl Sync for SetDefaultConfigIdRequest {}

// SAFETY:
// - `SetDefaultConfigIdRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetDefaultConfigIdRequest {}

impl ::protobuf::Proxied for SetDefaultConfigIdRequest {
  type View<'msg> = SetDefaultConfigIdRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SetDefaultConfigIdRequest {}

impl ::protobuf::MutProxied for SetDefaultConfigIdRequest {
  type Mut<'msg> = SetDefaultConfigIdRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetDefaultConfigIdRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetDefaultConfigIdRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SetDefaultConfigIdRequestView<'msg> {
  type Message = SetDefaultConfigIdRequest;
}

impl ::std::fmt::Debug for SetDefaultConfigIdRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigIdRequestView<'_> {
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

impl ::std::default::Default for SetDefaultConfigIdRequestView<'_> {
  fn default() -> SetDefaultConfigIdRequestView<'static> {
    SetDefaultConfigIdRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> SetDefaultConfigIdRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> SetDefaultConfigIdRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_id: optional int64
  pub fn config_id(self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `SetDefaultConfigIdRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetDefaultConfigIdRequestView<'_> {}

// SAFETY:
// - `SetDefaultConfigIdRequestView` is `Send` because while its alive a `SetDefaultConfigIdRequestMut` cannot.
// - `SetDefaultConfigIdRequestView` does not use thread-local data.
unsafe impl Send for SetDefaultConfigIdRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SetDefaultConfigIdRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SetDefaultConfigIdRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for SetDefaultConfigIdRequestView<'msg> {
  type Proxied = SetDefaultConfigIdRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, SetDefaultConfigIdRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetDefaultConfigIdRequestView<'msg> {
  fn into_view<'shorter>(self) -> SetDefaultConfigIdRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SetDefaultConfigIdRequest> for SetDefaultConfigIdRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetDefaultConfigIdRequest {
    let dst = SetDefaultConfigIdRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SetDefaultConfigIdRequest> for SetDefaultConfigIdRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetDefaultConfigIdRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SetDefaultConfigIdRequest {
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
      let prototype = <SetDefaultConfigIdRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for SetDefaultConfigIdRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<SetDefaultConfigIdRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SetDefaultConfigIdRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { SetDefaultConfigIdRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetDefaultConfigIdRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetDefaultConfigIdRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SetDefaultConfigIdRequestMut<'msg> {
  type Message = SetDefaultConfigIdRequest;
}

impl ::std::fmt::Debug for SetDefaultConfigIdRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigIdRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for SetDefaultConfigIdRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for SetDefaultConfigIdRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = SetDefaultConfigIdRequest>) {
    // SAFETY: self and src are both valid `SetDefaultConfigIdRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> SetDefaultConfigIdRequestMut<'msg> {
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

  pub fn to_owned(&self) -> SetDefaultConfigIdRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_id: optional int64
  pub fn config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_get(self.raw_msg()) }
  }
  pub fn set_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `SetDefaultConfigIdRequestMut` does not perform any shared mutation.
// - `SetDefaultConfigIdRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetDefaultConfigIdRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SetDefaultConfigIdRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SetDefaultConfigIdRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for SetDefaultConfigIdRequestMut<'msg> {
  type Proxied = SetDefaultConfigIdRequest;
  fn as_view(&self) -> ::protobuf::View<'_, SetDefaultConfigIdRequest> {
    SetDefaultConfigIdRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetDefaultConfigIdRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SetDefaultConfigIdRequest>
  where
      'msg: 'shorter {
    SetDefaultConfigIdRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for SetDefaultConfigIdRequestMut<'msg> {
  type MutProxied = SetDefaultConfigIdRequest;
  fn as_mut(&mut self) -> SetDefaultConfigIdRequestMut<'msg> {
    SetDefaultConfigIdRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SetDefaultConfigIdRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> SetDefaultConfigIdRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SetDefaultConfigIdRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdRequest_new() } } }
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

  pub fn as_view(&self) -> SetDefaultConfigIdRequestView {
    SetDefaultConfigIdRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetDefaultConfigIdRequestMut {
    SetDefaultConfigIdRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_id: optional int64
  pub fn config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_get(self.raw_msg()) }
  }
  pub fn set_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_set(self.raw_msg(), val) }
  }

}  // impl SetDefaultConfigIdRequest

impl ::std::ops::Drop for SetDefaultConfigIdRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for SetDefaultConfigIdRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SetDefaultConfigIdRequest {
  type Proxied = Self;
  fn as_view(&self) -> SetDefaultConfigIdRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SetDefaultConfigIdRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SetDefaultConfigIdRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szconfigmanager_SetDefaultConfigIdRequest_config_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

}

impl<'a> SetDefaultConfigIdRequestMut<'a> {
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

impl<'a> SetDefaultConfigIdRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for SetDefaultConfigIdRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for SetDefaultConfigIdRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for SetDefaultConfigIdRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for SetDefaultConfigIdRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for SetDefaultConfigIdRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for SetDefaultConfigIdRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct SetDefaultConfigIdResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for SetDefaultConfigIdResponse {}

impl ::std::default::Default for SetDefaultConfigIdResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for SetDefaultConfigIdResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for SetDefaultConfigIdResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for SetDefaultConfigIdResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigIdResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for SetDefaultConfigIdResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for SetDefaultConfigIdResponse {
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
// - `SetDefaultConfigIdResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SetDefaultConfigIdResponseMut`.
unsafe impl Sync for SetDefaultConfigIdResponse {}

// SAFETY:
// - `SetDefaultConfigIdResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetDefaultConfigIdResponse {}

impl ::protobuf::Proxied for SetDefaultConfigIdResponse {
  type View<'msg> = SetDefaultConfigIdResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SetDefaultConfigIdResponse {}

impl ::protobuf::MutProxied for SetDefaultConfigIdResponse {
  type Mut<'msg> = SetDefaultConfigIdResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetDefaultConfigIdResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetDefaultConfigIdResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SetDefaultConfigIdResponseView<'msg> {
  type Message = SetDefaultConfigIdResponse;
}

impl ::std::fmt::Debug for SetDefaultConfigIdResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigIdResponseView<'_> {
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

impl ::std::default::Default for SetDefaultConfigIdResponseView<'_> {
  fn default() -> SetDefaultConfigIdResponseView<'static> {
    SetDefaultConfigIdResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> SetDefaultConfigIdResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> SetDefaultConfigIdResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `SetDefaultConfigIdResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetDefaultConfigIdResponseView<'_> {}

// SAFETY:
// - `SetDefaultConfigIdResponseView` is `Send` because while its alive a `SetDefaultConfigIdResponseMut` cannot.
// - `SetDefaultConfigIdResponseView` does not use thread-local data.
unsafe impl Send for SetDefaultConfigIdResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SetDefaultConfigIdResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SetDefaultConfigIdResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for SetDefaultConfigIdResponseView<'msg> {
  type Proxied = SetDefaultConfigIdResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, SetDefaultConfigIdResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetDefaultConfigIdResponseView<'msg> {
  fn into_view<'shorter>(self) -> SetDefaultConfigIdResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SetDefaultConfigIdResponse> for SetDefaultConfigIdResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetDefaultConfigIdResponse {
    let dst = SetDefaultConfigIdResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SetDefaultConfigIdResponse> for SetDefaultConfigIdResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetDefaultConfigIdResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SetDefaultConfigIdResponse {
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
      let prototype = <SetDefaultConfigIdResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for SetDefaultConfigIdResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<SetDefaultConfigIdResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SetDefaultConfigIdResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { SetDefaultConfigIdResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetDefaultConfigIdResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetDefaultConfigIdResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SetDefaultConfigIdResponseMut<'msg> {
  type Message = SetDefaultConfigIdResponse;
}

impl ::std::fmt::Debug for SetDefaultConfigIdResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SetDefaultConfigIdResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for SetDefaultConfigIdResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for SetDefaultConfigIdResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = SetDefaultConfigIdResponse>) {
    // SAFETY: self and src are both valid `SetDefaultConfigIdResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> SetDefaultConfigIdResponseMut<'msg> {
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

  pub fn to_owned(&self) -> SetDefaultConfigIdResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `SetDefaultConfigIdResponseMut` does not perform any shared mutation.
// - `SetDefaultConfigIdResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetDefaultConfigIdResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SetDefaultConfigIdResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SetDefaultConfigIdResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for SetDefaultConfigIdResponseMut<'msg> {
  type Proxied = SetDefaultConfigIdResponse;
  fn as_view(&self) -> ::protobuf::View<'_, SetDefaultConfigIdResponse> {
    SetDefaultConfigIdResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetDefaultConfigIdResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SetDefaultConfigIdResponse>
  where
      'msg: 'shorter {
    SetDefaultConfigIdResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for SetDefaultConfigIdResponseMut<'msg> {
  type MutProxied = SetDefaultConfigIdResponse;
  fn as_mut(&mut self) -> SetDefaultConfigIdResponseMut<'msg> {
    SetDefaultConfigIdResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SetDefaultConfigIdResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> SetDefaultConfigIdResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SetDefaultConfigIdResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdResponse_new() } } }
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

  pub fn as_view(&self) -> SetDefaultConfigIdResponseView {
    SetDefaultConfigIdResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetDefaultConfigIdResponseMut {
    SetDefaultConfigIdResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl SetDefaultConfigIdResponse

impl ::std::ops::Drop for SetDefaultConfigIdResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for SetDefaultConfigIdResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SetDefaultConfigIdResponse {
  type Proxied = Self;
  fn as_view(&self) -> SetDefaultConfigIdResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SetDefaultConfigIdResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SetDefaultConfigIdResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfigmanager_SetDefaultConfigIdResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> SetDefaultConfigIdResponseMut<'a> {
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

impl<'a> SetDefaultConfigIdResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for SetDefaultConfigIdResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for SetDefaultConfigIdResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for SetDefaultConfigIdResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for SetDefaultConfigIdResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for SetDefaultConfigIdResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for SetDefaultConfigIdResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

