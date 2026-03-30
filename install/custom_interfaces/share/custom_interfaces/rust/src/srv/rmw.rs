#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__srv__RequestDeliveyes_Request() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__srv__RequestDeliveyes_Request__init(msg: *mut RequestDeliveyes_Request) -> bool;
    fn custom_interfaces__srv__RequestDeliveyes_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RequestDeliveyes_Request>, size: usize) -> bool;
    fn custom_interfaces__srv__RequestDeliveyes_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RequestDeliveyes_Request>);
    fn custom_interfaces__srv__RequestDeliveyes_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RequestDeliveyes_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RequestDeliveyes_Request>) -> bool;
}

// Corresponds to custom_interfaces__srv__RequestDeliveyes_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestDeliveyes_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub destination: rosidl_runtime_rs::String,

}



impl Default for RequestDeliveyes_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__srv__RequestDeliveyes_Request__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__srv__RequestDeliveyes_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RequestDeliveyes_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__srv__RequestDeliveyes_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__srv__RequestDeliveyes_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__srv__RequestDeliveyes_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RequestDeliveyes_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RequestDeliveyes_Request where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/srv/RequestDeliveyes_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__srv__RequestDeliveyes_Request() }
  }
}


#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__srv__RequestDeliveyes_Response() -> *const std::ffi::c_void;
}

#[link(name = "custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn custom_interfaces__srv__RequestDeliveyes_Response__init(msg: *mut RequestDeliveyes_Response) -> bool;
    fn custom_interfaces__srv__RequestDeliveyes_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RequestDeliveyes_Response>, size: usize) -> bool;
    fn custom_interfaces__srv__RequestDeliveyes_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RequestDeliveyes_Response>);
    fn custom_interfaces__srv__RequestDeliveyes_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RequestDeliveyes_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RequestDeliveyes_Response>) -> bool;
}

// Corresponds to custom_interfaces__srv__RequestDeliveyes_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestDeliveyes_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for RequestDeliveyes_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_interfaces__srv__RequestDeliveyes_Response__init(&mut msg as *mut _) {
        panic!("Call to custom_interfaces__srv__RequestDeliveyes_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RequestDeliveyes_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__srv__RequestDeliveyes_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__srv__RequestDeliveyes_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_interfaces__srv__RequestDeliveyes_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RequestDeliveyes_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RequestDeliveyes_Response where Self: Sized {
  const TYPE_NAME: &'static str = "custom_interfaces/srv/RequestDeliveyes_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_interfaces__srv__RequestDeliveyes_Response() }
  }
}






#[link(name = "custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__custom_interfaces__srv__RequestDeliveyes() -> *const std::ffi::c_void;
}

// Corresponds to custom_interfaces__srv__RequestDeliveyes
#[allow(missing_docs, non_camel_case_types)]
pub struct RequestDeliveyes;

impl rosidl_runtime_rs::Service for RequestDeliveyes {
    type Request = RequestDeliveyes_Request;
    type Response = RequestDeliveyes_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__custom_interfaces__srv__RequestDeliveyes() }
    }
}


