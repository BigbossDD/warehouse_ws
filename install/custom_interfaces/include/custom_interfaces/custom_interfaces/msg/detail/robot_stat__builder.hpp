// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from custom_interfaces:msg/RobotStat.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__BUILDER_HPP_
#define CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "custom_interfaces/msg/detail/robot_stat__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace custom_interfaces
{

namespace msg
{

namespace builder
{

class Init_RobotStat_is_charging
{
public:
  explicit Init_RobotStat_is_charging(::custom_interfaces::msg::RobotStat & msg)
  : msg_(msg)
  {}
  ::custom_interfaces::msg::RobotStat is_charging(::custom_interfaces::msg::RobotStat::_is_charging_type arg)
  {
    msg_.is_charging = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_interfaces::msg::RobotStat msg_;
};

class Init_RobotStat_status
{
public:
  explicit Init_RobotStat_status(::custom_interfaces::msg::RobotStat & msg)
  : msg_(msg)
  {}
  Init_RobotStat_is_charging status(::custom_interfaces::msg::RobotStat::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_RobotStat_is_charging(msg_);
  }

private:
  ::custom_interfaces::msg::RobotStat msg_;
};

class Init_RobotStat_battery_level
{
public:
  Init_RobotStat_battery_level()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_RobotStat_status battery_level(::custom_interfaces::msg::RobotStat::_battery_level_type arg)
  {
    msg_.battery_level = std::move(arg);
    return Init_RobotStat_status(msg_);
  }

private:
  ::custom_interfaces::msg::RobotStat msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_interfaces::msg::RobotStat>()
{
  return custom_interfaces::msg::builder::Init_RobotStat_battery_level();
}

}  // namespace custom_interfaces

#endif  // CUSTOM_INTERFACES__MSG__DETAIL__ROBOT_STAT__BUILDER_HPP_
