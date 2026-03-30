#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to custom_interfaces__srv__RequestDeliveyes_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestDeliveyes_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub destination: std::string::String,

}



impl Default for RequestDeliveyes_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RequestDeliveyes_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RequestDeliveyes_Request {
  type RmwMsg = super::srv::rmw::RequestDeliveyes_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        destination: msg.destination.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        destination: msg.destination.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      destination: msg.destination.to_string(),
    }
  }
}


// Corresponds to custom_interfaces__srv__RequestDeliveyes_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestDeliveyes_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for RequestDeliveyes_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RequestDeliveyes_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RequestDeliveyes_Response {
  type RmwMsg = super::srv::rmw::RequestDeliveyes_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
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


