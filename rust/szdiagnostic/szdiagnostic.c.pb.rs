const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.30.2-beta2");
#[allow(non_camel_case_types)]
pub struct CheckRepositoryPerformanceRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for CheckRepositoryPerformanceRequest {}

impl ::std::default::Default for CheckRepositoryPerformanceRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for CheckRepositoryPerformanceRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for CheckRepositoryPerformanceRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for CheckRepositoryPerformanceRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for CheckRepositoryPerformanceRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for CheckRepositoryPerformanceRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for CheckRepositoryPerformanceRequest {
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
// - `CheckRepositoryPerformanceRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `CheckRepositoryPerformanceRequestMut`.
unsafe impl Sync for CheckRepositoryPerformanceRequest {}

// SAFETY:
// - `CheckRepositoryPerformanceRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CheckRepositoryPerformanceRequest {}

impl ::protobuf::Proxied for CheckRepositoryPerformanceRequest {
  type View<'msg> = CheckRepositoryPerformanceRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CheckRepositoryPerformanceRequest {}

impl ::protobuf::MutProxied for CheckRepositoryPerformanceRequest {
  type Mut<'msg> = CheckRepositoryPerformanceRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CheckRepositoryPerformanceRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckRepositoryPerformanceRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CheckRepositoryPerformanceRequestView<'msg> {
  type Message = CheckRepositoryPerformanceRequest;
}

impl ::std::fmt::Debug for CheckRepositoryPerformanceRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CheckRepositoryPerformanceRequestView<'_> {
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

impl ::std::default::Default for CheckRepositoryPerformanceRequestView<'_> {
  fn default() -> CheckRepositoryPerformanceRequestView<'static> {
    CheckRepositoryPerformanceRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> CheckRepositoryPerformanceRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> CheckRepositoryPerformanceRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // seconds_to_run: optional int32
  pub fn seconds_to_run(self) -> i32 {
    unsafe { proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `CheckRepositoryPerformanceRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for CheckRepositoryPerformanceRequestView<'_> {}

// SAFETY:
// - `CheckRepositoryPerformanceRequestView` is `Send` because while its alive a `CheckRepositoryPerformanceRequestMut` cannot.
// - `CheckRepositoryPerformanceRequestView` does not use thread-local data.
unsafe impl Send for CheckRepositoryPerformanceRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for CheckRepositoryPerformanceRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for CheckRepositoryPerformanceRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for CheckRepositoryPerformanceRequestView<'msg> {
  type Proxied = CheckRepositoryPerformanceRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, CheckRepositoryPerformanceRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckRepositoryPerformanceRequestView<'msg> {
  fn into_view<'shorter>(self) -> CheckRepositoryPerformanceRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckRepositoryPerformanceRequest> for CheckRepositoryPerformanceRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckRepositoryPerformanceRequest {
    let dst = CheckRepositoryPerformanceRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckRepositoryPerformanceRequest> for CheckRepositoryPerformanceRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckRepositoryPerformanceRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for CheckRepositoryPerformanceRequest {
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
      let prototype = <CheckRepositoryPerformanceRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for CheckRepositoryPerformanceRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<CheckRepositoryPerformanceRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> CheckRepositoryPerformanceRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { CheckRepositoryPerformanceRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CheckRepositoryPerformanceRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckRepositoryPerformanceRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CheckRepositoryPerformanceRequestMut<'msg> {
  type Message = CheckRepositoryPerformanceRequest;
}

impl ::std::fmt::Debug for CheckRepositoryPerformanceRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CheckRepositoryPerformanceRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for CheckRepositoryPerformanceRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for CheckRepositoryPerformanceRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = CheckRepositoryPerformanceRequest>) {
    // SAFETY: self and src are both valid `CheckRepositoryPerformanceRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> CheckRepositoryPerformanceRequestMut<'msg> {
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

  pub fn to_owned(&self) -> CheckRepositoryPerformanceRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // seconds_to_run: optional int32
  pub fn seconds_to_run(&self) -> i32 {
    unsafe { proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_get(self.raw_msg()) }
  }
  pub fn set_seconds_to_run(&mut self, val: i32) {
    unsafe { proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `CheckRepositoryPerformanceRequestMut` does not perform any shared mutation.
// - `CheckRepositoryPerformanceRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CheckRepositoryPerformanceRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for CheckRepositoryPerformanceRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for CheckRepositoryPerformanceRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for CheckRepositoryPerformanceRequestMut<'msg> {
  type Proxied = CheckRepositoryPerformanceRequest;
  fn as_view(&self) -> ::protobuf::View<'_, CheckRepositoryPerformanceRequest> {
    CheckRepositoryPerformanceRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckRepositoryPerformanceRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CheckRepositoryPerformanceRequest>
  where
      'msg: 'shorter {
    CheckRepositoryPerformanceRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for CheckRepositoryPerformanceRequestMut<'msg> {
  type MutProxied = CheckRepositoryPerformanceRequest;
  fn as_mut(&mut self) -> CheckRepositoryPerformanceRequestMut<'msg> {
    CheckRepositoryPerformanceRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CheckRepositoryPerformanceRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> CheckRepositoryPerformanceRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CheckRepositoryPerformanceRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceRequest_new() } } }
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

  pub fn as_view(&self) -> CheckRepositoryPerformanceRequestView {
    CheckRepositoryPerformanceRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CheckRepositoryPerformanceRequestMut {
    CheckRepositoryPerformanceRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // seconds_to_run: optional int32
  pub fn seconds_to_run(&self) -> i32 {
    unsafe { proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_get(self.raw_msg()) }
  }
  pub fn set_seconds_to_run(&mut self, val: i32) {
    unsafe { proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_set(self.raw_msg(), val) }
  }

}  // impl CheckRepositoryPerformanceRequest

impl ::std::ops::Drop for CheckRepositoryPerformanceRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for CheckRepositoryPerformanceRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CheckRepositoryPerformanceRequest {
  type Proxied = Self;
  fn as_view(&self) -> CheckRepositoryPerformanceRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CheckRepositoryPerformanceRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CheckRepositoryPerformanceRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i32;
  fn proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceRequest_seconds_to_run_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i32);

}

impl<'a> CheckRepositoryPerformanceRequestMut<'a> {
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

impl<'a> CheckRepositoryPerformanceRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for CheckRepositoryPerformanceRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for CheckRepositoryPerformanceRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for CheckRepositoryPerformanceRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for CheckRepositoryPerformanceRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for CheckRepositoryPerformanceRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for CheckRepositoryPerformanceRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct CheckRepositoryPerformanceResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for CheckRepositoryPerformanceResponse {}

impl ::std::default::Default for CheckRepositoryPerformanceResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for CheckRepositoryPerformanceResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for CheckRepositoryPerformanceResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for CheckRepositoryPerformanceResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for CheckRepositoryPerformanceResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for CheckRepositoryPerformanceResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for CheckRepositoryPerformanceResponse {
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
// - `CheckRepositoryPerformanceResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `CheckRepositoryPerformanceResponseMut`.
unsafe impl Sync for CheckRepositoryPerformanceResponse {}

// SAFETY:
// - `CheckRepositoryPerformanceResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CheckRepositoryPerformanceResponse {}

impl ::protobuf::Proxied for CheckRepositoryPerformanceResponse {
  type View<'msg> = CheckRepositoryPerformanceResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CheckRepositoryPerformanceResponse {}

impl ::protobuf::MutProxied for CheckRepositoryPerformanceResponse {
  type Mut<'msg> = CheckRepositoryPerformanceResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CheckRepositoryPerformanceResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckRepositoryPerformanceResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CheckRepositoryPerformanceResponseView<'msg> {
  type Message = CheckRepositoryPerformanceResponse;
}

impl ::std::fmt::Debug for CheckRepositoryPerformanceResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CheckRepositoryPerformanceResponseView<'_> {
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

impl ::std::default::Default for CheckRepositoryPerformanceResponseView<'_> {
  fn default() -> CheckRepositoryPerformanceResponseView<'static> {
    CheckRepositoryPerformanceResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> CheckRepositoryPerformanceResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> CheckRepositoryPerformanceResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `CheckRepositoryPerformanceResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for CheckRepositoryPerformanceResponseView<'_> {}

// SAFETY:
// - `CheckRepositoryPerformanceResponseView` is `Send` because while its alive a `CheckRepositoryPerformanceResponseMut` cannot.
// - `CheckRepositoryPerformanceResponseView` does not use thread-local data.
unsafe impl Send for CheckRepositoryPerformanceResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for CheckRepositoryPerformanceResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for CheckRepositoryPerformanceResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for CheckRepositoryPerformanceResponseView<'msg> {
  type Proxied = CheckRepositoryPerformanceResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, CheckRepositoryPerformanceResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckRepositoryPerformanceResponseView<'msg> {
  fn into_view<'shorter>(self) -> CheckRepositoryPerformanceResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckRepositoryPerformanceResponse> for CheckRepositoryPerformanceResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckRepositoryPerformanceResponse {
    let dst = CheckRepositoryPerformanceResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckRepositoryPerformanceResponse> for CheckRepositoryPerformanceResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckRepositoryPerformanceResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for CheckRepositoryPerformanceResponse {
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
      let prototype = <CheckRepositoryPerformanceResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for CheckRepositoryPerformanceResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<CheckRepositoryPerformanceResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> CheckRepositoryPerformanceResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { CheckRepositoryPerformanceResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CheckRepositoryPerformanceResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckRepositoryPerformanceResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CheckRepositoryPerformanceResponseMut<'msg> {
  type Message = CheckRepositoryPerformanceResponse;
}

impl ::std::fmt::Debug for CheckRepositoryPerformanceResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CheckRepositoryPerformanceResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for CheckRepositoryPerformanceResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for CheckRepositoryPerformanceResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = CheckRepositoryPerformanceResponse>) {
    // SAFETY: self and src are both valid `CheckRepositoryPerformanceResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> CheckRepositoryPerformanceResponseMut<'msg> {
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

  pub fn to_owned(&self) -> CheckRepositoryPerformanceResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `CheckRepositoryPerformanceResponseMut` does not perform any shared mutation.
// - `CheckRepositoryPerformanceResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CheckRepositoryPerformanceResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for CheckRepositoryPerformanceResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for CheckRepositoryPerformanceResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for CheckRepositoryPerformanceResponseMut<'msg> {
  type Proxied = CheckRepositoryPerformanceResponse;
  fn as_view(&self) -> ::protobuf::View<'_, CheckRepositoryPerformanceResponse> {
    CheckRepositoryPerformanceResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckRepositoryPerformanceResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CheckRepositoryPerformanceResponse>
  where
      'msg: 'shorter {
    CheckRepositoryPerformanceResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for CheckRepositoryPerformanceResponseMut<'msg> {
  type MutProxied = CheckRepositoryPerformanceResponse;
  fn as_mut(&mut self) -> CheckRepositoryPerformanceResponseMut<'msg> {
    CheckRepositoryPerformanceResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CheckRepositoryPerformanceResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> CheckRepositoryPerformanceResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CheckRepositoryPerformanceResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceResponse_new() } } }
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

  pub fn as_view(&self) -> CheckRepositoryPerformanceResponseView {
    CheckRepositoryPerformanceResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CheckRepositoryPerformanceResponseMut {
    CheckRepositoryPerformanceResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl CheckRepositoryPerformanceResponse

impl ::std::ops::Drop for CheckRepositoryPerformanceResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for CheckRepositoryPerformanceResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CheckRepositoryPerformanceResponse {
  type Proxied = Self;
  fn as_view(&self) -> CheckRepositoryPerformanceResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CheckRepositoryPerformanceResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CheckRepositoryPerformanceResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_CheckRepositoryPerformanceResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szdiagnostic_CheckRepositoryPerformanceResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> CheckRepositoryPerformanceResponseMut<'a> {
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

impl<'a> CheckRepositoryPerformanceResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for CheckRepositoryPerformanceResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for CheckRepositoryPerformanceResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for CheckRepositoryPerformanceResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for CheckRepositoryPerformanceResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for CheckRepositoryPerformanceResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for CheckRepositoryPerformanceResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetRepositoryInfoRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetRepositoryInfoRequest {}

impl ::std::default::Default for GetRepositoryInfoRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetRepositoryInfoRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetRepositoryInfoRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetRepositoryInfoRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetRepositoryInfoRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetRepositoryInfoRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetRepositoryInfoRequest {
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
// - `GetRepositoryInfoRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetRepositoryInfoRequestMut`.
unsafe impl Sync for GetRepositoryInfoRequest {}

// SAFETY:
// - `GetRepositoryInfoRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetRepositoryInfoRequest {}

impl ::protobuf::Proxied for GetRepositoryInfoRequest {
  type View<'msg> = GetRepositoryInfoRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetRepositoryInfoRequest {}

impl ::protobuf::MutProxied for GetRepositoryInfoRequest {
  type Mut<'msg> = GetRepositoryInfoRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetRepositoryInfoRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetRepositoryInfoRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetRepositoryInfoRequestView<'msg> {
  type Message = GetRepositoryInfoRequest;
}

impl ::std::fmt::Debug for GetRepositoryInfoRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetRepositoryInfoRequestView<'_> {
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

impl ::std::default::Default for GetRepositoryInfoRequestView<'_> {
  fn default() -> GetRepositoryInfoRequestView<'static> {
    GetRepositoryInfoRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetRepositoryInfoRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetRepositoryInfoRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GetRepositoryInfoRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetRepositoryInfoRequestView<'_> {}

// SAFETY:
// - `GetRepositoryInfoRequestView` is `Send` because while its alive a `GetRepositoryInfoRequestMut` cannot.
// - `GetRepositoryInfoRequestView` does not use thread-local data.
unsafe impl Send for GetRepositoryInfoRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetRepositoryInfoRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetRepositoryInfoRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetRepositoryInfoRequestView<'msg> {
  type Proxied = GetRepositoryInfoRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetRepositoryInfoRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetRepositoryInfoRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetRepositoryInfoRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetRepositoryInfoRequest> for GetRepositoryInfoRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetRepositoryInfoRequest {
    let dst = GetRepositoryInfoRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetRepositoryInfoRequest> for GetRepositoryInfoRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetRepositoryInfoRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetRepositoryInfoRequest {
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
      let prototype = <GetRepositoryInfoRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetRepositoryInfoRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetRepositoryInfoRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetRepositoryInfoRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetRepositoryInfoRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetRepositoryInfoRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetRepositoryInfoRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetRepositoryInfoRequestMut<'msg> {
  type Message = GetRepositoryInfoRequest;
}

impl ::std::fmt::Debug for GetRepositoryInfoRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetRepositoryInfoRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetRepositoryInfoRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetRepositoryInfoRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetRepositoryInfoRequest>) {
    // SAFETY: self and src are both valid `GetRepositoryInfoRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetRepositoryInfoRequestMut<'msg> {
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

  pub fn to_owned(&self) -> GetRepositoryInfoRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `GetRepositoryInfoRequestMut` does not perform any shared mutation.
// - `GetRepositoryInfoRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetRepositoryInfoRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetRepositoryInfoRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetRepositoryInfoRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetRepositoryInfoRequestMut<'msg> {
  type Proxied = GetRepositoryInfoRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetRepositoryInfoRequest> {
    GetRepositoryInfoRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetRepositoryInfoRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetRepositoryInfoRequest>
  where
      'msg: 'shorter {
    GetRepositoryInfoRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetRepositoryInfoRequestMut<'msg> {
  type MutProxied = GetRepositoryInfoRequest;
  fn as_mut(&mut self) -> GetRepositoryInfoRequestMut<'msg> {
    GetRepositoryInfoRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetRepositoryInfoRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetRepositoryInfoRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetRepositoryInfoRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoRequest_new() } } }
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

  pub fn as_view(&self) -> GetRepositoryInfoRequestView {
    GetRepositoryInfoRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetRepositoryInfoRequestMut {
    GetRepositoryInfoRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl GetRepositoryInfoRequest

impl ::std::ops::Drop for GetRepositoryInfoRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetRepositoryInfoRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetRepositoryInfoRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetRepositoryInfoRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetRepositoryInfoRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetRepositoryInfoRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> GetRepositoryInfoRequestMut<'a> {
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

impl<'a> GetRepositoryInfoRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetRepositoryInfoRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetRepositoryInfoRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetRepositoryInfoRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetRepositoryInfoRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetRepositoryInfoRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetRepositoryInfoRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetRepositoryInfoResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetRepositoryInfoResponse {}

impl ::std::default::Default for GetRepositoryInfoResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetRepositoryInfoResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetRepositoryInfoResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetRepositoryInfoResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetRepositoryInfoResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetRepositoryInfoResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetRepositoryInfoResponse {
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
// - `GetRepositoryInfoResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetRepositoryInfoResponseMut`.
unsafe impl Sync for GetRepositoryInfoResponse {}

// SAFETY:
// - `GetRepositoryInfoResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetRepositoryInfoResponse {}

impl ::protobuf::Proxied for GetRepositoryInfoResponse {
  type View<'msg> = GetRepositoryInfoResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetRepositoryInfoResponse {}

impl ::protobuf::MutProxied for GetRepositoryInfoResponse {
  type Mut<'msg> = GetRepositoryInfoResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetRepositoryInfoResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetRepositoryInfoResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetRepositoryInfoResponseView<'msg> {
  type Message = GetRepositoryInfoResponse;
}

impl ::std::fmt::Debug for GetRepositoryInfoResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetRepositoryInfoResponseView<'_> {
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

impl ::std::default::Default for GetRepositoryInfoResponseView<'_> {
  fn default() -> GetRepositoryInfoResponseView<'static> {
    GetRepositoryInfoResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetRepositoryInfoResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetRepositoryInfoResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetRepositoryInfoResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetRepositoryInfoResponseView<'_> {}

// SAFETY:
// - `GetRepositoryInfoResponseView` is `Send` because while its alive a `GetRepositoryInfoResponseMut` cannot.
// - `GetRepositoryInfoResponseView` does not use thread-local data.
unsafe impl Send for GetRepositoryInfoResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetRepositoryInfoResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetRepositoryInfoResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetRepositoryInfoResponseView<'msg> {
  type Proxied = GetRepositoryInfoResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetRepositoryInfoResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetRepositoryInfoResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetRepositoryInfoResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetRepositoryInfoResponse> for GetRepositoryInfoResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetRepositoryInfoResponse {
    let dst = GetRepositoryInfoResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetRepositoryInfoResponse> for GetRepositoryInfoResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetRepositoryInfoResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetRepositoryInfoResponse {
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
      let prototype = <GetRepositoryInfoResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetRepositoryInfoResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetRepositoryInfoResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetRepositoryInfoResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetRepositoryInfoResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetRepositoryInfoResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetRepositoryInfoResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetRepositoryInfoResponseMut<'msg> {
  type Message = GetRepositoryInfoResponse;
}

impl ::std::fmt::Debug for GetRepositoryInfoResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetRepositoryInfoResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetRepositoryInfoResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetRepositoryInfoResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetRepositoryInfoResponse>) {
    // SAFETY: self and src are both valid `GetRepositoryInfoResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetRepositoryInfoResponseMut<'msg> {
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

  pub fn to_owned(&self) -> GetRepositoryInfoResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetRepositoryInfoResponseMut` does not perform any shared mutation.
// - `GetRepositoryInfoResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetRepositoryInfoResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetRepositoryInfoResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetRepositoryInfoResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetRepositoryInfoResponseMut<'msg> {
  type Proxied = GetRepositoryInfoResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetRepositoryInfoResponse> {
    GetRepositoryInfoResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetRepositoryInfoResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetRepositoryInfoResponse>
  where
      'msg: 'shorter {
    GetRepositoryInfoResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetRepositoryInfoResponseMut<'msg> {
  type MutProxied = GetRepositoryInfoResponse;
  fn as_mut(&mut self) -> GetRepositoryInfoResponseMut<'msg> {
    GetRepositoryInfoResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetRepositoryInfoResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetRepositoryInfoResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetRepositoryInfoResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoResponse_new() } } }
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

  pub fn as_view(&self) -> GetRepositoryInfoResponseView {
    GetRepositoryInfoResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetRepositoryInfoResponseMut {
    GetRepositoryInfoResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetRepositoryInfoResponse

impl ::std::ops::Drop for GetRepositoryInfoResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetRepositoryInfoResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetRepositoryInfoResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetRepositoryInfoResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetRepositoryInfoResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetRepositoryInfoResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_GetRepositoryInfoResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szdiagnostic_GetRepositoryInfoResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetRepositoryInfoResponseMut<'a> {
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

impl<'a> GetRepositoryInfoResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetRepositoryInfoResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetRepositoryInfoResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetRepositoryInfoResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetRepositoryInfoResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetRepositoryInfoResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetRepositoryInfoResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetFeatureRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetFeatureRequest {}

impl ::std::default::Default for GetFeatureRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetFeatureRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetFeatureRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetFeatureRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetFeatureRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetFeatureRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetFeatureRequest {
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
// - `GetFeatureRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetFeatureRequestMut`.
unsafe impl Sync for GetFeatureRequest {}

// SAFETY:
// - `GetFeatureRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetFeatureRequest {}

impl ::protobuf::Proxied for GetFeatureRequest {
  type View<'msg> = GetFeatureRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetFeatureRequest {}

impl ::protobuf::MutProxied for GetFeatureRequest {
  type Mut<'msg> = GetFeatureRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetFeatureRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetFeatureRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetFeatureRequestView<'msg> {
  type Message = GetFeatureRequest;
}

impl ::std::fmt::Debug for GetFeatureRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetFeatureRequestView<'_> {
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

impl ::std::default::Default for GetFeatureRequestView<'_> {
  fn default() -> GetFeatureRequestView<'static> {
    GetFeatureRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_GetFeatureRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetFeatureRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetFeatureRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // feature_id: optional int64
  pub fn feature_id(self) -> i64 {
    unsafe { proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `GetFeatureRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetFeatureRequestView<'_> {}

// SAFETY:
// - `GetFeatureRequestView` is `Send` because while its alive a `GetFeatureRequestMut` cannot.
// - `GetFeatureRequestView` does not use thread-local data.
unsafe impl Send for GetFeatureRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetFeatureRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetFeatureRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for GetFeatureRequestView<'msg> {
  type Proxied = GetFeatureRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetFeatureRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetFeatureRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetFeatureRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetFeatureRequest> for GetFeatureRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetFeatureRequest {
    let dst = GetFeatureRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetFeatureRequest> for GetFeatureRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetFeatureRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetFeatureRequest {
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
      let prototype = <GetFeatureRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetFeatureRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetFeatureRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetFeatureRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetFeatureRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetFeatureRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetFeatureRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetFeatureRequestMut<'msg> {
  type Message = GetFeatureRequest;
}

impl ::std::fmt::Debug for GetFeatureRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetFeatureRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetFeatureRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetFeatureRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetFeatureRequest>) {
    // SAFETY: self and src are both valid `GetFeatureRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetFeatureRequestMut<'msg> {
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

  pub fn to_owned(&self) -> GetFeatureRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // feature_id: optional int64
  pub fn feature_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_get(self.raw_msg()) }
  }
  pub fn set_feature_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `GetFeatureRequestMut` does not perform any shared mutation.
// - `GetFeatureRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetFeatureRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetFeatureRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetFeatureRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetFeatureRequestMut<'msg> {
  type Proxied = GetFeatureRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetFeatureRequest> {
    GetFeatureRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetFeatureRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetFeatureRequest>
  where
      'msg: 'shorter {
    GetFeatureRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetFeatureRequestMut<'msg> {
  type MutProxied = GetFeatureRequest;
  fn as_mut(&mut self) -> GetFeatureRequestMut<'msg> {
    GetFeatureRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetFeatureRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetFeatureRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetFeatureRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_GetFeatureRequest_new() } } }
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

  pub fn as_view(&self) -> GetFeatureRequestView {
    GetFeatureRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetFeatureRequestMut {
    GetFeatureRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // feature_id: optional int64
  pub fn feature_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_get(self.raw_msg()) }
  }
  pub fn set_feature_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_set(self.raw_msg(), val) }
  }

}  // impl GetFeatureRequest

impl ::std::ops::Drop for GetFeatureRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetFeatureRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetFeatureRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetFeatureRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetFeatureRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetFeatureRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_GetFeatureRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_GetFeatureRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szdiagnostic_GetFeatureRequest_feature_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

}

impl<'a> GetFeatureRequestMut<'a> {
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

impl<'a> GetFeatureRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetFeatureRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetFeatureRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetFeatureRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetFeatureRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetFeatureRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetFeatureRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct GetFeatureResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for GetFeatureResponse {}

impl ::std::default::Default for GetFeatureResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for GetFeatureResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for GetFeatureResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for GetFeatureResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for GetFeatureResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetFeatureResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for GetFeatureResponse {
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
// - `GetFeatureResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetFeatureResponseMut`.
unsafe impl Sync for GetFeatureResponse {}

// SAFETY:
// - `GetFeatureResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetFeatureResponse {}

impl ::protobuf::Proxied for GetFeatureResponse {
  type View<'msg> = GetFeatureResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetFeatureResponse {}

impl ::protobuf::MutProxied for GetFeatureResponse {
  type Mut<'msg> = GetFeatureResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetFeatureResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetFeatureResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetFeatureResponseView<'msg> {
  type Message = GetFeatureResponse;
}

impl ::std::fmt::Debug for GetFeatureResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetFeatureResponseView<'_> {
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

impl ::std::default::Default for GetFeatureResponseView<'_> {
  fn default() -> GetFeatureResponseView<'static> {
    GetFeatureResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_GetFeatureResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> GetFeatureResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> GetFeatureResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // result: optional string
  pub fn result(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetFeatureResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetFeatureResponseView<'_> {}

// SAFETY:
// - `GetFeatureResponseView` is `Send` because while its alive a `GetFeatureResponseMut` cannot.
// - `GetFeatureResponseView` does not use thread-local data.
unsafe impl Send for GetFeatureResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetFeatureResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for GetFeatureResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for GetFeatureResponseView<'msg> {
  type Proxied = GetFeatureResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, GetFeatureResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetFeatureResponseView<'msg> {
  fn into_view<'shorter>(self) -> GetFeatureResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetFeatureResponse> for GetFeatureResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetFeatureResponse {
    let dst = GetFeatureResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetFeatureResponse> for GetFeatureResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetFeatureResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for GetFeatureResponse {
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
      let prototype = <GetFeatureResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for GetFeatureResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<GetFeatureResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> GetFeatureResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { GetFeatureResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetFeatureResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetFeatureResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetFeatureResponseMut<'msg> {
  type Message = GetFeatureResponse;
}

impl ::std::fmt::Debug for GetFeatureResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for GetFeatureResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for GetFeatureResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for GetFeatureResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = GetFeatureResponse>) {
    // SAFETY: self and src are both valid `GetFeatureResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> GetFeatureResponseMut<'msg> {
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

  pub fn to_owned(&self) -> GetFeatureResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `GetFeatureResponseMut` does not perform any shared mutation.
// - `GetFeatureResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetFeatureResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for GetFeatureResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for GetFeatureResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for GetFeatureResponseMut<'msg> {
  type Proxied = GetFeatureResponse;
  fn as_view(&self) -> ::protobuf::View<'_, GetFeatureResponse> {
    GetFeatureResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetFeatureResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetFeatureResponse>
  where
      'msg: 'shorter {
    GetFeatureResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for GetFeatureResponseMut<'msg> {
  type MutProxied = GetFeatureResponse;
  fn as_mut(&mut self) -> GetFeatureResponseMut<'msg> {
    GetFeatureResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetFeatureResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> GetFeatureResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetFeatureResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_GetFeatureResponse_new() } } }
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

  pub fn as_view(&self) -> GetFeatureResponseView {
    GetFeatureResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetFeatureResponseMut {
    GetFeatureResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // result: optional string
  pub fn result(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_result(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_set(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl GetFeatureResponse

impl ::std::ops::Drop for GetFeatureResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for GetFeatureResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetFeatureResponse {
  type Proxied = Self;
  fn as_view(&self) -> GetFeatureResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetFeatureResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetFeatureResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_GetFeatureResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_GetFeatureResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_szdiagnostic_GetFeatureResponse_result_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> GetFeatureResponseMut<'a> {
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

impl<'a> GetFeatureResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for GetFeatureResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for GetFeatureResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for GetFeatureResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for GetFeatureResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetFeatureResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for GetFeatureResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct PurgeRepositoryRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for PurgeRepositoryRequest {}

impl ::std::default::Default for PurgeRepositoryRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PurgeRepositoryRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for PurgeRepositoryRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for PurgeRepositoryRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for PurgeRepositoryRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for PurgeRepositoryRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for PurgeRepositoryRequest {
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
// - `PurgeRepositoryRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PurgeRepositoryRequestMut`.
unsafe impl Sync for PurgeRepositoryRequest {}

// SAFETY:
// - `PurgeRepositoryRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PurgeRepositoryRequest {}

impl ::protobuf::Proxied for PurgeRepositoryRequest {
  type View<'msg> = PurgeRepositoryRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PurgeRepositoryRequest {}

impl ::protobuf::MutProxied for PurgeRepositoryRequest {
  type Mut<'msg> = PurgeRepositoryRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PurgeRepositoryRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PurgeRepositoryRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PurgeRepositoryRequestView<'msg> {
  type Message = PurgeRepositoryRequest;
}

impl ::std::fmt::Debug for PurgeRepositoryRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PurgeRepositoryRequestView<'_> {
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

impl ::std::default::Default for PurgeRepositoryRequestView<'_> {
  fn default() -> PurgeRepositoryRequestView<'static> {
    PurgeRepositoryRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> PurgeRepositoryRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> PurgeRepositoryRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `PurgeRepositoryRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PurgeRepositoryRequestView<'_> {}

// SAFETY:
// - `PurgeRepositoryRequestView` is `Send` because while its alive a `PurgeRepositoryRequestMut` cannot.
// - `PurgeRepositoryRequestView` does not use thread-local data.
unsafe impl Send for PurgeRepositoryRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PurgeRepositoryRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PurgeRepositoryRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for PurgeRepositoryRequestView<'msg> {
  type Proxied = PurgeRepositoryRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PurgeRepositoryRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PurgeRepositoryRequestView<'msg> {
  fn into_view<'shorter>(self) -> PurgeRepositoryRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PurgeRepositoryRequest> for PurgeRepositoryRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PurgeRepositoryRequest {
    let dst = PurgeRepositoryRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PurgeRepositoryRequest> for PurgeRepositoryRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PurgeRepositoryRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for PurgeRepositoryRequest {
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
      let prototype = <PurgeRepositoryRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for PurgeRepositoryRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<PurgeRepositoryRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PurgeRepositoryRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { PurgeRepositoryRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PurgeRepositoryRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PurgeRepositoryRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PurgeRepositoryRequestMut<'msg> {
  type Message = PurgeRepositoryRequest;
}

impl ::std::fmt::Debug for PurgeRepositoryRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PurgeRepositoryRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for PurgeRepositoryRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for PurgeRepositoryRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = PurgeRepositoryRequest>) {
    // SAFETY: self and src are both valid `PurgeRepositoryRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> PurgeRepositoryRequestMut<'msg> {
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

  pub fn to_owned(&self) -> PurgeRepositoryRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `PurgeRepositoryRequestMut` does not perform any shared mutation.
// - `PurgeRepositoryRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PurgeRepositoryRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PurgeRepositoryRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PurgeRepositoryRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for PurgeRepositoryRequestMut<'msg> {
  type Proxied = PurgeRepositoryRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PurgeRepositoryRequest> {
    PurgeRepositoryRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PurgeRepositoryRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PurgeRepositoryRequest>
  where
      'msg: 'shorter {
    PurgeRepositoryRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for PurgeRepositoryRequestMut<'msg> {
  type MutProxied = PurgeRepositoryRequest;
  fn as_mut(&mut self) -> PurgeRepositoryRequestMut<'msg> {
    PurgeRepositoryRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PurgeRepositoryRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PurgeRepositoryRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PurgeRepositoryRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryRequest_new() } } }
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

  pub fn as_view(&self) -> PurgeRepositoryRequestView {
    PurgeRepositoryRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> PurgeRepositoryRequestMut {
    PurgeRepositoryRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl PurgeRepositoryRequest

impl ::std::ops::Drop for PurgeRepositoryRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for PurgeRepositoryRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PurgeRepositoryRequest {
  type Proxied = Self;
  fn as_view(&self) -> PurgeRepositoryRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PurgeRepositoryRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PurgeRepositoryRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> PurgeRepositoryRequestMut<'a> {
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

impl<'a> PurgeRepositoryRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for PurgeRepositoryRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for PurgeRepositoryRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for PurgeRepositoryRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for PurgeRepositoryRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for PurgeRepositoryRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for PurgeRepositoryRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct PurgeRepositoryResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for PurgeRepositoryResponse {}

impl ::std::default::Default for PurgeRepositoryResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PurgeRepositoryResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for PurgeRepositoryResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for PurgeRepositoryResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for PurgeRepositoryResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for PurgeRepositoryResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for PurgeRepositoryResponse {
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
// - `PurgeRepositoryResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PurgeRepositoryResponseMut`.
unsafe impl Sync for PurgeRepositoryResponse {}

// SAFETY:
// - `PurgeRepositoryResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PurgeRepositoryResponse {}

impl ::protobuf::Proxied for PurgeRepositoryResponse {
  type View<'msg> = PurgeRepositoryResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PurgeRepositoryResponse {}

impl ::protobuf::MutProxied for PurgeRepositoryResponse {
  type Mut<'msg> = PurgeRepositoryResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PurgeRepositoryResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PurgeRepositoryResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PurgeRepositoryResponseView<'msg> {
  type Message = PurgeRepositoryResponse;
}

impl ::std::fmt::Debug for PurgeRepositoryResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PurgeRepositoryResponseView<'_> {
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

impl ::std::default::Default for PurgeRepositoryResponseView<'_> {
  fn default() -> PurgeRepositoryResponseView<'static> {
    PurgeRepositoryResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> PurgeRepositoryResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> PurgeRepositoryResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `PurgeRepositoryResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PurgeRepositoryResponseView<'_> {}

// SAFETY:
// - `PurgeRepositoryResponseView` is `Send` because while its alive a `PurgeRepositoryResponseMut` cannot.
// - `PurgeRepositoryResponseView` does not use thread-local data.
unsafe impl Send for PurgeRepositoryResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PurgeRepositoryResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PurgeRepositoryResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for PurgeRepositoryResponseView<'msg> {
  type Proxied = PurgeRepositoryResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PurgeRepositoryResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PurgeRepositoryResponseView<'msg> {
  fn into_view<'shorter>(self) -> PurgeRepositoryResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PurgeRepositoryResponse> for PurgeRepositoryResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PurgeRepositoryResponse {
    let dst = PurgeRepositoryResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PurgeRepositoryResponse> for PurgeRepositoryResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PurgeRepositoryResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for PurgeRepositoryResponse {
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
      let prototype = <PurgeRepositoryResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for PurgeRepositoryResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<PurgeRepositoryResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PurgeRepositoryResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { PurgeRepositoryResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PurgeRepositoryResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PurgeRepositoryResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PurgeRepositoryResponseMut<'msg> {
  type Message = PurgeRepositoryResponse;
}

impl ::std::fmt::Debug for PurgeRepositoryResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PurgeRepositoryResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for PurgeRepositoryResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for PurgeRepositoryResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = PurgeRepositoryResponse>) {
    // SAFETY: self and src are both valid `PurgeRepositoryResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> PurgeRepositoryResponseMut<'msg> {
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

  pub fn to_owned(&self) -> PurgeRepositoryResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `PurgeRepositoryResponseMut` does not perform any shared mutation.
// - `PurgeRepositoryResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PurgeRepositoryResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PurgeRepositoryResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PurgeRepositoryResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for PurgeRepositoryResponseMut<'msg> {
  type Proxied = PurgeRepositoryResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PurgeRepositoryResponse> {
    PurgeRepositoryResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PurgeRepositoryResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PurgeRepositoryResponse>
  where
      'msg: 'shorter {
    PurgeRepositoryResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for PurgeRepositoryResponseMut<'msg> {
  type MutProxied = PurgeRepositoryResponse;
  fn as_mut(&mut self) -> PurgeRepositoryResponseMut<'msg> {
    PurgeRepositoryResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PurgeRepositoryResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PurgeRepositoryResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PurgeRepositoryResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryResponse_new() } } }
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

  pub fn as_view(&self) -> PurgeRepositoryResponseView {
    PurgeRepositoryResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> PurgeRepositoryResponseMut {
    PurgeRepositoryResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl PurgeRepositoryResponse

impl ::std::ops::Drop for PurgeRepositoryResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for PurgeRepositoryResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PurgeRepositoryResponse {
  type Proxied = Self;
  fn as_view(&self) -> PurgeRepositoryResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PurgeRepositoryResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PurgeRepositoryResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_PurgeRepositoryResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> PurgeRepositoryResponseMut<'a> {
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

impl<'a> PurgeRepositoryResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for PurgeRepositoryResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for PurgeRepositoryResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for PurgeRepositoryResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for PurgeRepositoryResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for PurgeRepositoryResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for PurgeRepositoryResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct ReinitializeRequest {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for ReinitializeRequest {}

impl ::std::default::Default for ReinitializeRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ReinitializeRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for ReinitializeRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for ReinitializeRequest {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for ReinitializeRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for ReinitializeRequest {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for ReinitializeRequest {
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
// - `ReinitializeRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ReinitializeRequestMut`.
unsafe impl Sync for ReinitializeRequest {}

// SAFETY:
// - `ReinitializeRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ReinitializeRequest {}

impl ::protobuf::Proxied for ReinitializeRequest {
  type View<'msg> = ReinitializeRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ReinitializeRequest {}

impl ::protobuf::MutProxied for ReinitializeRequest {
  type Mut<'msg> = ReinitializeRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReinitializeRequestView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReinitializeRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReinitializeRequestView<'msg> {
  type Message = ReinitializeRequest;
}

impl ::std::fmt::Debug for ReinitializeRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ReinitializeRequestView<'_> {
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

impl ::std::default::Default for ReinitializeRequestView<'_> {
  fn default() -> ReinitializeRequestView<'static> {
    ReinitializeRequestView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_ReinitializeRequest_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> ReinitializeRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> ReinitializeRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_id: optional int64
  pub fn config_id(self) -> i64 {
    unsafe { proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `ReinitializeRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReinitializeRequestView<'_> {}

// SAFETY:
// - `ReinitializeRequestView` is `Send` because while its alive a `ReinitializeRequestMut` cannot.
// - `ReinitializeRequestView` does not use thread-local data.
unsafe impl Send for ReinitializeRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ReinitializeRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ReinitializeRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for ReinitializeRequestView<'msg> {
  type Proxied = ReinitializeRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ReinitializeRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReinitializeRequestView<'msg> {
  fn into_view<'shorter>(self) -> ReinitializeRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ReinitializeRequest> for ReinitializeRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReinitializeRequest {
    let dst = ReinitializeRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ReinitializeRequest> for ReinitializeRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReinitializeRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ReinitializeRequest {
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
      let prototype = <ReinitializeRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ReinitializeRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ReinitializeRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ReinitializeRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ReinitializeRequestView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReinitializeRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReinitializeRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReinitializeRequestMut<'msg> {
  type Message = ReinitializeRequest;
}

impl ::std::fmt::Debug for ReinitializeRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ReinitializeRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for ReinitializeRequestMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for ReinitializeRequestMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = ReinitializeRequest>) {
    // SAFETY: self and src are both valid `ReinitializeRequest`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> ReinitializeRequestMut<'msg> {
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

  pub fn to_owned(&self) -> ReinitializeRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // config_id: optional int64
  pub fn config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_get(self.raw_msg()) }
  }
  pub fn set_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `ReinitializeRequestMut` does not perform any shared mutation.
// - `ReinitializeRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ReinitializeRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ReinitializeRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ReinitializeRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for ReinitializeRequestMut<'msg> {
  type Proxied = ReinitializeRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ReinitializeRequest> {
    ReinitializeRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReinitializeRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ReinitializeRequest>
  where
      'msg: 'shorter {
    ReinitializeRequestView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for ReinitializeRequestMut<'msg> {
  type MutProxied = ReinitializeRequest;
  fn as_mut(&mut self) -> ReinitializeRequestMut<'msg> {
    ReinitializeRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReinitializeRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ReinitializeRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ReinitializeRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_ReinitializeRequest_new() } } }
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

  pub fn as_view(&self) -> ReinitializeRequestView {
    ReinitializeRequestView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ReinitializeRequestMut {
    ReinitializeRequestMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // config_id: optional int64
  pub fn config_id(&self) -> i64 {
    unsafe { proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_get(self.raw_msg()) }
  }
  pub fn set_config_id(&mut self, val: i64) {
    unsafe { proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_set(self.raw_msg(), val) }
  }

}  // impl ReinitializeRequest

impl ::std::ops::Drop for ReinitializeRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ReinitializeRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ReinitializeRequest {
  type Proxied = Self;
  fn as_view(&self) -> ReinitializeRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ReinitializeRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReinitializeRequestMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_ReinitializeRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_ReinitializeRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> i64;
  fn proto2_rust_thunk_szdiagnostic_ReinitializeRequest_config_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: i64);

}

impl<'a> ReinitializeRequestMut<'a> {
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

impl<'a> ReinitializeRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for ReinitializeRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ReinitializeRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for ReinitializeRequestView<'a> {
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

impl ::protobuf::__internal::MatcherEq for ReinitializeRequest {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for ReinitializeRequestMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for ReinitializeRequestView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct ReinitializeResponse {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for ReinitializeResponse {}

impl ::std::default::Default for ReinitializeResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ReinitializeResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for ReinitializeResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::MergeFrom for ReinitializeResponse {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for ReinitializeResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for ReinitializeResponse {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for ReinitializeResponse {
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
// - `ReinitializeResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ReinitializeResponseMut`.
unsafe impl Sync for ReinitializeResponse {}

// SAFETY:
// - `ReinitializeResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ReinitializeResponse {}

impl ::protobuf::Proxied for ReinitializeResponse {
  type View<'msg> = ReinitializeResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ReinitializeResponse {}

impl ::protobuf::MutProxied for ReinitializeResponse {
  type Mut<'msg> = ReinitializeResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReinitializeResponseView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReinitializeResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReinitializeResponseView<'msg> {
  type Message = ReinitializeResponse;
}

impl ::std::fmt::Debug for ReinitializeResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ReinitializeResponseView<'_> {
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

impl ::std::default::Default for ReinitializeResponseView<'_> {
  fn default() -> ReinitializeResponseView<'static> {
    ReinitializeResponseView::new(::protobuf::__internal::Private, unsafe { proto2_rust_thunk_Message_szdiagnostic_ReinitializeResponse_default_instance() })
  }
}

#[allow(dead_code)]
impl<'msg> ReinitializeResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> ReinitializeResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ReinitializeResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReinitializeResponseView<'_> {}

// SAFETY:
// - `ReinitializeResponseView` is `Send` because while its alive a `ReinitializeResponseMut` cannot.
// - `ReinitializeResponseView` does not use thread-local data.
unsafe impl Send for ReinitializeResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ReinitializeResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ReinitializeResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for ReinitializeResponseView<'msg> {
  type Proxied = ReinitializeResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ReinitializeResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReinitializeResponseView<'msg> {
  fn into_view<'shorter>(self) -> ReinitializeResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ReinitializeResponse> for ReinitializeResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReinitializeResponse {
    let dst = ReinitializeResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.msg, self.msg) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ReinitializeResponse> for ReinitializeResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReinitializeResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ReinitializeResponse {
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
      let prototype = <ReinitializeResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ReinitializeResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ReinitializeResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ReinitializeResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ReinitializeResponseView::new(::protobuf::__internal::Private, value.val.m) }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReinitializeResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReinitializeResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReinitializeResponseMut<'msg> {
  type Message = ReinitializeResponse;
}

impl ::std::fmt::Debug for ReinitializeResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ReinitializeResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for ReinitializeResponseMut<'_> {
  fn clear(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_clear(self.raw_msg()) }
  }
}

impl ::protobuf::MergeFrom for ReinitializeResponseMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = ReinitializeResponse>) {
    // SAFETY: self and src are both valid `ReinitializeResponse`s.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_merge_from(self.raw_msg(), src.as_view().raw_msg());
    }
  }
}

#[allow(dead_code)]
impl<'msg> ReinitializeResponseMut<'msg> {
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

  pub fn to_owned(&self) -> ReinitializeResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


}

// SAFETY:
// - `ReinitializeResponseMut` does not perform any shared mutation.
// - `ReinitializeResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ReinitializeResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ReinitializeResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ReinitializeResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for ReinitializeResponseMut<'msg> {
  type Proxied = ReinitializeResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ReinitializeResponse> {
    ReinitializeResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReinitializeResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ReinitializeResponse>
  where
      'msg: 'shorter {
    ReinitializeResponseView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for ReinitializeResponseMut<'msg> {
  type MutProxied = ReinitializeResponse;
  fn as_mut(&mut self) -> ReinitializeResponseMut<'msg> {
    ReinitializeResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReinitializeResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ReinitializeResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ReinitializeResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: unsafe { proto2_rust_thunk_Message_szdiagnostic_ReinitializeResponse_new() } } }
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

  pub fn as_view(&self) -> ReinitializeResponseView {
    ReinitializeResponseView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ReinitializeResponseMut {
    ReinitializeResponseMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

}  // impl ReinitializeResponse

impl ::std::ops::Drop for ReinitializeResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ReinitializeResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ReinitializeResponse {
  type Proxied = Self;
  fn as_view(&self) -> ReinitializeResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ReinitializeResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReinitializeResponseMut {
    self.as_mut()
  }
}


extern "C" {
  fn proto2_rust_thunk_Message_szdiagnostic_ReinitializeResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_szdiagnostic_ReinitializeResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
}

impl<'a> ReinitializeResponseMut<'a> {
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

impl<'a> ReinitializeResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for ReinitializeResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    Self { inner: ::protobuf::__internal::runtime::MessageInner { msg: ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap() } }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ReinitializeResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for ReinitializeResponseView<'a> {
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

impl ::protobuf::__internal::MatcherEq for ReinitializeResponse {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for ReinitializeResponseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for ReinitializeResponseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::raw_message_equals(self.msg, o.msg)
    }
  }
}

