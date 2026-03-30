// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from custom_interfaces:srv/RequestDeliveyes.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_INTERFACES__SRV__DETAIL__REQUEST_DELIVEYES__STRUCT_H_
#define CUSTOM_INTERFACES__SRV__DETAIL__REQUEST_DELIVEYES__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'destination'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/RequestDeliveyes in the package custom_interfaces.
typedef struct custom_interfaces__srv__RequestDeliveyes_Request
{
  rosidl_runtime_c__String destination;
} custom_interfaces__srv__RequestDeliveyes_Request;

// Struct for a sequence of custom_interfaces__srv__RequestDeliveyes_Request.
typedef struct custom_interfaces__srv__RequestDeliveyes_Request__Sequence
{
  custom_interfaces__srv__RequestDeliveyes_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} custom_interfaces__srv__RequestDeliveyes_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/RequestDeliveyes in the package custom_interfaces.
typedef struct custom_interfaces__srv__RequestDeliveyes_Response
{
  bool success;
  rosidl_runtime_c__String message;
} custom_interfaces__srv__RequestDeliveyes_Response;

// Struct for a sequence of custom_interfaces__srv__RequestDeliveyes_Response.
typedef struct custom_interfaces__srv__RequestDeliveyes_Response__Sequence
{
  custom_interfaces__srv__RequestDeliveyes_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} custom_interfaces__srv__RequestDeliveyes_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CUSTOM_INTERFACES__SRV__DETAIL__REQUEST_DELIVEYES__STRUCT_H_
