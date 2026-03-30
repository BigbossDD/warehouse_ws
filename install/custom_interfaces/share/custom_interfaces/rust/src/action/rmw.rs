
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_Goal() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__action__DeliverPackages_Goal__init(msg: *mut DeliverPackages_Goal) -> bool;
    fn custom_interfaces__action__DeliverPackages_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Goal>, size: usize) -> bool;
    fn custom_interfaces__action__DeliverPackages_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Goal>);
    fn custom_interfaces__action__DeliverPackages_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeliverPackages_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Goal>) -> bool;
}

// Corresponds to custom_interfaces__action__DeliverPackages_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeliverPackages_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub destination: rosidl_runtime_rs::String,

}



impl Default for DeliverPackages_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__action__DeliverPackages_Goal__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__action__DeliverPackages_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeliverPackages_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeliverPackages_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeliverPackages_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/action/DeliverPackages_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_Goal() }
  }
}


#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_Result() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__action__DeliverPackages_Result__init(msg: *mut DeliverPackages_Result) -> bool;
    fn custom_interfaces__action__DeliverPackages_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Result>, size: usize) -> bool;
    fn custom_interfaces__action__DeliverPackages_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Result>);
    fn custom_interfaces__action__DeliverPackages_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeliverPackages_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Result>) -> bool;
}

// Corresponds to custom_interfaces__action__DeliverPackages_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeliverPackages_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for DeliverPackages_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__action__DeliverPackages_Result__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__action__DeliverPackages_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeliverPackages_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeliverPackages_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeliverPackages_Result where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/action/DeliverPackages_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_Result() }
  }
}


#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__action__DeliverPackages_Feedback__init(msg: *mut DeliverPackages_Feedback) -> bool;
    fn custom_interfaces__action__DeliverPackages_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Feedback>, size: usize) -> bool;
    fn custom_interfaces__action__DeliverPackages_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Feedback>);
    fn custom_interfaces__action__DeliverPackages_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeliverPackages_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_Feedback>) -> bool;
}

// Corresponds to custom_interfaces__action__DeliverPackages_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeliverPackages_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_remaining: f32,

}



impl Default for DeliverPackages_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__action__DeliverPackages_Feedback__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__action__DeliverPackages_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeliverPackages_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeliverPackages_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeliverPackages_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/action/DeliverPackages_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_Feedback() }
  }
}


#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__action__DeliverPackages_FeedbackMessage__init(msg: *mut DeliverPackages_FeedbackMessage) -> bool;
    fn custom_interfaces__action__DeliverPackages_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_FeedbackMessage>, size: usize) -> bool;
    fn custom_interfaces__action__DeliverPackages_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_FeedbackMessage>);
    fn custom_interfaces__action__DeliverPackages_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeliverPackages_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_FeedbackMessage>) -> bool;
}

// Corresponds to custom_interfaces__action__DeliverPackages_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeliverPackages_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::DeliverPackages_Feedback,

}



impl Default for DeliverPackages_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__action__DeliverPackages_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__action__DeliverPackages_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeliverPackages_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeliverPackages_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeliverPackages_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/action/DeliverPackages_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_FeedbackMessage() }
  }
}




#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__action__DeliverPackages_SendGoal_Request__init(msg: *mut DeliverPackages_SendGoal_Request) -> bool;
    fn custom_interfaces__action__DeliverPackages_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_SendGoal_Request>, size: usize) -> bool;
    fn custom_interfaces__action__DeliverPackages_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_SendGoal_Request>);
    fn custom_interfaces__action__DeliverPackages_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeliverPackages_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_SendGoal_Request>) -> bool;
}

// Corresponds to custom_interfaces__action__DeliverPackages_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeliverPackages_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::DeliverPackages_Goal,

}



impl Default for DeliverPackages_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__action__DeliverPackages_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__action__DeliverPackages_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeliverPackages_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeliverPackages_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeliverPackages_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/action/DeliverPackages_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_SendGoal_Request() }
  }
}


#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__action__DeliverPackages_SendGoal_Response__init(msg: *mut DeliverPackages_SendGoal_Response) -> bool;
    fn custom_interfaces__action__DeliverPackages_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_SendGoal_Response>, size: usize) -> bool;
    fn custom_interfaces__action__DeliverPackages_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_SendGoal_Response>);
    fn custom_interfaces__action__DeliverPackages_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeliverPackages_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_SendGoal_Response>) -> bool;
}

// Corresponds to custom_interfaces__action__DeliverPackages_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeliverPackages_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for DeliverPackages_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__action__DeliverPackages_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__action__DeliverPackages_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeliverPackages_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeliverPackages_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeliverPackages_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/action/DeliverPackages_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_SendGoal_Response() }
  }
}


#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__action__DeliverPackages_GetResult_Request__init(msg: *mut DeliverPackages_GetResult_Request) -> bool;
    fn custom_interfaces__action__DeliverPackages_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_GetResult_Request>, size: usize) -> bool;
    fn custom_interfaces__action__DeliverPackages_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_GetResult_Request>);
    fn custom_interfaces__action__DeliverPackages_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeliverPackages_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_GetResult_Request>) -> bool;
}

// Corresponds to custom_interfaces__action__DeliverPackages_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeliverPackages_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for DeliverPackages_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__action__DeliverPackages_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__action__DeliverPackages_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeliverPackages_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeliverPackages_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeliverPackages_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/action/DeliverPackages_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_GetResult_Request() }
  }
}


#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__action__DeliverPackages_GetResult_Response__init(msg: *mut DeliverPackages_GetResult_Response) -> bool;
    fn custom_interfaces__action__DeliverPackages_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_GetResult_Response>, size: usize) -> bool;
    fn custom_interfaces__action__DeliverPackages_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_GetResult_Response>);
    fn custom_interfaces__action__DeliverPackages_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeliverPackages_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeliverPackages_GetResult_Response>) -> bool;
}

// Corresponds to custom_interfaces__action__DeliverPackages_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeliverPackages_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::DeliverPackages_Result,

}



impl Default for DeliverPackages_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__action__DeliverPackages_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__action__DeliverPackages_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeliverPackages_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__action__DeliverPackages_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeliverPackages_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeliverPackages_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/action/DeliverPackages_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__action__DeliverPackages_GetResult_Response() }
  }
}






#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__custom_interfaces__action__DeliverPackages_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to custom_interfaces__action__DeliverPackages_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct DeliverPackages_SendGoal;

impl rosidl_runtime_rs::Service for DeliverPackages_SendGoal {
    type Request = DeliverPackages_SendGoal_Request;
    type Response = DeliverPackages_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__custom_interfaces__action__DeliverPackages_SendGoal() }
    }
}




#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__custom_interfaces__action__DeliverPackages_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to custom_interfaces__action__DeliverPackages_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct DeliverPackages_GetResult;

impl rosidl_runtime_rs::Service for DeliverPackages_GetResult {
    type Request = DeliverPackages_GetResult_Request;
    type Response = DeliverPackages_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__custom_interfaces__action__DeliverPackages_GetResult() }
    }
}


