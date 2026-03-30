// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from custom_interfaces:msg/RobotStat.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__STRUCT_H_
#define CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'status'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/RobotStat in the package custom_interfaces.
typedef struct custom_interfaces__msg__RobotStat
{
  float battery_level;
  rosidl_runtime_c__String status;
  bool is_charging;
} custom_interfaces__msg__RobotStat;

// Struct for a sequence of custom_interfaces__msg__RobotStat.
typedef struct custom_interfaces__msg__RobotStat__Sequence
{
  custom_interfaces__msg__RobotStat * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} custom_interfaces__msg__RobotStat__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__STRUCT_H_
