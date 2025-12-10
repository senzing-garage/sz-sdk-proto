const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.30.2-beta2");
#[allow(non_camel_case_types)]
pub struct GetDataSourceRegistryRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetDataSourceRegistryRequest {}

impl ::std::default::Default for GetDataSourceRegistryRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetDataSourceRegistryRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetDataSourceRegistryRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetDataSourceRegistryRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetDataSourceRegistryRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetDataSourceRegistryRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetDataSourceRegistryRequest {
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
// - `GetDataSourceRegistryRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetDataSourceRegistryRequestMut`.
unsafe impl Sync for GetDataSourceRegistryRequest {}

// SAFETY:
// - `GetDataSourceRegistryRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetDataSourceRegistryRequest {}

impl ::protobuf::Proxied for GetDataSourceRegistryRequest {
  type View<'msg> = GetDataSourceRegistryRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetDataSourceRegistryRequest {}

impl ::protobuf::MutProxied for GetDataSourceRegistryRequest {
  type Mut<'msg> = GetDataSourceRegistryRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetDataSourceRegistryRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetDataSourceRegistryRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetDataSourceRegistryRequestView<'msg> {
  type Message = GetDataSourceRegistryRequest;
}

impl ::std::fmt::Debug for GetDataSourceRegistryRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetDataSourceRegistryRequestView<'_> {
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

impl ::std::default::Default for GetDataSourceRegistryRequestView<'_> {
  fn default() -> GetDataSourceRegistryRequestView<'static> {
    GetDataSourceRegistryRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetDataSourceRegistryRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetDataSourceRegistryRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_definition: optional string
  pub fn config_definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetDataSourceRegistryRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetDataSourceRegistryRequestView<'_> {}

// SAFETY:
// - `GetDataSourceRegistryRequestView` is `Send` because while its alive a `GetDataSourceRegistryRequestMut` cannot.
// - `GetDataSourceRegistryRequestView` does not use thread-local data.
unsafe impl Send for GetDataSourceRegistryRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetDataSourceRegistryRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetDataSourceRegistryRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetDataSourceRegistryRequestView<'msg> {
  type Proxied = GetDataSourceRegistryRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetDataSourceRegistryRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetDataSourceRegistryRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetDataSourceRegistryRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetDataSourceRegistryRequest> for GetDataSourceRegistryRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetDataSourceRegistryRequest {
    let dst = GetDataSourceRegistryRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetDataSourceRegistryRequest> for GetDataSourceRegistryRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetDataSourceRegistryRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetDataSourceRegistryRequest {
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
      let prototype = <GetDataSourceRegistryRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetDataSourceRegistryRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetDataSourceRegistryRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetDataSourceRegistryRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetDataSourceRegistryRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetDataSourceRegistryRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetDataSourceRegistryRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetDataSourceRegistryRequestMut<'msg> {
  type Message = GetDataSourceRegistryRequest;
}

impl ::std::fmt::Debug for GetDataSourceRegistryRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetDataSourceRegistryRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetDataSourceRegistryRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetDataSourceRegistryRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetDataSourceRegistryRequest>) {
    // SAFETY: self and src are both valid `GetDataSourceRegistryRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetDataSourceRegistryRequestMut<'msg> {
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

  pub fn to_owned(&self) -> GetDataSourceRegistryRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetDataSourceRegistryRequestMut` does not perform any shared mutation.
// - `GetDataSourceRegistryRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetDataSourceRegistryRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetDataSourceRegistryRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetDataSourceRegistryRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetDataSourceRegistryRequestMut<'msg> {
  type Proxied = GetDataSourceRegistryRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetDataSourceRegistryRequest> {
    GetDataSourceRegistryRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetDataSourceRegistryRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetDataSourceRegistryRequest>
  where
      'msg: 'shorter {
    GetDataSourceRegistryRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetDataSourceRegistryRequestMut<'msg> {
  type MutProxied = GetDataSourceRegistryRequest;
  fn as_mut(&mut self) -> GetDataSourceRegistryRequestMut<'msg> {
    GetDataSourceRegistryRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetDataSourceRegistryRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetDataSourceRegistryRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetDataSourceRegistryRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryRequest_new() } } }
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

