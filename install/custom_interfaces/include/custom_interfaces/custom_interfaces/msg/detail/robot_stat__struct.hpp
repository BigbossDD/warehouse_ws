// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from custom_interfaces:msg/RobotStat.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__STRUCT_HPP_
#define CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__custom_interfaces__msg__RobotStat __attribute__((deprecated))
#else
# define DEPRECATED__custom_interfaces__msg__RobotStat __declspec(deprecated)
#endif

namespace custom_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct RobotStat_
{
  using Type = RobotStat_<ContainerAllocator>;

  explicit RobotStat_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->battery_level = 0.0f;
      this->status = "";
      this->is_charging = false;
    }
  }

  explicit RobotStat_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : status(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->battery_level = 0.0f;
      this->status = "";
      this->is_charging = false;
    }
  }

  // field types and members
  using _battery_level_type =
    float;
  _battery_level_type battery_level;
  using _status_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _status_type status;
  using _is_charging_type =
    bool;
  _is_charging_type is_charging;

  // setters for named parameter idiom
  Type & set__battery_level(
    const float & _arg)
  {
    this->battery_level = _arg;
    return *this;
  }
  Type & set__status(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->status = _arg;
    return *this;
  }
  Type & set__is_charging(
    const bool & _arg)
  {
    this->is_charging = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    custom_interfaces::msg::RobotStat_<ContainerAllocator> *;
  using ConstRawPtr =
    const custom_interfaces::msg::RobotStat_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<custom_interfaces::msg::RobotStat_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<custom_interfaces::msg::RobotStat_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      custom_interfaces::msg::RobotStat_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<custom_interfaces::msg::RobotStat_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      custom_interfaces::msg::RobotStat_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<custom_interfaces::msg::RobotStat_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<custom_interfaces::msg::RobotStat_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<custom_interfaces::msg::RobotStat_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__custom_interfaces__msg__RobotStat
    std::shared_ptr<custom_interfaces::msg::RobotStat_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__custom_interfaces__msg__RobotStat
    std::shared_ptr<custom_interfaces::msg::RobotStat_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const RobotStat_ & other) const
  {
    if (this->battery_level != other.battery_level) {
      return false;
    }
    if (this->status != other.status) {
      return false;
    }
    if (this->is_charging != other.is_charging) {
      return false;
    }
    return true;
  }
  bool operator!=(const RobotStat_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct RobotStat_

// alias to use template instance with default allocator
using RobotStat =
  custom_interfaces::msg::RobotStat_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace custom_interfaces

#endif  // CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__STRUCT_HPP_
