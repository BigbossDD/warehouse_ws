// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from custom_interfaces:msg/RobotStat.idl
// generated code does not contain a copyright notice
#include "custom_interfaces/msg/detail/robot_stat__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `status`
#include "rosidl_runtime_c/string_functions.h"

bool
custom_interfaces__msg__RobotStat__init(custom_interfaces__msg__RobotStat * msg)
{
  if (!msg) {
    return false;
  }
  // battery_level
  // status
  if (!rosidl_runtime_c__String__init(&msg->status)) {
    custom_interfaces__msg__RobotStat__fini(msg);
    return false;
  }
  // is_charging
  return true;
}

void
custom_interfaces__msg__RobotStat__fini(custom_interfaces__msg__RobotStat * msg)
{
  if (!msg) {
    return;
  }
  // battery_level
  // status
  rosidl_runtime_c__String__fini(&msg->status);
  // is_charging
}

bool
custom_interfaces__msg__RobotStat__are_equal(const custom_interfaces__msg__RobotStat * lhs, const custom_interfaces__msg__RobotStat * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // battery_level
  if (lhs->battery_level != rhs->battery_level) {
    return false;
  }
  // status
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->status), &(rhs->status)))
  {
    return false;
  }
  // is_charging
  if (lhs->is_charging != rhs->is_charging) {
    return false;
  }
  return true;
}

bool
custom_interfaces__msg__RobotStat__copy(
  const custom_interfaces__msg__RobotStat * input,
  custom_interfaces__msg__RobotStat * output)
{
  if (!input || !output) {
    return false;
  }
  // battery_level
  output->battery_level = input->battery_level;
  // status
  if (!rosidl_runtime_c__String__copy(
      &(input->status), &(output->status)))
  {
    return false;
  }
  // is_charging
  output->is_charging = input->is_charging;
  return true;
}

custom_interfaces__msg__RobotStat *
custom_interfaces__msg__RobotStat__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  custom_interfaces__msg__RobotStat * msg = (custom_interfaces__msg__RobotStat *)allocator.allocate(sizeof(custom_interfaces__msg__RobotStat), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(custom_interfaces__msg__RobotStat));
  bool success = custom_interfaces__msg__RobotStat__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
custom_interfaces__msg__RobotStat__destroy(custom_interfaces__msg__RobotStat * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    custom_interfaces__msg__RobotStat__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
custom_interfaces__msg__RobotStat__Sequence__init(custom_interfaces__msg__RobotStat__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  custom_interfaces__msg__RobotStat * data = NULL;

  if (size) {
    data = (custom_interfaces__msg__RobotStat *)allocator.zero_allocate(size, sizeof(custom_interfaces__msg__RobotStat), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = custom_interfaces__msg__RobotStat__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        custom_interfaces__msg__RobotStat__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
custom_interfaces__msg__RobotStat__Sequence__fini(custom_interfaces__msg__RobotStat__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      custom_interfaces__msg__RobotStat__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

custom_interfaces__msg__RobotStat__Sequence *
custom_interfaces__msg__RobotStat__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  custom_interfaces__msg__RobotStat__Sequence * array = (custom_interfaces__msg__RobotStat__Sequence *)allocator.allocate(sizeof(custom_interfaces__msg__RobotStat__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = custom_interfaces__msg__RobotStat__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
custom_interfaces__msg__RobotStat__Sequence__destroy(custom_interfaces__msg__RobotStat__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    custom_interfaces__msg__RobotStat__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
custom_interfaces__msg__RobotStat__Sequence__are_equal(const custom_interfaces__msg__RobotStat__Sequence * lhs, const custom_interfaces__msg__RobotStat__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!custom_interfaces__msg__RobotStat__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
custom_interfaces__msg__RobotStat__Sequence__copy(
  const custom_interfaces__msg__RobotStat__Sequence * input,
  custom_interfaces__msg__RobotStat__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(custom_interfaces__msg__RobotStat);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    custom_interfaces__msg__RobotStat * data =
      (custom_interfaces__msg__RobotStat *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!custom_interfaces__msg__RobotStat__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          custom_interfaces__msg__RobotStat__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!custom_interfaces__msg__RobotStat__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