  pub fn as_view(&self) -> GetDataSourceRegistryRequestView {
    GetDataSourceRegistryRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetDataSourceRegistryRequestMut {
    GetDataSourceRegistryRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetDataSourceRegistryRequest

impl ::std::ops::Drop for GetDataSourceRegistryRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetDataSourceRegistryRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetDataSourceRegistryRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetDataSourceRegistryRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetDataSourceRegistryRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetDataSourceRegistryRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_GetDataSourceRegistryRequest_config_definition_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetDataSourceRegistryRequestMut<'a> {
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

impl<'a> GetDataSourceRegistryRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetDataSourceRegistryRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetDataSourceRegistryRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetDataSourceRegistryRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetDataSourceRegistryRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetDataSourceRegistryRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetDataSourceRegistryRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetDataSourceRegistryResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetDataSourceRegistryResponse {}

impl ::std::default::Default for GetDataSourceRegistryResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetDataSourceRegistryResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetDataSourceRegistryResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetDataSourceRegistryResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetDataSourceRegistryResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetDataSourceRegistryResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetDataSourceRegistryResponse {
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
// - `GetDataSourceRegistryResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetDataSourceRegistryResponseMut`.
unsafe impl Sync for GetDataSourceRegistryResponse {}

// SAFETY:
// - `GetDataSourceRegistryResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetDataSourceRegistryResponse {}

impl ::protobuf::Proxied for GetDataSourceRegistryResponse {
  type View<'msg> = GetDataSourceRegistryResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetDataSourceRegistryResponse {}

impl ::protobuf::MutProxied for GetDataSourceRegistryResponse {
  type Mut<'msg> = GetDataSourceRegistryResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetDataSourceRegistryResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetDataSourceRegistryResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetDataSourceRegistryResponseView<'msg> {
  type Message = GetDataSourceRegistryResponse;
}

impl ::std::fmt::Debug for GetDataSourceRegistryResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetDataSourceRegistryResponseView<'_> {
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

impl ::std::default::Default for GetDataSourceRegistryResponseView<'_> {
  fn default() -> GetDataSourceRegistryResponseView<'static> {
    GetDataSourceRegistryResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetDataSourceRegistryResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetDataSourceRegistryResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetDataSourceRegistryResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetDataSourceRegistryResponseView<'_> {}

// SAFETY:
// - `GetDataSourceRegistryResponseView` is `Send` because while its alive a `GetDataSourceRegistryResponseMut` cannot.
// - `GetDataSourceRegistryResponseView` does not use thread-local data.
unsafe impl Send for GetDataSourceRegistryResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetDataSourceRegistryResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetDataSourceRegistryResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetDataSourceRegistryResponseView<'msg> {
  type Proxied = GetDataSourceRegistryResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetDataSourceRegistryResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetDataSourceRegistryResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetDataSourceRegistryResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetDataSourceRegistryResponse> for GetDataSourceRegistryResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetDataSourceRegistryResponse {
    let dst = GetDataSourceRegistryResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetDataSourceRegistryResponse> for GetDataSourceRegistryResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetDataSourceRegistryResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetDataSourceRegistryResponse {
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
      let prototype = <GetDataSourceRegistryResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetDataSourceRegistryResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetDataSourceRegistryResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetDataSourceRegistryResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetDataSourceRegistryResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetDataSourceRegistryResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetDataSourceRegistryResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetDataSourceRegistryResponseMut<'msg> {
  type Message = GetDataSourceRegistryResponse;
}

impl ::std::fmt::Debug for GetDataSourceRegistryResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetDataSourceRegistryResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetDataSourceRegistryResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetDataSourceRegistryResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetDataSourceRegistryResponse>) {
    // SAFETY: self and src are both valid `GetDataSourceRegistryResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetDataSourceRegistryResponseMut<'msg> {
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

  pub fn to_owned(&self) -> GetDataSourceRegistryResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetDataSourceRegistryResponseMut` does not perform any shared mutation.
// - `GetDataSourceRegistryResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetDataSourceRegistryResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetDataSourceRegistryResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetDataSourceRegistryResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetDataSourceRegistryResponseMut<'msg> {
  type Proxied = GetDataSourceRegistryResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetDataSourceRegistryResponse> {
    GetDataSourceRegistryResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetDataSourceRegistryResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetDataSourceRegistryResponse>
  where
      'msg: 'shorter {
    GetDataSourceRegistryResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetDataSourceRegistryResponseMut<'msg> {
  type MutProxied = GetDataSourceRegistryResponse;
  fn as_mut(&mut self) -> GetDataSourceRegistryResponseMut<'msg> {
    GetDataSourceRegistryResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetDataSourceRegistryResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetDataSourceRegistryResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetDataSourceRegistryResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryResponse_new() } } }
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

  pub fn as_view(&self) -> GetDataSourceRegistryResponseView {
    GetDataSourceRegistryResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetDataSourceRegistryResponseMut {
    GetDataSourceRegistryResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetDataSourceRegistryResponse

impl ::std::ops::Drop for GetDataSourceRegistryResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetDataSourceRegistryResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetDataSourceRegistryResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetDataSourceRegistryResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetDataSourceRegistryResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetDataSourceRegistryResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfig_GetDataSourceRegistryResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_GetDataSourceRegistryResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetDataSourceRegistryResponseMut<'a> {
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

impl<'a> GetDataSourceRegistryResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetDataSourceRegistryResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetDataSourceRegistryResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetDataSourceRegistryResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetDataSourceRegistryResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetDataSourceRegistryResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetDataSourceRegistryResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct RegisterDataSourceRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for RegisterDataSourceRequest {}

impl ::std::default::Default for RegisterDataSourceRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for RegisterDataSourceRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for RegisterDataSourceRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for RegisterDataSourceRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for RegisterDataSourceRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for RegisterDataSourceRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for RegisterDataSourceRequest {
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
// - `RegisterDataSourceRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `RegisterDataSourceRequestMut`.
unsafe impl Sync for RegisterDataSourceRequest {}

// SAFETY:
// - `RegisterDataSourceRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RegisterDataSourceRequest {}

impl ::protobuf::Proxied for RegisterDataSourceRequest {
  type View<'msg> = RegisterDataSourceRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RegisterDataSourceRequest {}

impl ::protobuf::MutProxied for RegisterDataSourceRequest {
  type Mut<'msg> = RegisterDataSourceRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RegisterDataSourceRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegisterDataSourceRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RegisterDataSourceRequestView<'msg> {
  type Message = RegisterDataSourceRequest;
}

impl ::std::fmt::Debug for RegisterDataSourceRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for RegisterDataSourceRequestView<'_> {
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

impl ::std::default::Default for RegisterDataSourceRequestView<'_> {
  fn default() -> RegisterDataSourceRequestView<'static> {
    RegisterDataSourceRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfig_RegisterDataSourceRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> RegisterDataSourceRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> RegisterDataSourceRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_definition: optional string
  pub fn config_definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // data_source_code: optional string
  pub fn data_source_code(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `RegisterDataSourceRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for RegisterDataSourceRequestView<'_> {}

// SAFETY:
// - `RegisterDataSourceRequestView` is `Send` because while its alive a `RegisterDataSourceRequestMut` cannot.
// - `RegisterDataSourceRequestView` does not use thread-local data.
unsafe impl Send for RegisterDataSourceRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RegisterDataSourceRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RegisterDataSourceRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for RegisterDataSourceRequestView<'msg> {
  type Proxied = RegisterDataSourceRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, RegisterDataSourceRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegisterDataSourceRequestView<'msg> {
  fn into_view<'shorter>(self) -> RegisterDataSourceRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RegisterDataSourceRequest> for RegisterDataSourceRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegisterDataSourceRequest {
    let dst = RegisterDataSourceRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RegisterDataSourceRequest> for RegisterDataSourceRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegisterDataSourceRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for RegisterDataSourceRequest {
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
      let prototype = <RegisterDataSourceRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for RegisterDataSourceRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<RegisterDataSourceRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> RegisterDataSourceRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { RegisterDataSourceRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RegisterDataSourceRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegisterDataSourceRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RegisterDataSourceRequestMut<'msg> {
  type Message = RegisterDataSourceRequest;
}

impl ::std::fmt::Debug for RegisterDataSourceRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for RegisterDataSourceRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for RegisterDataSourceRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for RegisterDataSourceRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = RegisterDataSourceRequest>) {
    // SAFETY: self and src are both valid `RegisterDataSourceRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> RegisterDataSourceRequestMut<'msg> {
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

  pub fn to_owned(&self) -> RegisterDataSourceRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // data_source_code: optional string
  pub fn data_source_code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_source_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `RegisterDataSourceRequestMut` does not perform any shared mutation.
// - `RegisterDataSourceRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for RegisterDataSourceRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RegisterDataSourceRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RegisterDataSourceRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for RegisterDataSourceRequestMut<'msg> {
  type Proxied = RegisterDataSourceRequest;
  fn as_view(&self) -> ::protobuf::View<'_, RegisterDataSourceRequest> {
    RegisterDataSourceRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegisterDataSourceRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RegisterDataSourceRequest>
  where
      'msg: 'shorter {
    RegisterDataSourceRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for RegisterDataSourceRequestMut<'msg> {
  type MutProxied = RegisterDataSourceRequest;
  fn as_mut(&mut self) -> RegisterDataSourceRequestMut<'msg> {
    RegisterDataSourceRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RegisterDataSourceRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> RegisterDataSourceRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RegisterDataSourceRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfig_RegisterDataSourceRequest_new() } } }
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

  pub fn as_view(&self) -> RegisterDataSourceRequestView {
    RegisterDataSourceRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> RegisterDataSourceRequestMut {
    RegisterDataSourceRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // data_source_code: optional string
  pub fn data_source_code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_source_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl RegisterDataSourceRequest

impl ::std::ops::Drop for RegisterDataSourceRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for RegisterDataSourceRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RegisterDataSourceRequest {
  type Proxied = Self;
  fn as_view(&self) -> RegisterDataSourceRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RegisterDataSourceRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RegisterDataSourceRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfig_RegisterDataSourceRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfig_RegisterDataSourceRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_RegisterDataSourceRequest_config_definition_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_RegisterDataSourceRequest_data_source_code_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> RegisterDataSourceRequestMut<'a> {
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

impl<'a> RegisterDataSourceRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for RegisterDataSourceRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for RegisterDataSourceRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for RegisterDataSourceRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for RegisterDataSourceRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for RegisterDataSourceRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for RegisterDataSourceRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct RegisterDataSourceResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for RegisterDataSourceResponse {}

impl ::std::default::Default for RegisterDataSourceResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for RegisterDataSourceResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for RegisterDataSourceResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for RegisterDataSourceResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for RegisterDataSourceResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for RegisterDataSourceResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for RegisterDataSourceResponse {
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
// - `RegisterDataSourceResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `RegisterDataSourceResponseMut`.
unsafe impl Sync for RegisterDataSourceResponse {}

// SAFETY:
// - `RegisterDataSourceResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RegisterDataSourceResponse {}

impl ::protobuf::Proxied for RegisterDataSourceResponse {
  type View<'msg> = RegisterDataSourceResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RegisterDataSourceResponse {}

impl ::protobuf::MutProxied for RegisterDataSourceResponse {
  type Mut<'msg> = RegisterDataSourceResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RegisterDataSourceResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegisterDataSourceResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RegisterDataSourceResponseView<'msg> {
  type Message = RegisterDataSourceResponse;
}

impl ::std::fmt::Debug for RegisterDataSourceResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for RegisterDataSourceResponseView<'_> {
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

impl ::std::default::Default for RegisterDataSourceResponseView<'_> {
  fn default() -> RegisterDataSourceResponseView<'static> {
    RegisterDataSourceResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfig_RegisterDataSourceResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> RegisterDataSourceResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> RegisterDataSourceResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // config_definition: optional string
  pub fn config_definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `RegisterDataSourceResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for RegisterDataSourceResponseView<'_> {}

// SAFETY:
// - `RegisterDataSourceResponseView` is `Send` because while its alive a `RegisterDataSourceResponseMut` cannot.
// - `RegisterDataSourceResponseView` does not use thread-local data.
unsafe impl Send for RegisterDataSourceResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RegisterDataSourceResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RegisterDataSourceResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for RegisterDataSourceResponseView<'msg> {
  type Proxied = RegisterDataSourceResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, RegisterDataSourceResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegisterDataSourceResponseView<'msg> {
  fn into_view<'shorter>(self) -> RegisterDataSourceResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RegisterDataSourceResponse> for RegisterDataSourceResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegisterDataSourceResponse {
    let dst = RegisterDataSourceResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RegisterDataSourceResponse> for RegisterDataSourceResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegisterDataSourceResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for RegisterDataSourceResponse {
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
      let prototype = <RegisterDataSourceResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for RegisterDataSourceResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<RegisterDataSourceResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> RegisterDataSourceResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { RegisterDataSourceResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RegisterDataSourceResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegisterDataSourceResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RegisterDataSourceResponseMut<'msg> {
  type Message = RegisterDataSourceResponse;
}

impl ::std::fmt::Debug for RegisterDataSourceResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for RegisterDataSourceResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for RegisterDataSourceResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for RegisterDataSourceResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = RegisterDataSourceResponse>) {
    // SAFETY: self and src are both valid `RegisterDataSourceResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> RegisterDataSourceResponseMut<'msg> {
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

  pub fn to_owned(&self) -> RegisterDataSourceResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `RegisterDataSourceResponseMut` does not perform any shared mutation.
// - `RegisterDataSourceResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for RegisterDataSourceResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RegisterDataSourceResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RegisterDataSourceResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for RegisterDataSourceResponseMut<'msg> {
  type Proxied = RegisterDataSourceResponse;
  fn as_view(&self) -> ::protobuf::View<'_, RegisterDataSourceResponse> {
    RegisterDataSourceResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegisterDataSourceResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RegisterDataSourceResponse>
  where
      'msg: 'shorter {
    RegisterDataSourceResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for RegisterDataSourceResponseMut<'msg> {
  type MutProxied = RegisterDataSourceResponse;
  fn as_mut(&mut self) -> RegisterDataSourceResponseMut<'msg> {
    RegisterDataSourceResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RegisterDataSourceResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> RegisterDataSourceResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RegisterDataSourceResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfig_RegisterDataSourceResponse_new() } } }
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

  pub fn as_view(&self) -> RegisterDataSourceResponseView {
    RegisterDataSourceResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> RegisterDataSourceResponseMut {
    RegisterDataSourceResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl RegisterDataSourceResponse

impl ::std::ops::Drop for RegisterDataSourceResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for RegisterDataSourceResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RegisterDataSourceResponse {
  type Proxied = Self;
  fn as_view(&self) -> RegisterDataSourceResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RegisterDataSourceResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RegisterDataSourceResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfig_RegisterDataSourceResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfig_RegisterDataSourceResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_RegisterDataSourceResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_RegisterDataSourceResponse_config_definition_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> RegisterDataSourceResponseMut<'a> {
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

impl<'a> RegisterDataSourceResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for RegisterDataSourceResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for RegisterDataSourceResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for RegisterDataSourceResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for RegisterDataSourceResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for RegisterDataSourceResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for RegisterDataSourceResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct UnregisterDataSourceRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for UnregisterDataSourceRequest {}

impl ::std::default::Default for UnregisterDataSourceRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for UnregisterDataSourceRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for UnregisterDataSourceRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for UnregisterDataSourceRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for UnregisterDataSourceRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for UnregisterDataSourceRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for UnregisterDataSourceRequest {
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
// - `UnregisterDataSourceRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `UnregisterDataSourceRequestMut`.
unsafe impl Sync for UnregisterDataSourceRequest {}

// SAFETY:
// - `UnregisterDataSourceRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UnregisterDataSourceRequest {}

impl ::protobuf::Proxied for UnregisterDataSourceRequest {
  type View<'msg> = UnregisterDataSourceRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UnregisterDataSourceRequest {}

impl ::protobuf::MutProxied for UnregisterDataSourceRequest {
  type Mut<'msg> = UnregisterDataSourceRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UnregisterDataSourceRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnregisterDataSourceRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UnregisterDataSourceRequestView<'msg> {
  type Message = UnregisterDataSourceRequest;
}

impl ::std::fmt::Debug for UnregisterDataSourceRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for UnregisterDataSourceRequestView<'_> {
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

impl ::std::default::Default for UnregisterDataSourceRequestView<'_> {
  fn default() -> UnregisterDataSourceRequestView<'static> {
    UnregisterDataSourceRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfig_UnregisterDataSourceRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> UnregisterDataSourceRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> UnregisterDataSourceRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_definition: optional string
  pub fn config_definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // data_source_code: optional string
  pub fn data_source_code(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `UnregisterDataSourceRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for UnregisterDataSourceRequestView<'_> {}

// SAFETY:
// - `UnregisterDataSourceRequestView` is `Send` because while its alive a `UnregisterDataSourceRequestMut` cannot.
// - `UnregisterDataSourceRequestView` does not use thread-local data.
unsafe impl Send for UnregisterDataSourceRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for UnregisterDataSourceRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for UnregisterDataSourceRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for UnregisterDataSourceRequestView<'msg> {
  type Proxied = UnregisterDataSourceRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, UnregisterDataSourceRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnregisterDataSourceRequestView<'msg> {
  fn into_view<'shorter>(self) -> UnregisterDataSourceRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UnregisterDataSourceRequest> for UnregisterDataSourceRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UnregisterDataSourceRequest {
    let dst = UnregisterDataSourceRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UnregisterDataSourceRequest> for UnregisterDataSourceRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UnregisterDataSourceRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for UnregisterDataSourceRequest {
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
      let prototype = <UnregisterDataSourceRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for UnregisterDataSourceRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<UnregisterDataSourceRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> UnregisterDataSourceRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { UnregisterDataSourceRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UnregisterDataSourceRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnregisterDataSourceRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UnregisterDataSourceRequestMut<'msg> {
  type Message = UnregisterDataSourceRequest;
}

impl ::std::fmt::Debug for UnregisterDataSourceRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for UnregisterDataSourceRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for UnregisterDataSourceRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for UnregisterDataSourceRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = UnregisterDataSourceRequest>) {
    // SAFETY: self and src are both valid `UnregisterDataSourceRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> UnregisterDataSourceRequestMut<'msg> {
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

  pub fn to_owned(&self) -> UnregisterDataSourceRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // data_source_code: optional string
  pub fn data_source_code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_source_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `UnregisterDataSourceRequestMut` does not perform any shared mutation.
// - `UnregisterDataSourceRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UnregisterDataSourceRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for UnregisterDataSourceRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for UnregisterDataSourceRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for UnregisterDataSourceRequestMut<'msg> {
  type Proxied = UnregisterDataSourceRequest;
  fn as_view(&self) -> ::protobuf::View<'_, UnregisterDataSourceRequest> {
    UnregisterDataSourceRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnregisterDataSourceRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UnregisterDataSourceRequest>
  where
      'msg: 'shorter {
    UnregisterDataSourceRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for UnregisterDataSourceRequestMut<'msg> {
  type MutProxied = UnregisterDataSourceRequest;
  fn as_mut(&mut self) -> UnregisterDataSourceRequestMut<'msg> {
    UnregisterDataSourceRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UnregisterDataSourceRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> UnregisterDataSourceRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UnregisterDataSourceRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfig_UnregisterDataSourceRequest_new() } } }
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

  pub fn as_view(&self) -> UnregisterDataSourceRequestView {
    UnregisterDataSourceRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> UnregisterDataSourceRequestMut {
    UnregisterDataSourceRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // data_source_code: optional string
  pub fn data_source_code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_source_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl UnregisterDataSourceRequest

impl ::std::ops::Drop for UnregisterDataSourceRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for UnregisterDataSourceRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UnregisterDataSourceRequest {
  type Proxied = Self;
  fn as_view(&self) -> UnregisterDataSourceRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UnregisterDataSourceRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UnregisterDataSourceRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfig_UnregisterDataSourceRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfig_UnregisterDataSourceRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_config_definition_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_UnregisterDataSourceRequest_data_source_code_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> UnregisterDataSourceRequestMut<'a> {
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

impl<'a> UnregisterDataSourceRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for UnregisterDataSourceRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for UnregisterDataSourceRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for UnregisterDataSourceRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for UnregisterDataSourceRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for UnregisterDataSourceRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for UnregisterDataSourceRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct UnregisterDataSourceResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for UnregisterDataSourceResponse {}

impl ::std::default::Default for UnregisterDataSourceResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for UnregisterDataSourceResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for UnregisterDataSourceResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for UnregisterDataSourceResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for UnregisterDataSourceResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for UnregisterDataSourceResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for UnregisterDataSourceResponse {
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
// - `UnregisterDataSourceResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `UnregisterDataSourceResponseMut`.
unsafe impl Sync for UnregisterDataSourceResponse {}

// SAFETY:
// - `UnregisterDataSourceResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UnregisterDataSourceResponse {}

impl ::protobuf::Proxied for UnregisterDataSourceResponse {
  type View<'msg> = UnregisterDataSourceResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UnregisterDataSourceResponse {}

impl ::protobuf::MutProxied for UnregisterDataSourceResponse {
  type Mut<'msg> = UnregisterDataSourceResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UnregisterDataSourceResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnregisterDataSourceResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UnregisterDataSourceResponseView<'msg> {
  type Message = UnregisterDataSourceResponse;
}

impl ::std::fmt::Debug for UnregisterDataSourceResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for UnregisterDataSourceResponseView<'_> {
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

impl ::std::default::Default for UnregisterDataSourceResponseView<'_> {
  fn default() -> UnregisterDataSourceResponseView<'static> {
    UnregisterDataSourceResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfig_UnregisterDataSourceResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> UnregisterDataSourceResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> UnregisterDataSourceResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // config_definition: optional string
  pub fn config_definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `UnregisterDataSourceResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for UnregisterDataSourceResponseView<'_> {}

// SAFETY:
// - `UnregisterDataSourceResponseView` is `Send` because while its alive a `UnregisterDataSourceResponseMut` cannot.
// - `UnregisterDataSourceResponseView` does not use thread-local data.
unsafe impl Send for UnregisterDataSourceResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for UnregisterDataSourceResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for UnregisterDataSourceResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for UnregisterDataSourceResponseView<'msg> {
  type Proxied = UnregisterDataSourceResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, UnregisterDataSourceResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnregisterDataSourceResponseView<'msg> {
  fn into_view<'shorter>(self) -> UnregisterDataSourceResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UnregisterDataSourceResponse> for UnregisterDataSourceResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UnregisterDataSourceResponse {
    let dst = UnregisterDataSourceResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UnregisterDataSourceResponse> for UnregisterDataSourceResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UnregisterDataSourceResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for UnregisterDataSourceResponse {
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
      let prototype = <UnregisterDataSourceResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for UnregisterDataSourceResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<UnregisterDataSourceResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> UnregisterDataSourceResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { UnregisterDataSourceResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UnregisterDataSourceResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnregisterDataSourceResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UnregisterDataSourceResponseMut<'msg> {
  type Message = UnregisterDataSourceResponse;
}

impl ::std::fmt::Debug for UnregisterDataSourceResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for UnregisterDataSourceResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for UnregisterDataSourceResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for UnregisterDataSourceResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = UnregisterDataSourceResponse>) {
    // SAFETY: self and src are both valid `UnregisterDataSourceResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> UnregisterDataSourceResponseMut<'msg> {
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

  pub fn to_owned(&self) -> UnregisterDataSourceResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `UnregisterDataSourceResponseMut` does not perform any shared mutation.
// - `UnregisterDataSourceResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UnregisterDataSourceResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for UnregisterDataSourceResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for UnregisterDataSourceResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for UnregisterDataSourceResponseMut<'msg> {
  type Proxied = UnregisterDataSourceResponse;
  fn as_view(&self) -> ::protobuf::View<'_, UnregisterDataSourceResponse> {
    UnregisterDataSourceResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnregisterDataSourceResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UnregisterDataSourceResponse>
  where
      'msg: 'shorter {
    UnregisterDataSourceResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for UnregisterDataSourceResponseMut<'msg> {
  type MutProxied = UnregisterDataSourceResponse;
  fn as_mut(&mut self) -> UnregisterDataSourceResponseMut<'msg> {
    UnregisterDataSourceResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UnregisterDataSourceResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> UnregisterDataSourceResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UnregisterDataSourceResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfig_UnregisterDataSourceResponse_new() } } }
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

  pub fn as_view(&self) -> UnregisterDataSourceResponseView {
    UnregisterDataSourceResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> UnregisterDataSourceResponseMut {
    UnregisterDataSourceResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl UnregisterDataSourceResponse

impl ::std::ops::Drop for UnregisterDataSourceResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for UnregisterDataSourceResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UnregisterDataSourceResponse {
  type Proxied = Self;
  fn as_view(&self) -> UnregisterDataSourceResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UnregisterDataSourceResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UnregisterDataSourceResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfig_UnregisterDataSourceResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfig_UnregisterDataSourceResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_UnregisterDataSourceResponse_config_definition_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> UnregisterDataSourceResponseMut<'a> {
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

impl<'a> UnregisterDataSourceResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for UnregisterDataSourceResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for UnregisterDataSourceResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for UnregisterDataSourceResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for UnregisterDataSourceResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for UnregisterDataSourceResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for UnregisterDataSourceResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct VerifyConfigRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for VerifyConfigRequest {}

impl ::std::default::Default for VerifyConfigRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for VerifyConfigRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for VerifyConfigRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for VerifyConfigRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for VerifyConfigRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for VerifyConfigRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for VerifyConfigRequest {
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
// - `VerifyConfigRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `VerifyConfigRequestMut`.
unsafe impl Sync for VerifyConfigRequest {}

// SAFETY:
// - `VerifyConfigRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for VerifyConfigRequest {}

impl ::protobuf::Proxied for VerifyConfigRequest {
  type View<'msg> = VerifyConfigRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for VerifyConfigRequest {}

impl ::protobuf::MutProxied for VerifyConfigRequest {
  type Mut<'msg> = VerifyConfigRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VerifyConfigRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VerifyConfigRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for VerifyConfigRequestView<'msg> {
  type Message = VerifyConfigRequest;
}

impl ::std::fmt::Debug for VerifyConfigRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for VerifyConfigRequestView<'_> {
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

impl ::std::default::Default for VerifyConfigRequestView<'_> {
  fn default() -> VerifyConfigRequestView<'static> {
    VerifyConfigRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfig_VerifyConfigRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> VerifyConfigRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> VerifyConfigRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_definition: optional string
  pub fn config_definition(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `VerifyConfigRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for VerifyConfigRequestView<'_> {}

// SAFETY:
// - `VerifyConfigRequestView` is `Send` because while its alive a `VerifyConfigRequestMut` cannot.
// - `VerifyConfigRequestView` does not use thread-local data.
unsafe impl Send for VerifyConfigRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for VerifyConfigRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for VerifyConfigRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for VerifyConfigRequestView<'msg> {
  type Proxied = VerifyConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, VerifyConfigRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VerifyConfigRequestView<'msg> {
  fn into_view<'shorter>(self) -> VerifyConfigRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<VerifyConfigRequest> for VerifyConfigRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VerifyConfigRequest {
    let dst = VerifyConfigRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<VerifyConfigRequest> for VerifyConfigRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VerifyConfigRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for VerifyConfigRequest {
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
      let prototype = <VerifyConfigRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for VerifyConfigRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<VerifyConfigRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> VerifyConfigRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { VerifyConfigRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VerifyConfigRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VerifyConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for VerifyConfigRequestMut<'msg> {
  type Message = VerifyConfigRequest;
}

impl ::std::fmt::Debug for VerifyConfigRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for VerifyConfigRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for VerifyConfigRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for VerifyConfigRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = VerifyConfigRequest>) {
    // SAFETY: self and src are both valid `VerifyConfigRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> VerifyConfigRequestMut<'msg> {
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

  pub fn to_owned(&self) -> VerifyConfigRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `VerifyConfigRequestMut` does not perform any shared mutation.
// - `VerifyConfigRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for VerifyConfigRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for VerifyConfigRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for VerifyConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for VerifyConfigRequestMut<'msg> {
  type Proxied = VerifyConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'_, VerifyConfigRequest> {
    VerifyConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VerifyConfigRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, VerifyConfigRequest>
  where
      'msg: 'shorter {
    VerifyConfigRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for VerifyConfigRequestMut<'msg> {
  type MutProxied = VerifyConfigRequest;
  fn as_mut(&mut self) -> VerifyConfigRequestMut<'msg> {
    VerifyConfigRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for VerifyConfigRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> VerifyConfigRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl VerifyConfigRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfig_VerifyConfigRequest_new() } } }
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

  pub fn as_view(&self) -> VerifyConfigRequestView {
    VerifyConfigRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> VerifyConfigRequestMut {
    VerifyConfigRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_definition: optional string
  pub fn config_definition(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_config_definition(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl VerifyConfigRequest

impl ::std::ops::Drop for VerifyConfigRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for VerifyConfigRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for VerifyConfigRequest {
  type Proxied = Self;
  fn as_view(&self) -> VerifyConfigRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for VerifyConfigRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VerifyConfigRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfig_VerifyConfigRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfig_VerifyConfigRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szconfig_VerifyConfigRequest_config_definition_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> VerifyConfigRequestMut<'a> {
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

impl<'a> VerifyConfigRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for VerifyConfigRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for VerifyConfigRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for VerifyConfigRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for VerifyConfigRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for VerifyConfigRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for VerifyConfigRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct VerifyConfigResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for VerifyConfigResponse {}

impl ::std::default::Default for VerifyConfigResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for VerifyConfigResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for VerifyConfigResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for VerifyConfigResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for VerifyConfigResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for VerifyConfigResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for VerifyConfigResponse {
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
// - `VerifyConfigResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `VerifyConfigResponseMut`.
unsafe impl Sync for VerifyConfigResponse {}

// SAFETY:
// - `VerifyConfigResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for VerifyConfigResponse {}

impl ::protobuf::Proxied for VerifyConfigResponse {
  type View<'msg> = VerifyConfigResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for VerifyConfigResponse {}

impl ::protobuf::MutProxied for VerifyConfigResponse {
  type Mut<'msg> = VerifyConfigResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VerifyConfigResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VerifyConfigResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for VerifyConfigResponseView<'msg> {
  type Message = VerifyConfigResponse;
}

impl ::std::fmt::Debug for VerifyConfigResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for VerifyConfigResponseView<'_> {
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

impl ::std::default::Default for VerifyConfigResponseView<'_> {
  fn default() -> VerifyConfigResponseView<'static> {
    VerifyConfigResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szconfig_VerifyConfigResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> VerifyConfigResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> VerifyConfigResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional bool
  pub fn result(self) -> bool {
    unsafe { proto2_rust_thunk_szconfig_VerifyConfigResponse_result_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `VerifyConfigResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for VerifyConfigResponseView<'_> {}

// SAFETY:
// - `VerifyConfigResponseView` is `Send` because while its alive a `VerifyConfigResponseMut` cannot.
// - `VerifyConfigResponseView` does not use thread-local data.
unsafe impl Send for VerifyConfigResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for VerifyConfigResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for VerifyConfigResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for VerifyConfigResponseView<'msg> {
  type Proxied = VerifyConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, VerifyConfigResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VerifyConfigResponseView<'msg> {
  fn into_view<'shorter>(self) -> VerifyConfigResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<VerifyConfigResponse> for VerifyConfigResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VerifyConfigResponse {
    let dst = VerifyConfigResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<VerifyConfigResponse> for VerifyConfigResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VerifyConfigResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for VerifyConfigResponse {
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
      let prototype = <VerifyConfigResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for VerifyConfigResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<VerifyConfigResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> VerifyConfigResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { VerifyConfigResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VerifyConfigResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VerifyConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for VerifyConfigResponseMut<'msg> {
  type Message = VerifyConfigResponse;
}

impl ::std::fmt::Debug for VerifyConfigResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for VerifyConfigResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for VerifyConfigResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for VerifyConfigResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = VerifyConfigResponse>) {
    // SAFETY: self and src are both valid `VerifyConfigResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> VerifyConfigResponseMut<'msg> {
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

  pub fn to_owned(&self) -> VerifyConfigResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional bool
  pub fn result(&self) -> bool {
    unsafe { proto2_rust_thunk_szconfig_VerifyConfigResponse_result_get(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: bool) {
    unsafe { proto2_rust_thunk_szconfig_VerifyConfigResponse_result_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `VerifyConfigResponseMut` does not perform any shared mutation.
// - `VerifyConfigResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for VerifyConfigResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for VerifyConfigResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for VerifyConfigResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for VerifyConfigResponseMut<'msg> {
  type Proxied = VerifyConfigResponse;
  fn as_view(&self) -> ::protobuf::View<'_, VerifyConfigResponse> {
    VerifyConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VerifyConfigResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, VerifyConfigResponse>
  where
      'msg: 'shorter {
    VerifyConfigResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for VerifyConfigResponseMut<'msg> {
  type MutProxied = VerifyConfigResponse;
  fn as_mut(&mut self) -> VerifyConfigResponseMut<'msg> {
    VerifyConfigResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for VerifyConfigResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> VerifyConfigResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl VerifyConfigResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szconfig_VerifyConfigResponse_new() } } }
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

  pub fn as_view(&self) -> VerifyConfigResponseView {
    VerifyConfigResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> VerifyConfigResponseMut {
    VerifyConfigResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional bool
  pub fn result(&self) -> bool {
    unsafe { proto2_rust_thunk_szconfig_VerifyConfigResponse_result_get(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: bool) {
    unsafe { proto2_rust_thunk_szconfig_VerifyConfigResponse_result_set(self.raw_msg(), val) }
  }

}  // impl VerifyConfigResponse

impl ::std::ops::Drop for VerifyConfigResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for VerifyConfigResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for VerifyConfigResponse {
  type Proxied = Self;
  fn as_view(&self) -> VerifyConfigResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for VerifyConfigResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VerifyConfigResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szconfig_VerifyConfigResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szconfig_VerifyConfigResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szconfig_VerifyConfigResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_szconfig_VerifyConfigResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: bool);

}

impl<'a> VerifyConfigResponseMut<'a> {
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

impl<'a> VerifyConfigResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for VerifyConfigResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for VerifyConfigResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for VerifyConfigResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for VerifyConfigResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for VerifyConfigResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for VerifyConfigResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

